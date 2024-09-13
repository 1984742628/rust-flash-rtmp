
pub mod tcp_transport;

use std::io::Result;

pub trait Transport: Send {
    fn connect(&mut self, ip: &str, port: u16) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;

    fn read_data(&mut self) -> Result<Vec<u8>>;
    fn write_data(&mut self, data: Vec<u8>) -> Result<()>;
    
    // fn get_bytes_read(&self) -> u64;
    // fn get_bytes_sent(&self) -> u64;
}

