pub mod packets;
pub mod state;
pub mod errors;
pub mod handshake;

pub const RTMP_PROTOCOL_VERSION: u8 = 3;
pub const RANDOM_ECHO_SIZE: usize = 1528;