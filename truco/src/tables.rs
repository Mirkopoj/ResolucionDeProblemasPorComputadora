use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::game_commands::{GAME_COMMANDS, STATIC_COMMANDS};
use crate::game_logic::GameLogic;
use crate::{lobby, streams::*};

#[derive(Clone, Copy, Debug)]
pub enum Chairs {
    Two = 2,
    Four = 4,
    Six = 6,
}

impl Chairs {
    pub fn new(number_of_chairs: u8) -> Option<Self> {
        match number_of_chairs {
            2 => Some(Chairs::Two),
            4 => Some(Chairs::Four),
            6 => Some(Chairs::Six),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Table {
    name: String,
    players: Arc<Mutex<usize>>,
    chairs: Chairs,
    observers: Arc<Mutex<usize>>,
    tx: Sender<(TcpStream, bool)>,
}

impl Table {
    pub fn new(name: String, chairs: Chairs, tables: Arc<Mutex<Tables>>) -> Self {
        let (tx, rx) = channel(16);
        let players = Arc::new(Mutex::new(0));
        let players_clone = players.clone();
        let observers = Arc::new(Mutex::new(0));
        let observers_clone = observers.clone();
        let name_clone = name.clone();
        tokio::spawn(async move {
            table_thread(
                rx,
                players_clone,
                observers_clone,
                chairs as usize,
                name_clone,
                tables,
            )
            .await
        });
        Self {
            name,
            players,
            chairs,
            observers,
            tx,
        }
    }

    pub fn join(&mut self, stream: TcpStream, as_player: bool) {
        let tx = self.tx.clone();
        tokio::spawn(async move { tx.send((stream, as_player)).await });
    }

    pub fn players_num(&self) -> usize {
        *self.players.lock().unwrap()
    }

    pub fn observers_num(&self) -> usize {
        *self.observers.lock().unwrap()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn chairs(&self) -> Chairs {
        self.chairs
    }
}

pub enum Routing {
    Single(String),
    BroadCast,
}

async fn table_thread(
    mut join_requests: Receiver<(TcpStream, bool)>,
    player_count: Arc<Mutex<usize>>,
    observer_count: Arc<Mutex<usize>>,
    chairs: usize,
    name: String,
    tables: Arc<Mutex<Tables>>,
) {
    let mut unnamed_players = Vec::with_capacity(chairs);
    let mut observers = Vec::new();
    let (name_tx, mut name_rx) = channel(16);
    let (tx, mut rx) = channel(16);
    let mut game_logic = GameLogic::new(name.clone(), chairs, false);
    loop {
        if game_logic.game_over() {
            break;
        }
        tokio::select! {

            Some(join_data) = join_requests.recv() => {
                joining_routine(
                    join_data,
                    &name_tx,
                    player_count.clone(),
                    observer_count.clone(),
                    &mut unnamed_players,
                    &game_logic,
                    &mut observers
                ).await;
            },

            Some(naming_data) = name_rx.recv() => {
                naming_routine(
                    naming_data,
                    &mut unnamed_players,
                    &mut game_logic,
                    tx.clone(),
                ).await;
            },

            Some(game_command_data) = rx.recv() => {
                game_command_routine(
                    game_command_data,
                    &mut observers,
                    &mut game_logic,
                ).await;
            },
        };
    }
    for stream in game_logic.streams().into_iter() {
        let table_clone = tables.clone();
        tokio::spawn(async move {
            let (read_half, write_half) = stream.join().await;
            let socket = read_half.reunite(write_half).unwrap();
            tokio::spawn(async move {
                lobby(socket, table_clone).await;
            });
        });
    }
    for socket in observers {
        let table_clone = tables.clone();
        tokio::spawn(async move {
            lobby(socket, table_clone).await;
        });
    }
    let mut tables = tables.lock().unwrap();
    let index = tables.iter().position(|t| t.name() == &name).unwrap();
    tables.remove(index);
}

async fn joining_routine(
    (mut stream, as_player): (TcpStream, bool),
    name_tx: &Sender<(usize, String)>,
    player_count: Arc<Mutex<usize>>,
    observer_count: Arc<Mutex<usize>>,
    unnamed_players: &mut Vec<(
        usize,
        OwnedWriteHalf,
        Sender<bool>,
        JoinHandle<OwnedReadHalf>,
    )>,
    game_logic: &GameLogic,
    observers: &mut Vec<TcpStream>,
) {
    if as_player {
        stream.write_all(b"Enter your name\n").await.unwrap();
        let (reader, writer) = stream.into_split();
        let tx = name_tx.clone();
        let mut count = player_count.lock().unwrap();
        let id = *count;
        let (name_feedback_tx, name_feedback_rx) = channel(16);
        let join_handle =
            tokio::spawn(async move { naming_stream(id, reader, tx, name_feedback_rx).await });
        unnamed_players.push((id, writer, name_feedback_tx, join_handle));
        *count = game_logic.player_count() + unnamed_players.len();
    } else {
        observers.push(stream);
        *observer_count.lock().unwrap() = observers.len();
    }
}

async fn naming_routine(
    (id, player_name): (usize, String),
    unnamed_players: &mut Vec<(
        usize,
        OwnedWriteHalf,
        Sender<bool>,
        JoinHandle<OwnedReadHalf>,
    )>,
    game_logic: &mut GameLogic,
    tx: Sender<(String, Vec<String>)>,
) {
    if let Some(index) = unnamed_players.iter().position(|(x, _, _, _)| *x == id) {
        if game_logic
            .players()
            .iter()
            .find(|p| p.name() == &player_name)
            .is_none()
        {
            let (_, stream, naming_tx, join_handle) = unnamed_players.remove(index);
            naming_tx.send(true).await.unwrap();
            let reader = join_handle.await.unwrap();
            let name_clone = player_name.clone();
            let tx_clone = tx.clone();
            let join_handle =
                tokio::spawn(async move { game_stream(name_clone, reader, tx_clone).await });
            if game_logic.add_player(player_name.clone(), stream, join_handle) {
                tx.send(("  start  ".to_string(), vec!["start".to_string()]))
                    .await
                    .unwrap();
            }
        } else {
            let (_, stream, tx, _) = unnamed_players.get_mut(index).unwrap();
            stream.write_all(b"Name allready taken\n").await.unwrap();
            tx.send(false).await.unwrap();
        }
    }
}

async fn game_command_routine(
    (name, commands): (String, Vec<String>),
    observers: &mut Vec<TcpStream>,
    game_logic: &mut GameLogic,
) {
    let command_name = &commands[0];
    println!("{name}: {commands:?}");
    for (route, answer) in if let Some(command) = GAME_COMMANDS[..STATIC_COMMANDS]
        .iter()
        .chain(game_logic.available_game_commands(&name).into_iter())
        .find(|command| command.name() == command_name)
    {
        command.run(name, commands, game_logic)
    } else {
        vec![(
            Routing::Single(name),
            format!("ERROR: command {command_name} is not found\n"),
        )]
    } {
        for (player_name, socket) in game_logic.players().iter_mut().map(|p| p.server_tuple()) {
            match &route {
                Routing::Single(routing_name) => {
                    if player_name == routing_name {
                        socket.write_all(answer.as_bytes()).await.unwrap();
                        break;
                    }
                }
                Routing::BroadCast => {
                    socket.write_all(answer.as_bytes()).await.unwrap();
                }
            }
        }
        for socket in &mut *observers {
            socket.write_all(answer.as_bytes()).await.unwrap();
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<16} │ {:>7}/{:<6} │ {}",
            self.name,
            self.players_num(),
            self.chairs as u8,
            self.observers_num(),
        )
    }
}

pub struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }
}

impl Deref for Tables {
    type Target = Vec<Table>;
    fn deref(&self) -> &Self::Target {
        &self.tables
    }
}

impl DerefMut for Tables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tables
    }
}

impl Display for Tables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:<16} │ players/chairs │ observers\n{:─<17}┼{:─<16}┼{:─<10}\n",
                "name", "", "", ""
            )?;
            for table in &self.tables {
                write!(f, "{}\n", table)?;
            }
            write!(f, "")
        } else {
            for table in &self.tables {
                write!(
                    f,
                    "{} {} {} {}\n",
                    table.name,
                    table.players_num(),
                    table.chairs as u8,
                    table.observers_num(),
                )?;
            }
            write!(f, "")
        }
    }
}
