use std::process::exit;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listeners = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _addr) = listeners.accept().await.unwrap();
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&line.as_bytes()).await.unwrap();
        line.clear();
    }
}
