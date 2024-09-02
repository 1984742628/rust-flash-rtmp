#[derive(Debug, Clone, Copy)]
pub enum HandshakeState {
    Uninitialized,      // Initial state before any packets are exchanged

    ClientHelloSent,    // After the client sends C0 + C1 (ClientHello), and before the server sends S0 + S1 + S2 (ServerHello)

    HandshakeDone,      // After the client sends C2 (ClientAck) + AMF0 connect command
}

#[derive(Debug, Clone)]
pub struct HandshakeResult {
    pub response: Vec<u8>,
    pub is_done: bool
}