use std::{fmt::write, io::{BufWriter, Cursor, Write}, path::Display, string::FromUtf8Error};

use bytes::Buf;

#[derive(Debug)]
pub enum Error {
    InComplete,
    Other(crate::Error)
  } 

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    // Bulk(Bytes),
    Null,
    // Array(Vec<Frame>),
}
impl Frame{
    pub fn check(buf:& mut Cursor<&[u8]>)->Result<(),Error>{
        let first_byte=get_u8(buf)?;
        match first_byte {
            b'-' => {
                get_line(buf)?;
                Ok(())
                // Frame::Error(String::from_utf8(line.to_vec()).unwrap())
            }
            b'+' => {
                let _ = get_line(buf)?;
                // Frame::Simple(String::from_utf8(line.to_vec()).unwrap())
                Ok(())
            }
            b':' => {
                use atoi::atoi;
                let line = get_line(buf)?;
                let num = atoi::<u64>(line);
                match num {
                    Some(_) => Ok(()),
                    None => Err("not found integer".into()),
                }
            }
            b'_'=>{
                Ok(())
            },
            _ => {
                std::result::Result::Err("Unknow frame type".into())
            }
        }
        // Ok(Frame::Null)
    }

    pub fn parse(buf:& mut Cursor<&[u8]>)->Result<Frame,Error>{
        let first_byte=get_u8(buf)?;
        match first_byte {
            b'-'=>{
                let line=get_line(buf)?;
                Ok(Frame::Error(String::from_utf8(line.to_vec())?))
            },
            b'+'=>{
                let line=get_line(buf)?;
                Ok(Frame::Simple(String::from_utf8(line.to_vec())?))
            },
            b':'=>{
                use atoi::atoi;
                let line = get_line(buf)?;
                let num = atoi::<u64>(line);
                match num {
                    Some(i) => Ok(Frame::Integer(i)),
                    None => Err("not found integer".into()),
                }
            }
            b'_'=>{
                Ok(Frame::Null)
            },
            _ => {
                std::result::Result::Err("Unknow frame type".into())
            }
        }
    }
}

impl From<&Frame> for Vec<u8>{
    fn from(frame: &Frame) -> Self{
        println!("frame:{:?}",frame);
        let  mut  vec =Vec::new();
        let mut writer= BufWriter::new(&mut vec);
        match frame {
            Frame::Simple(s) =>{
              writer.write(b'+'.to_be_bytes().as_slice()).unwrap();
              writer.write_all(s.as_bytes()).unwrap();
              writer.write_all("\r\n".as_bytes()).unwrap();
            },
            Frame::Error(e) => {
                writer.write(b'-'.to_be_bytes().as_slice()).unwrap();
                writer.write_all(e.as_bytes()).unwrap();
                writer.write_all("\r\n".as_bytes()).unwrap();
            },
            Frame::Integer(i) => {
                writer.write(b':'.to_be_bytes().as_slice()).unwrap();
                writer.write_all(i.to_string().as_bytes()).unwrap();
                writer.write_all("\r\n".as_bytes()).unwrap();
            },
            Frame::Null => {
                writer.write(b'_'.to_be_bytes().as_slice()).unwrap();
                },
        };
        writer.flush().unwrap();
        drop(writer);
        vec
    }
}

fn get_line<'a>(cursor_reader: &mut Cursor<&'a [u8]>) -> Result<&'a [u8],Error> {
    let start = cursor_reader.position() as usize;
    let mut end = cursor_reader.get_ref().len();
    for i in start..end {
        if cursor_reader.get_ref()[i] == b'\r' && cursor_reader.get_ref()[i + 1] == b'\n' {
            cursor_reader.set_position((i + 2) as u64);
            end = i;
            break;
        }
    }
   let res= cursor_reader.get_ref()[start..end].as_ref();
   Ok(res)
}

fn get_u8(buf:& mut Cursor<&[u8]>)->Result<u8,Error>{
 if !buf.has_remaining(){
    Err(Error::InComplete)
 }else {
        Ok(buf.get_u8())
 }
}

impl From<&str> for Error{
    fn from(value: &str) -> Self {
        Error::Other(value.into())
    }
}
impl From<FromUtf8Error> for Error{
    fn from(value: FromUtf8Error) -> Self {
        Error::Other(value.into())
    }
    
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InComplete => write!(f,"not complete frame  "),
            Error::Other(error) => error.fmt(f),
        }
    }
}