use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let connect_addr = "127.0.0.1:6379";
    let mut socket = TcpStream::connect(connect_addr)
        .expect(format!("connect server :{} error", connect_addr).as_str());
    let request = "hello tcp server";
    socket.write_all(request.as_bytes()).unwrap();
    println!("send seccess!");
    let mut read_buff = [0; 100];
    let mut read_vec = Vec::new();
    loop {
        if let Ok(r) = socket.read(&mut read_buff) {
            if r < 1 {
                break;
            } else {
                read_vec.append(&mut read_buff.to_vec());
            }
        } else {
            break;
        }
    }
    let resp = String::from_utf8(read_vec).unwrap();
    println!("responsed from server:{}", resp);
}
