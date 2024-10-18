pub mod server;
pub mod connection;
// use connection::Frame;
pub mod frame;

type Result<T>=std::result::Result<T, Box<dyn std::error::Error>>;