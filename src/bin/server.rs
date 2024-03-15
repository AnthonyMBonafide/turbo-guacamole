use tguac::cache::Cache;
use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:8727").await.unwrap();

    let (stream, _) = listener.accept().await.unwrap();
    tokio::spawn(async move {});
}

async fn process<T>(stream: TcpStream, db: Cache<T>) {}
