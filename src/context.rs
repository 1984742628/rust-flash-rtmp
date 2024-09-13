#[derive(Debug)]
pub enum PeerBandwidthLimit {
    HARD = 0,
    SOFT = 1,
    DYNAMIC = 2
}

#[derive(Debug)]
pub enum HandshakeStatus {
    InProgress,
    Done
}

#[derive(Debug)]
pub enum ObjectEncoding {
    AMF0 = 0,
    AMF3 = 3
}

#[derive(Debug)]
pub struct ConnectionArgs {
    app: String,
    flash_ver: String,
    swf_url: String,
    tc_url: String,
    fpad: bool,
    audio_codecs: u32,
    video_codecs: u32,
    video_function: u32,
    page_url: String,
    object_encoding: ObjectEncoding
}

#[derive(Debug)]
pub struct NetConnectionContext {
    connection_args: ConnectionArgs,

    handshake_status: HandshakeStatus,

    recv_bytes: u32,

    chunk_size: u32,

    window_ack_size: u32,

    peer_bandwidth_limit: PeerBandwidthLimit,

    relative_timestamp: u32,
}