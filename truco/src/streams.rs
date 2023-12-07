use tokio::sync::mpsc::{Sender, Receiver};

use tokio::{io::ErrorKind, net::tcp::OwnedReadHalf};

pub async fn game_stream(name: String, reader: OwnedReadHalf, tx: Sender<(String, Vec<String>)>) -> OwnedReadHalf {
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
        if tx.send((name.clone(), commands)).await.is_err() {
            break;
        }
    }
    reader
}

pub async fn naming_stream(
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

