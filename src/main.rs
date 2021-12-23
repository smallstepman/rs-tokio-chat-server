use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listeners = TcpListener::bind("localhost:8080").await.unwrap();
    let (socket, addr) = listeners.accept().await.unwrap();
    let mut buffer = [0u8; 1024];
    socket.read(&mut buffer).await.unwrap();
    println!("Hello, world!");
}
