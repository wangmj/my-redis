use std::{
    collections::HashMap,
    io::Error,
    sync::{Arc, Mutex},
};

use connection::Frame;
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, String>>>;
mod connection;
#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));

    let bind_ip = "127.0.0.1:9000";
    let tcp_listener = TcpListener::bind(bind_ip).await.unwrap();
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
        connection::Frame::Simple(s) => {
            println!("get simple frame:{}", s);
            connection.write_frame(Frame::Null);
        }
        connection::Frame::Bulk(command) => match command {
            connection::Command::Set { key, val } => {
                let mut locked_db = db.lock().unwrap();
                locked_db.insert(key, val);
            }
            connection::Command::Get { key } => {
                let locked_db = db.lock().unwrap();
                let val = locked_db.get(&key);
                match val {
                    Some(v) => {
                        connection.write_frame(Frame::Simple(v.to_string()));
                    }
                    None => {
                        connection.write_frame(Frame::Null);
                    }
                }
            }
        },
        connection::Frame::Error(e) => {
            println!("get error frame:{e}");
        }
        connection::Frame::Integer(i) => {
            println!("get integer frame:{}", i);
            connection.write_frame(Frame::Null);
        }
        connection::Frame::Null => {
            println!("get null frame");
        },
    }
    println!("this connection process done");
    Ok(())
}
