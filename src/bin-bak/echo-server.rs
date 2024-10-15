use core::panic;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::Arc,
};

use bytes::Bytes;
use tokio::{
    self,
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[allow(unused)]
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // let db:Db=Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (mut socket, addr) = tcp_listener.accept().await.unwrap();
        println!("get request from: {}", addr);
        tokio::spawn(async move {
            process_socket_handle(socket).await;
        });
        // tokio::spawn(async move {
        //     let (mut rd, mut wr) = socket.split();

        //     if io::copy(&mut rd, &mut wr).await.is_err() {
        //         eprintln!("failed to copy");
        //     }
        // });
        // tokio::spawn(async move {
        //     let mut buf = vec![0; 1024];

        //     loop {
        //         match socket.read(&mut buf).await {
        //             // Return value of `Ok(0)` signifies that the remote has
        //             // closed
        //             Ok(0) => return,
        //             Ok(n) => {
        //                 // Copy the data back to socket
        //                 if socket.write_all(&buf[..n]).await.is_err() {
        //                     // Unexpected socket error. There isn't much we can
        //                     // do here so just stop processing.
        //                     return;
        //                 }
        //             }
        //             Err(_) => {
        //                 // Unexpected socket error. There isn't much we can do
        //                 // here so just stop processing.
        //                 return;
        //             }
        //         }
        //     }
        // });
    }
}

async fn process_socket(mut socket: TcpStream) {
    // let (mut read,mut write)= socket.split();
    let (mut read, mut write) = socket.split();
    if io::copy(&mut read, &mut write).await.is_err() {
        eprintln!("failed to copy");
    };
}

async fn process_socket_handle(mut socket: TcpStream) {
    let (mut read, mut write) = socket.split();
    println!("split socket");
    let mut read_buff = [0; 10];
    let mut read_vec = Vec::new();
    loop {
        match read.read(&mut read_buff).await {
            Ok(0) =>{
                println!("read 0");
                break;
            },
            Ok(n) => {
                println!("read {n}");
                read_vec.append(&mut read_buff[..n].to_vec());
                if n<10{
                    break;
                }
            },
            Err(e) => panic!("err,{}", e),
        }
    }
    let request= String::from_utf8(read_vec).unwrap();
    println!("received: {}",request);
   let wr= write.write(request.as_bytes()).await.unwrap();
   println!("write {wr} bytes");
}
