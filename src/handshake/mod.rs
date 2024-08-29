pub mod packets;
pub mod state;
pub mod errors;

pub const RTMP_PROTOCOL_VERSION: u8 = 3;
pub const RANDOM_ECHO_SIZE: usize = 1528;

pub use state::{HandshakeState, HandshakeResult};
pub use packets::*;
pub use errors::HandshakeError;

#[derive(Debug)]
pub struct RTMPHandshakeNegiotator {
    state: HandshakeState,
}

impl RTMPHandshakeNegiotator {
    pub fn new() -> Self {
        RTMPHandshakeNegiotator {
            state: HandshakeState::Uninitialized,
        }
    }

    pub fn consume(&mut self, data: Option<&[u8]>) -> Result<HandshakeResult, HandshakeError> {
        match self.state {
            HandshakeState::Uninitialized => Ok(self.handle_uninitialized()),
            HandshakeState::ClientHelloSent => self.handle_server_hello_ack_received(data),
            HandshakeState::HandshakeDone => Err(HandshakeError::HandshakeAlreadyDone)
        }
    }

    fn handle_uninitialized(&mut self) -> HandshakeResult {
        let client_hello = self.create_client_hello();

        self.state = HandshakeState::ClientHelloSent;

        HandshakeResult::InProgress {
            response: client_hello.to_bytes()
        }
    }

    fn handle_server_hello_ack_received(&mut self, data: Option<&[u8]>) -> Result<HandshakeResult, HandshakeError> {
        let data = data.ok_or(HandshakeError::NoData)?;

        let (_, server_hello_ack) = ServerHelloAck::from_bytes(data).expect("Failed to parse ServerHelloAck");

        // Check if the version is valid
        if !server_hello_ack.s0.is_valid() {
            return Err(HandshakeError::VersionError(server_hello_ack.s0.version));
        }

        let client_ack_and_connect = self.create_client_ack_and_connect(server_hello_ack.s1);

        self.state = HandshakeState::HandshakeDone;

        Ok(HandshakeResult::Done {
            response: client_ack_and_connect.to_bytes()
        })

    }

    fn create_client_hello(&self) -> ClientHello {
        let handshake_bytes: Vec<u8> = (0..1528).map(|_| b'x').collect();

        ClientHello::new(
            RTMP_PROTOCOL_VERSION,
            0,
            handshake_bytes.try_into().expect("Failed to convert handshake bytes")
        )
    }

    fn create_client_ack_and_connect(&self, s1: C1S1Packet) -> ClientAckAndConnect {
        ClientAckAndConnect::new(C2S2Packet {
            time: 0,
            time2: 0,
            random_echo: s1.random_data,
        })
    }
}
