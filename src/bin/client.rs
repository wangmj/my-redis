// use std::sync::mpsc::Sender;

use bytes::Bytes;
use mini_redis::client::{self};
use tokio::sync::{mpsc, oneshot::{self, Sender}};

enum Command {
    Get{
        key:String,
        // resp_val:Option<Bytes>,
        resp_sender:Sender<String>
    },
    Set{
        key:String,
        val:Bytes,
        resp_sender:Sender<String>,
        // resp_val:Option<String>
    }
}

#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel::<Command>(100);
    // let tx2=tx.clone();
    let manager= tokio::spawn(async move{
        let mut  client=client::connect("127.0.0.1:6379").await.unwrap();
        match rx.recv().await.unwrap(){
            Command::Set{key,val,resp_sender}=>{
               client.set(key.as_str(), val).await.unwrap();
                resp_sender.send("ok".to_string()).unwrap();
            },
            Command::Get{key,resp_sender:_}=>{
                client.get(key.as_str()).await.unwrap();
            }
        }
    });
    let t1=tokio::spawn(async move{
        let (os_tx,os_rv)= oneshot::channel();
        let set_cmd=Command::Set { key: "hello".to_string(), val: Bytes::from("world") ,resp_sender:os_tx};
        let _= tx.send(set_cmd).await;
        let rec= os_rv.await.unwrap();
            println!("Set got:{:?}",rec);
    });

    let _= manager.await;
    let _=t1.await;
    println!("Complete!");
}