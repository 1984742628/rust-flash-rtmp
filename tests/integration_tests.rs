use rust_flash_rtmp::handshake::packets::{ClientHello, ServerHelloAck};
use rust_flash_rtmp::handshake::RTMP_PROTOCOL_VERSION;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialzize_client_hello() {
        let client_hello = ClientHello::new(
            RTMP_PROTOCOL_VERSION,
            0,
            [74; 1528]
        );
        let bytes = client_hello.to_bytes();

        println!("{:?}", bytes);
    }

    #[test]
    fn deserialize_s0_s1_s2() {
        let data = include_bytes!(concat!("packet/", "s0_s1_s2", ".dat"));
        let (_, server_hello_ack) = ServerHelloAck::from_bytes(data).expect("Failed to parse ServerHelloAck");

        println!("{:?}", server_hello_ack);
    }
}