use tguac::cache::{self, Cache};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8727").await.unwrap();
    let database: Cache<String> = Cache::new();
    let (stream, _) = listener.accept().await.unwrap();
    tokio::spawn(async move { process(stream, database).await });
}

async fn process<T>(stream: TcpStream, db: Cache<T>) {
    let (stream_writer, stream_reader) = io::split(stream);
}
