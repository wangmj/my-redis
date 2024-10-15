use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("bind tcp listener port failed!");
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let db_clone = db.clone();
        if let Ok((socket, _)) = tcp_listener.accept().await {
            tokio::spawn(async move {
                process_socket(socket, db_clone).await;
            });
        }
    }
}

async fn process_socket(socket: TcpStream, db: Db) {
    use mini_redis::Command;
    // let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    loop {
        let read_frame = connection.read_frame().await.expect("read frame failed!");
        if let Some(frame) = read_frame {
            println!("{:?}", frame);
            let command = Command::from_frame(frame).expect("read frame failed!");

            let response = match command {
                Command::Set(set) => {
                    db.lock()
                        .unwrap()
                        .insert(set.key().to_string(), Bytes::copy_from_slice(set.value()));
                    Frame::Simple("OK".to_string())
                }
                Command::Get(get) => {
                    let key = get.key();
                    let unlocked_db = db.lock().unwrap();
                    let value = unlocked_db.get(key);
                    // let value = db.lock().unwrap().get(key);
                    match value {
                        Some(v) => Frame::Bulk(v.clone()),
                        None => Frame::Null,
                    }
                }
                _ => Frame::Error("Not implement".into()),
            };
            // let response_frame = Frame::Error("Not implement!".to_string());
            connection.write_frame(&response).await.unwrap();
        }
    }
}
