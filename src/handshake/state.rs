#[derive(Debug, Clone, Copy)]
pub enum HandshakeState {
    Uninitialized,      // Initial state before any packets are exchanged

    ClientHelloSent,    // After the client sends C0 + C1 (ClientHello)
    ServerHelloAckSent, // After the server responds with S0 + S1 + S2 (ServerHelloAck)

    ClientAckSent,      // After the client sends C2 + (optional AMF connect command) (ClientAckAndConnect)
    
    HandshakeDone,      // Final state when the handshake is complete
}
