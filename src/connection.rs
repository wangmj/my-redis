
use bytes::{BufMut, Bytes, BytesMut};
use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::{tcp::{OwnedReadHalf, OwnedWriteHalf}, TcpStream}};
use crate::{frame::Frame, Result};

pub struct Connection{
    // read_socket:OwnedReadHalf, 
    // write_socket:OwnedWriteHalf,
    socket:BufWriter<TcpStream>,
    // buffer:BytesMut,
}
impl Connection{
    pub fn new(socket:TcpStream)->Connection{
    //    let (read,write)= socket.into_split();
        Connection{
            // read_socket:read,
            // write_socket:write,
            socket:BufWriter::new(socket)
            // buffer:BytesMut::with_capacity(4*1024)
        }
    }
    pub async fn read_frame(&mut self)->Result<Frame>{
        let frame=self.parse_frame().await;
        Ok(frame)
    }
    pub async  fn write_frame(&mut self,frame:&Frame){
        // let s:Vec<u8>=frame.into();
        let s= Vec::from(frame);
        println!("write frame len:{:?}",s.len());
         self.socket.write_all(&s).await.unwrap();
        // println!("write {} bytes",writed_count);
        self.socket.flush().await.unwrap();
        
    }
    async  fn parse_frame(&mut self)->Frame{
        let mut buf=Vec::new();
        println!("start read frame");
        let mut tmp_buf=[0;10];
        loop {
            let count= self.socket.read(&mut tmp_buf).await.unwrap();
            println!("read count:{:?}",count);
            if count<10{
                buf.put(&tmp_buf[..count]);
                break;
            }else{
                buf.put(&tmp_buf[..]);
            }
        }
    //    self.socket.read_to_end(& mut buf).await.unwrap();
      println!("read frame len:{:?}",buf.len());
       Frame::from(buf)
    }
}