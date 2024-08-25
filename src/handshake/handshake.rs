use crate::handshake::state::{HandshakeState, HandshakeResult};
use crate::handshake::packets::*;
use crate::handshake::errors::HandshakeError;
use crate::handshake::RTMP_PROTOCOL_VERSION;

use rand::Rng;

#[derive(Debug)]
struct SentReceivedPackets {
    client_hello: Option<ClientHello>,
    server_hello_ack: Option<ServerHelloAck>,
    client_ack_and_connect: Option<ClientAckAndConnect>,
}

impl Default for SentReceivedPackets {
    fn default() -> Self {
        SentReceivedPackets {
            client_hello: None,
            server_hello_ack: None,
            client_ack_and_connect: None,
        }
    }
}

#[derive(Debug)]
struct RTMPHandshake {
    state: HandshakeState,
    packets: SentReceivedPackets,
}

impl RTMPHandshake {
    pub fn new() -> Self {
        RTMPHandshake {
            state: HandshakeState::Uninitialized,
            packets: SentReceivedPackets::default(),
        }
    }

    pub fn consume(&mut self, data: Option<&[u8]>) -> Result<HandshakeResult, HandshakeError> {
        match self.state {
            HandshakeState::Uninitialized => Ok(self.handle_uninitialized()),
            HandshakeState::ClientHelloSent => self.handle_client_hello_sent(data),
            HandshakeState::HandshakeDone => Err(HandshakeError::HandshakeAlreadyDone)
        }
    }

    fn handle_uninitialized(&mut self) -> HandshakeResult {
        let client_hello = self.create_client_hello();

        self.state = HandshakeState::ClientHelloSent;
        self.packets.client_hello = Some(client_hello);

        HandshakeResult::InProgress {
            response: client_hello.to_bytes()
        }
    }

    fn handle_client_hello_sent(&mut self, data: Option<&[u8]>) -> Result<HandshakeResult, HandshakeError> {
        let data = data.ok_or(HandshakeError::NoData)?;

        let (_, server_hello_ack) = ServerHelloAck::from_bytes(data).expect("Failed to parse ServerHelloAck");

        // Check if the version is valid
        if server_hello_ack.s0.is_valid() {
            return Err(HandshakeError::VersionError(server_hello_ack.s0.version));
        }

        let client_ack_and_connect = self.create_client_ack_and_connect();

        self.state = HandshakeState::HandshakeDone;
        self.packets.server_hello_ack = Some(server_hello_ack);

        Ok(HandshakeResult::Done {
            response: client_ack_and_connect.to_bytes()
        })

    }

    fn create_client_hello(&self) -> ClientHello {
        let mut rng = rand::thread_rng();

        let handshake_bytes: Vec<u8> = (0..1528).map(|_| rng.gen()).collect();

        ClientHello::new(
            RTMP_PROTOCOL_VERSION,
            0,
            handshake_bytes.try_into().expect("Failed to convert handshake bytes")
        )
    }

    fn create_client_ack_and_connect(&self) -> ClientAckAndConnect {
        let server_hello_ack = self.packets.server_hello_ack.as_ref().expect("ServerHelloAck not received");

        ClientAckAndConnect::new(C2S2Packet {
            time: 0,
            time2: 0,
            random_echo: [0; 1528],
        })
    }
}