use my_redis::connection::{Connection, Frame};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let ipadd = "127.0.0.1:9000";
    let tcpsocket = TcpStream::connect(ipadd).await.unwrap();
    let mut socket_conn = Connection::new(tcpsocket);
    let frame = Frame::Simple("hello".to_string());
    socket_conn.write_frame(frame);
    let resp_frame = socket_conn.read_frame().await;
    println!("get response frame:{:?}", resp_frame);
}
