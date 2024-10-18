use std::{
    collections::HashMap,
    io::Error,
    sync::{Arc, Mutex},
};

use tokio::net::{TcpListener, TcpStream};

use crate::{connection, frame::Frame};

type Db = Arc<Mutex<HashMap<String, String>>>;

pub async fn run(port:u16) {
    let db = Arc::new(Mutex::new(HashMap::new()));

    let bind_ip = format!("127.0.0.1:{port}");
    let tcp_listener = TcpListener::bind(&bind_ip).await.unwrap();
    println!("server start at:{}", bind_ip);
    loop {
        let accept_tcp = tcp_listener.accept().await;
        let (socket, addr) = accept_tcp.unwrap();
        println!("get request from: {}", addr);
        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            let _ = process_socket(socket, db_clone).await;
        });
    }
}

async fn process_socket(socket: TcpStream, db: Db) -> Result<(), Error> {
    let mut connection = connection::Connection::new(socket);
    let frame = connection.read_frame().await.expect("get frame error");
    match frame {
        Frame::Simple(s) => {
            println!("get simple frame:{}", s);
            connection.write_frame(&Frame::Null).await;
        }
        Frame::Error(e) => {
            println!("get error frame:{e}");
        }
        Frame::Integer(i) => {
            println!("get integer frame:{}", i);
            connection.write_frame(&Frame::Null).await;
        }
        Frame::Null => {
            println!("get null frame");
        },
    }
    println!("this connection process done");
    Ok(())
}
