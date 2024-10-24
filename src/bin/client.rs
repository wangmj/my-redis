use my_redis::{connection::Connection, frame::Frame};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let ipadd = "127.0.0.1:9000";
    let tcpsocket = TcpStream::connect(ipadd).await.unwrap();
    let mut socket_conn = Connection::new(tcpsocket);
    let frame = Frame::Simple("hello this is a large string......".to_string());
    socket_conn.write_frame(&frame).await;
    let resp_frame = socket_conn.read_frame().await;
    println!("get response frame:{:?}", resp_frame);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let frame = Frame::Simple("hello second! this is a large string......".to_string());
    socket_conn.write_frame(&frame).await;
    let resp_frame = socket_conn.read_frame().await;
    println!("get response frame:{:?}", resp_frame);
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
