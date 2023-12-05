use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncWriteExt, ErrorKind};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::game_commands::GAME_COMMANDS;

/*use crate::motor::contador::Contador;
use crate::motor::jugador::Jugador;
use crate::motor::mazo::Mazo;
use crate::motor::mesa::Mesa;*/

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
    pub fn new(name: String, chairs: Chairs) -> Self {
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
) {
    let mut players = Vec::with_capacity(chairs as usize);
    let mut unnamed_players = Vec::with_capacity(chairs as usize);
    let mut observers = Vec::new();
    let (name_tx, mut name_rx) = channel(16);
    let (tx, mut rx) = channel(16);
    loop {
        tokio::select! {

            Some((mut stream, as_player)) = join_requests.recv() => {
                if as_player {
                    stream.write_all(b"Enter your name\n").await.unwrap();
                    let (reader, writer) = stream.into_split();
                    let tx = name_tx.clone();
                    let mut count = player_count.lock().unwrap();
                    let id = *count;
                    let (name_feedback_tx, name_feedback_rx) = channel(16);
                    let join_handle = tokio::spawn(async move { naming_stream(id, reader, tx, name_feedback_rx).await });
                    unnamed_players.push((id, writer, name_feedback_tx, join_handle));
                    *count = players.len() + unnamed_players.len();
                } else {
                    observers.push(stream);
                    *observer_count.lock().unwrap() = observers.len();
                }
            },

            Some((id, player_name)) = name_rx.recv() => {
                if let Some(index) = unnamed_players.iter().position(|(x, _, _, _)| *x == id){

                    if players
                        .iter()
                        .find(|(name, _)| name == &player_name)
                        .is_none()
                    {
                        let (_, stream, naming_tx, join_handle) = unnamed_players.remove(id);
                        naming_tx.send(true).await.unwrap();
                        players.push((player_name.clone(), stream));
                        let reader = join_handle.await.unwrap();
                        let tx = tx.clone();
                        let name_clone = player_name.clone();
                        tokio::spawn(async move { game_stream(name_clone, reader, tx).await });
                        if players.len() == chairs {
                            println!("Table {name}: Begining game");
                        }
                    } else {
                        let (_, stream,  tx, _) = unnamed_players.get_mut(index).unwrap();
                        stream.write_all(b"Name allready taken\n").await.unwrap();
                        tx.send(false).await.unwrap();
                    }
                }
            },

            Some((name, commands)) = rx.recv() => {
                let command_name = &commands[0];
                println!("command: {commands:?}");
                let (route, answer) = if let Some(command) = GAME_COMMANDS
                    .iter()
                    .find(|command| command.name() == command_name)
                {
                    command.run(name, commands)
                } else {
                    (Routing::Single(name), format!("ERROR: command {command_name} is not found\n"))
                };
                for (player_name, socket) in &mut players {
                    match &route {
                        Routing::Single(routing_name) => {
                            if player_name == routing_name {
                                socket.write_all(answer.as_bytes()).await.unwrap();
                                break;
                            }
                        },
                        Routing::BroadCast => {
                            socket.write_all(answer.as_bytes()).await.unwrap();
                        }
                    }
                }
                for socket in &mut observers {
                    socket.write_all(answer.as_bytes()).await.unwrap();
                }
            },
        };
    }
    /*
    let mut mazo = Mazo::new();
    let mut mesa = Mesa::new(chairs);
    let mut contador = Contador::new(false);
    let mut jugadores: Vec<Jugador> = Vec::new();
    for i in 0..chairs {
        jugadores.push(Jugador::new(i));
    }
    loop {
        mazo.mezclar();
        mazo.repartir(&mut jugadores);
        println!("{}", mesa);
        let mut ganador = None;
        for _ in 0..3 {
            for i in mesa.indices_de_turnos() {
                jugadores[i].turno(&mut mesa);
                println!("{}", mesa);
            }
            mesa.final_de_ronda();
            ganador = mesa.ganador();
            if ganador.is_some() {
                break;
            }
        }
        if contador.sumar(ganador) {
            break;
        }
        println!("{}", contador);
        mesa.siguiente();
    }
    println!("{}", contador);
    println!();
    println!("Ganador {}", contador.ganador());*/
}

async fn game_stream(name: String, reader: OwnedReadHalf, tx: Sender<(String, Vec<String>)>) {
    loop {
        let mut buffer = [0; 1024];
        reader.readable().await.unwrap();
        let n_bytes = match reader.try_read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    continue;
                }
                0
            }
        };
        let commands = String::from_utf8_lossy(&buffer[0..n_bytes]);
        let commands = commands
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if commands.len() == 0 {
            continue;
        }
        tx.send((name.clone(), commands)).await.unwrap();
    }
}

async fn naming_stream(
    id: usize,
    reader: OwnedReadHalf,
    tx: Sender<(usize, String)>,
    mut rx: Receiver<bool>,
) -> OwnedReadHalf {
    loop {
        let mut buffer = [0; 1024];
        reader.readable().await.unwrap();
        let n_bytes = match reader.try_read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    continue;
                }
                0
            }
        };
        let name = String::from_utf8_lossy(&buffer[0..n_bytes])
            .to_string()
            .split_whitespace()
            .collect();
        tx.send((id, name)).await.unwrap();
        if rx.recv().await.unwrap() {
            break;
        }
    }
    reader
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
