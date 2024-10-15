use std::io::{self, Read, Write};

use connection::{Connection, Frame};
use tokio::net::TcpStream;

mod connection;
#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the Redis server
    let stream = TcpStream::connect("127.0.0.1:9000").await?;
    println!("Connected to the server!");

    let mut conn = Connection::new(stream);
    conn.write_frame(Frame::Simple("hello".to_string()));
    // Send a PING command to the Redis server

    let resp_frame= conn.read_frame().await.unwrap();
    println!("get response frame:{:?}",resp_frame);
    Ok(())
}
