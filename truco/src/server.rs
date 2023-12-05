use std::sync::{Arc, Mutex};

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

mod tables;
use tables::*;

mod commands;
use commands::COMMANDS;

mod motor;

mod game_commands;

mod streams;

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
        let command_name = commands[0];
        println!("command: {commands:?}");
        let answer = if let Some(command) = COMMANDS
            .iter()
            .find(|command| command.name() == command_name)
        {
            let (sock_op, string) = command.run(commands, socket, tables.clone());
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
