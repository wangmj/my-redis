pub mod server;
pub mod connection;
// use connection::Frame;
pub mod frame;

type Result<T>=std::result::Result<T, Error>;
type Error=Box<dyn Send+Sync+ std::error::Error>;