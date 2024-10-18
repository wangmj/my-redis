use std::io::{BufWriter, Cursor, Write};

use bytes::Buf;

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    // Bulk(Bytes),
    Null,
    // Array(Vec<Frame>),
}

impl From<Vec<u8>> for Frame {
    fn from(buf: Vec<u8>) -> Self {
        let mut cursor_reader = Cursor::new(buf);
        let flag = cursor_reader.get_u8();
        match flag {
            b'-' => {
                let line = get_line(&mut cursor_reader);
                Frame::Error(String::from_utf8(line.to_vec()).unwrap())
            }
            b'+' => {
                let line = get_line(&mut cursor_reader);
                Frame::Simple(String::from_utf8(line.to_vec()).unwrap())
            }
            b':' => {
                use atoi::atoi;
                let line = get_line(&mut cursor_reader);
                let num = atoi::<u64>(line);
                match num {
                    Some(num) => Frame::Integer(num),
                    None => Frame::Error("Parse integer error".to_string()),
                }
            }
            b'_'=>{
                Frame::Null
            },
            _ => {
                Frame::Error("Unknow frame type".to_string())
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
        println!("write frame len:{:?}",vec.len());
        vec
    }
}

fn get_line(cursor_reader: &mut Cursor<Vec<u8>>) -> &[u8] {
    let start = cursor_reader.position() as usize;
    let mut end = cursor_reader.get_ref().len();
    for i in start..end {
        if cursor_reader.get_ref()[i] == b'\r' && cursor_reader.get_ref()[i + 1] == b'\n' {
            cursor_reader.set_position((i + 2) as u64);
            end = i;
            break;
        }
    }
    cursor_reader.get_ref()[start..end].as_ref()
}
