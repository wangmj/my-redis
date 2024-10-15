use std::convert;

use tokio::{io::AsyncReadExt, net::TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct Connection {
    socket:TcpStream 
}
impl Connection{
    pub fn new(socket:TcpStream)->Connection{
        Connection{
            socket
        }
    }
    pub async fn read_frame(&mut self)->Result<Frame>{
        let mut buf=Vec::new();
        let mut buffer=[0;1024];
        loop {
            let read_bytes_count= self.socket.read(&mut buffer).await.unwrap();
            if read_bytes_count<1024{
                buf.extend_from_slice(&buffer[..read_bytes_count]);
                break;
            }else{
                buf.extend_from_slice(&buffer);
            }
        }
     
        let frame:Frame= convert::From::from(buf);
        Ok(frame)
    }
    pub  fn write_frame(&mut self,frame:Frame){
        let s:Vec<u8>=frame.into();
        let writed_count= self.socket.try_write(&s).unwrap();
        println!("write {} bytes",writed_count);
    }
}
#[derive(Serialize, Deserialize,Debug)]
pub enum Frame{
    Simple(String),
    Bulk(Command),
    Error(String),
    Integer(u64),
    Null,
    // Arrary(Vec<Frame>)
}

impl From<Frame> for Vec<u8> {
    fn from(frame: Frame) -> Self {
        let json=serde_json::to_string(&frame).unwrap();
        json.into_bytes()
    }
}
impl convert::From<Vec<u8>> for Frame {
    fn from(buf: Vec<u8>) -> Self {
        let json=String::from_utf8(buf).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
#[derive(Serialize, Deserialize,Debug)]
pub enum Command {
    Set {
        key: String,
        val: String,
    },
    Get {
        key: String,
    },
}
impl From<Command> for String {
    fn from(s: Command) -> String {
        serde_json::to_string(&s).unwrap()
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
   fn test_json_command(){
    let cmd=Command::Set { key: String::from("abc"), val: String::from("123") };
   let str= serde_json::to_string(&cmd).unwrap();
   println!("{}",str);
   assert!(str.len()>0);
   }
}