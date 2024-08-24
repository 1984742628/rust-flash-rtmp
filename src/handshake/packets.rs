use crate::handshake::{RTMP_PROTOCOL_VERSION, RANDOM_ECHO_SIZE};

// C0 and S0 Packet (1 byte)
#[derive(Debug, Clone, Copy)]
pub struct Version {
    /// In C0, this field identifies the RTMP version
    /// requested by the client. In S0, this field identifies the RTMP
    /// version selected by the server. The version defined by this
    /// specification is 3. Values 0-2 are deprecated values used by
    /// earlier proprietary products; 4-31 are reserved for future
    /// implementations; and 32-255 are not allowed (to allow
    /// distinguishing RTMP from text-based protocols, which always start
    /// with a printable character). A server that does not recognize the
    /// client’s requested version SHOULD respond with 3. The client MAY
    /// choose to degrade to version 3, or to abandon the handshake.
    pub version: u8,
}

// C1 and S1 Packet (1536 bytes)
#[derive(Debug, Clone, Copy)]
pub struct C1S1Packet {
    /// This field contains a timestamp, which SHOULD be
    /// used as the epoch for all future chunks sent from this endpoint.
    /// This may be 0, or some arbitrary value. To synchronize multiple
    /// chunkstreams, the endpoint may wish to send the current value of
    /// the other chunkstream’s timestamp.
    pub time: u32,

    /// This field MUST be all 0s.
    pub zero: u32,

    /// This field can contain any arbitrary
    /// values. Since each endpoint has to distinguish between the
    /// response to the handshake it has initiated and the handshake
    /// initiated by its peer, this data SHOULD send something sufficiently
    /// random. But there is no need for cryptographically-secure
    /// randomness, or even dynamic values.
    pub random_data: [u8; 1528],
}

// C2 and S2 Packet (1536 bytes)
#[derive(Debug, Clone, Copy)]
pub struct C2S2Packet {
    /// This field MUST contain the timestamp sent by the peer in S1 (for C2) or C1 (for S2).
    pub time: u32,

    /// This field MUST contain the timestamp at which the previous packet (S1 or C1) sent by the peer was read.
    pub time2: u32,

    /// This field MUST contain the random data
    /// field sent by the peer in S1 (for C2) or S2 (for C1). Either peer
    /// can use the time and time2 fields together with the current
    /// timestamp as a quick estimate of the bandwidth and/or latency of
    /// the connection, but this is unlikely to be useful.
    pub random_echo: [u8; 1528],
}

// Combined C0 and C1 Packet (Client -> Server)
#[derive(Debug, Clone, Copy)]
pub struct ClientHello {
    pub c0: Version,
    pub c1: C1S1Packet,
}

// Combined S0, S1, and S2 Packet (Server -> Client)
#[derive(Debug, Clone, Copy)]
pub struct ServerHelloAck {
    pub s0: Version,
    pub s1: C1S1Packet,
    pub s2: C2S2Packet,
}

// Combined C2 Packet and AMF Connect Command (Client -> Server)
#[derive(Debug, Clone)]
pub struct ClientAckAndConnect {
    pub c2: C2S2Packet,
    // TODO: Add AMF connect command
    // pub amf_connect_command: Vec<u8>, // Placeholder for AMF connect command, typically a Vec<u8>
}
