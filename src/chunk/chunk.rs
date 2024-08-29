// Enum for the Chunk Type (fmt)
#[derive(Debug, Clone, Copy)]
pub enum ChunkType {
    Type0, // 11 bytes
    Type1, // 7 bytes
    Type2, // 3 bytes
    Type3, // No header
}

// Struct for the Chunk Basic Header
#[derive(Debug)]
pub struct ChunkBasicHeader {
    pub chunk_type: ChunkType,
    pub stream_id: u32, // The chunk stream ID
}

// Enum for the Chunk Stream ID encoding
#[derive(Debug)]
pub enum ChunkStreamIdEncoding {
    OneByte(u8), // For IDs 2-63
    TwoBytes(u8), // For IDs 64-319
    ThreeBytes(u16), // For IDs 320-65599
}

// Enum for the Message Type ID
#[derive(Debug)]
pub enum MessageTypeId {
    /// Protocol control message 1, Set Chunk Size, is used to notify the
    /// peer of a new maximum chunk size.
    SetChunkSize = 0x01,

    /// Protocol control message 2, Abort Message, is used to notify the peer
    /// if it is waiting for chunks to complete a message, then to discard
    /// the partially received message over a chunk stream. The peer
    /// receives the chunk stream ID as this protocol message’s payload. An
    /// application may send this message when closing in order to indicate
    /// that further processing of the messages is not required.
    AbortMessage = 0x02,

    // The client or the server MUST send an acknowledgment to the peer
    // after receiving bytes equal to the window size. The window size is
    // the maximum number of bytes that the sender sends without receiving
    // acknowledgment from the receiver. This message specifies the
    // sequence number, which is the number of the bytes received so far.
    Acknowledgement = 0x03,

    /// RTMP uses message type ID 4 for User Control messages. These
    /// messages contain information used by the RTMP streaming layer.
    UserControlMessage = 0x04,

    /// The client or the server sends this message to inform the peer of the
    /// window size to use between sending acknowledgments. The sender
    /// expects acknowledgment from its peer after the sender sends window
    /// size bytes. The receiving peer MUST send an Acknowledgement
    /// (Section 5.4.3) after receiving the indicated number of bytes since
    /// the last Acknowledgement was sent, or from the beginning of the
    /// session if no Acknowledgement has yet been sent.
    WindowAcknowledgementSize = 0x05,

    /// The client or the server sends this message to limit the output
    /// bandwidth of its peer. The peer receiving this message limits its
    /// output bandwidth by limiting the amount of sent but unacknowledged
    /// data to the window size indicated in this message. The peer
    /// receiving this message SHOULD respond with a Window Acknowledgement
    /// Size message if the window size is different from the last one sent
    /// to the sender of this message.
    SetPeerBandwidth = 0x06,

    /// The client or the server sends this message to send audio data to the
    /// peer. The message type value of 8 is reserved for audio messages.
    AudioData = 0x08,

    /// The client or the server sends this message to send video data to the
    /// peer. The message type value of 9 is reserved for video messages.
    VideoData = 0x09,

    /// The client or the server sends this message to send Metadata or any
    /// user data to the peer. Metadata includes details about the
    /// data(audio, video etc.) like creation time, duration, theme and so
    /// on.
    DataAMF3 = 0x0F,
    DataAMF0 = 0x12,

    /// A shared object is a Flash object (a collection of name value pairs)
    /// that are in synchronization across multiple clients, instances, and
    /// so on.
    SharedObjectAMF3 = 0x10,
    SharedObjectAMF0 = 0x13,

    /// Command messages carry the AMF-encoded commands between the client
    /// and the server. encoding. These messages are sent to perform some 
    /// operations like connect, createStream, publish, play, pause on the peer. 
    /// Command messages like onstatus, result etc. are used to inform the sender
    /// about the status of the requested commands. A command message
    /// consists of command name, transaction ID, and command object that
    /// contains related parameters. A client or a server can request Remote
    /// Procedure Calls (RPC) over streams that are communicated using the
    /// command messages to the peer.
    CommandAMF0 = 0x14,
    CommandAMF3 = 0x11,

    /// An aggregate message is a single message that contains a series of
    /// RTMP sub-messages using the format described in Section 6.1. Message
    /// type 22 is used for aggregate messages.
    AggregateMessage = 0x16,

}

// Struct for the Chunk Message Header
#[derive(Debug)]
pub struct ChunkMessageHeader {
    pub timestamp: Option<u32>,
    pub timestamp_delta: Option<u32>,
    pub message_length: Option<u32>,
    pub message_type_id: Option<MessageTypeId>,
    pub message_stream_id: Option<u32>,
}

