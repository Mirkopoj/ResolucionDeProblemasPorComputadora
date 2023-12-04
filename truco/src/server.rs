use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Parser)]
#[command(author, version, about, long_about = "Sever for hosting a Truco game")]
struct Cli {
    #[arg(short, long, default_value_t = 1234)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let listener = TcpListener::bind("0.0.0.0:".to_string() + &cli.port.to_string())
        .await
        .unwrap();

    println!("Listening on port: {}", cli.port);

    let tables = Arc::new(Mutex::new(Tables::new()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Conection from: {:?}", addr);
        let tables = tables.clone();
        tokio::spawn(async move {
            lobby(socket, tables).await;
        });
    }
}

async fn lobby(mut socket: TcpStream, tables: Arc<Mutex<Tables>>) {
    loop {
        let mut buffer = [0; 1024];
        let n_bytes = socket.read(&mut buffer).await.unwrap();
        let commands = String::from_utf8_lossy(&buffer[0..n_bytes]);
        let commands = commands.to_string();
        let commands = commands.split_whitespace().collect::<Vec<&str>>();
        if commands.len() == 0 {
            continue;
        }
        let command_name = &commands[0];
        println!("command: {commands:?}");
        let answer = if let Some(command) = COMMANDS
            .iter()
            .find(|command| &command.name == command_name)
        {
            let (sock_op, string) = (command.run)(commands, socket, tables.clone());
            if let Some(sock) = sock_op {
                socket = sock;
            } else {
                break;
            }
            string
        } else {
            format!("ERROR: command {command_name} is not found\n")
        };
        socket.write_all(answer.as_bytes()).await.unwrap();
    }
}

type CommandRet = (Option<TcpStream>, String);
struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(Vec<&str>, socket: TcpStream, tables: Arc<Mutex<Tables>>) -> CommandRet,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6} - {}", self.name, self.description)
    }
}

const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description:
            "help {command0} {command1} .. gives the description of a command, otherwise describes all commands",
        run: help,
    },
    Command {
        name: "list",
        description: "list all available commands",
        run: list,
    },
    Command {
        name: "new",
        description: "new <name> <chairs[2, 4, 6]> {o..}, creates and joins a table with the given name and number of chairs. Optionaly specify to join as observer",
        run: new_table,
    },
    Command {
        name: "join",
        description: "join <name> {o..}, joins a table with the given name. Optionaly specify to join as observer",
        run: join_table,
    },
];

fn usage() -> String {
    let mut ret = String::new();
    for command in COMMANDS {
        ret.push_str(&format!("{}\n", command));
    }
    ret
}

fn help(commands: Vec<&str>, s: TcpStream, _: Arc<Mutex<Tables>>) -> CommandRet {
    if commands.len() < 2 {
        return (Some(s), usage());
    }
    let mut ret = String::new();
    for command_name in commands[1..].iter() {
        if let Some(command) = COMMANDS
            .iter()
            .find(|command| &command.name == command_name)
        {
            ret.push_str(&format!("{}\n", command));
            continue;
        }
        ret.push_str(&format!("ERROR: command {command_name} is not found\n"));
    }
    (Some(s), ret)
}

fn list(_: Vec<&str>, s: TcpStream, tables: Arc<Mutex<Tables>>) -> CommandRet {
    (Some(s), format!("{:#}", tables.lock().unwrap()))
}

fn new_table(args: Vec<&str>, socket: TcpStream, tables: Arc<Mutex<Tables>>) -> CommandRet {
    if args.len() < 3 {
        return (Some(socket), "ERROR: insuficient arguments\n".to_string());
    }
    let mut chairs = None;
    if let Ok(chair_num) = args[2].parse() {
        chairs = Chairs::new(chair_num);
    }
    if chairs.is_none() {
        return (
            Some(socket),
            "ERROR: Not a valid chair number\n".to_string(),
        );
    }
    let name = args[1].to_string();
    let mut as_player = true;
    if args.len() >= 4 {
        if args[3].to_lowercase().starts_with("o") {
            as_player = false;
        }
    }
    if let Some(_) = tables.lock().unwrap().iter().find(|t| t.name == name) {
        return (
            Some(socket),
            "ERROR: table name allready taken\n".to_string(),
        );
    }
    let mut tables = tables.lock().unwrap();
    tables.push(Table::new(name, chairs.unwrap()));
    let last = tables.len() - 1;
    tables[last].join(socket, as_player);
    (None, "SUCCESS\n".to_string())
}

fn join_table(args: Vec<&str>, socket: TcpStream, tables: Arc<Mutex<Tables>>) -> CommandRet {
    if args.len() < 2 {
        return (Some(socket), "ERROR: insuficient arguments\n".to_string());
    }
    let name = args[1].to_string();
    let mut as_player = true;
    if args.len() >= 3 {
        if args[2].to_lowercase().starts_with("o") {
            as_player = false;
        }
    }
    let mut sock_op = Some(socket);
    let mut success = false;
    let mut tables = tables.lock().unwrap();
    for i in 0..tables.len() {
        if tables[i].name == name {
            if as_player && tables[i].players_num() >= tables[i].chairs as usize {
                return (sock_op, "ERROR: table allready full\n".to_string());
            }
            tables[i].join(sock_op.take().unwrap(), as_player);
            success = true;
            break;
        }
    }
    let msg = if success {
        "SUCCESS\n"
    } else {
        "ERROR: no such table\n"
    };
    (sock_op, msg.to_string())
}

#[derive(Clone, Copy, Debug)]
enum Chairs {
    Two = 2,
    Four = 4,
    Six = 6,
}

impl Chairs {
    fn new(number_of_chairs: u8) -> Option<Self> {
        match number_of_chairs {
            2 => Some(Chairs::Two),
            4 => Some(Chairs::Four),
            6 => Some(Chairs::Six),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Table {
    name: String,
    players: Arc<Mutex<usize>>,
    chairs: Chairs,
    observers: Arc<Mutex<usize>>,
    tx: Sender<(TcpStream, bool)>,
}

impl Table {
    fn new(name: String, chairs: Chairs) -> Self {
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

    fn join(&mut self, stream: TcpStream, as_player: bool) {
        let tx = self.tx.clone();
        tokio::spawn(async move { tx.send((stream, as_player)).await });
    }

    fn players_num(&self) -> usize {
        *self.players.lock().unwrap()
    }

    fn observers_num(&self) -> usize {
        *self.observers.lock().unwrap()
    }
}

async fn table_thread(
    mut rx: Receiver<(TcpStream, bool)>,
    player_count: Arc<Mutex<usize>>,
    observer_count: Arc<Mutex<usize>>,
    chairs: usize,
    name: String,
) {
    let mut players = Vec::new();
    let mut observers = Vec::new();
    loop {
        let (stream, as_player) = rx.recv().await.unwrap();
        if as_player {
            players.push(stream);
            *player_count.lock().unwrap() = players.len();
            if players.len() == chairs {
                println!("Table {name}: Begining game");
            }
        } else {
            observers.push(stream);
            *observer_count.lock().unwrap() = observers.len();
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

struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    fn new() -> Self {
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
