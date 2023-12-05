use std::fmt::Display;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

use crate::tables::*;

pub type CommandRet = (Option<TcpStream>, String);
pub struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(Vec<&str>, TcpStream, Arc<Mutex<Tables>>) -> CommandRet,
}

impl Command {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run(
        &self,
        commands: Vec<&str>,
        socket: TcpStream,
        tables: Arc<Mutex<Tables>>,
    ) -> CommandRet {
        (self.run)(commands, socket, tables)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6} - {}", self.name, self.description)
    }
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description:
            "help {command0} {command1} .. gives the description of a command, otherwise describes all commands",
        run: help,
    },
    Command {
        name: "list",
        description: "list all available tables",
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
    if let Some(_) = tables.lock().unwrap().iter().find(|t| t.name() == &name) {
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
        if tables[i].name() == &name {
            if as_player && tables[i].players_num() >= tables[i].chairs() as usize {
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

