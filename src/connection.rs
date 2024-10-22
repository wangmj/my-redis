use std::io::Cursor;

use crate::{frame::Frame, Result};

use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

pub struct Connection {
    // read_socket:OwnedReadHalf,
    // write_socket:OwnedWriteHalf,
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
}
impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            socket: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
    pub async fn read_frame(&mut self) -> Result<Frame> {
        loop {
            if let Ok(Some(frame)) = self.parse_frame().await {
                return Ok(frame);
            }

            if 0 == self.socket.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(Frame::Null);
                } else {
                    return Err("connection had been closed!".into());
                }
            }
        }
    }
    pub async fn write_frame(&mut self, frame: &Frame) {
        let s = Vec::from(frame);
        println!("write frame len:{:?}", s.len());
        self.socket.write_all(&s).await.unwrap();
        // println!("write {} bytes",writed_count);
        self.socket.flush().await.unwrap();
    }
    async fn parse_frame(&mut self) -> Result<Option<Frame>> {
        println!("parse frame start");
        if self.buffer.is_empty() {
            return Ok(None);
        }
        //从BytesMut中拿出来一个副本。
        let mut cursor: Cursor<&[u8]> = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut cursor) {
            Ok(()) => {
                println!("check frame ok");

                let len = cursor.get_ref().len();
                println!("cursor len:{}", len);
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;
                println!("parse frame:{:?}", frame);
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(e) => {
                println!("check frame error:{:?}", e);
                Err(e.into())
            }
        }
    }
}
