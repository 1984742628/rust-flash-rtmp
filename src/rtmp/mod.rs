pub mod packets;
mod reader;
mod writer;

use crate::handshake::{RTMPHandshakeNegotiator};
use crate::rtmp::writer::RTMPWriter;
use crate::rtmp::packets::AMF0CommandMessage;
use flash_lso::types::{Value, Element};
use packets::RTMPMessageType;
use std::rc::Rc;

#[derive(Debug)]
struct RTMPConnectionArgs {
    tc_url: String,
    page_url: String,
    swf_url: String,
    app: String
}

#[derive(Debug)]
struct RTMPClient {
    negotiator: Option<RTMPHandshakeNegotiator>,
    connection_args: RTMPConnectionArgs,
}

impl RTMPClient {
    pub fn new(connection_args: RTMPConnectionArgs) -> RTMPClient {
        RTMPClient {
            negotiator: Some(RTMPHandshakeNegotiator::new()),
            connection_args
        }
    }

    fn negotiate_handshake(&mut self, data: Option<Vec<u8>>) -> Vec<u8> {
        let negotiator = self.negotiator.as_mut().unwrap();
        let result;

        match data {
            Some(data) => result = negotiator.consume(Some(data.as_slice())),
            None => result = negotiator.consume(None)
        }

        match result {
            Ok(handshake_result) => {
                let mut response = handshake_result.response;
                let mut additional_response: Vec<u8> = Vec::new();

                if handshake_result.is_done {
                    self.negotiator = None;
                    let mut rtmp_writer = RTMPWriter::new();

                    let command_message = AMF0CommandMessage {
                        procedure_name: String::from("connect"),
                        transaction_id: 1f64,
                        command_object: Some(Rc::new(Value::Object(
                            vec![
                                Element { name: String::from("videoCodecs"), value: Rc::new(Value::Number(252.0)) },
                                Element { name: String::from("audioCodecs"), value: Rc::new(Value::Number(3191.0)) },
                                Element { name: String::from("flashVer"), value: Rc::new(Value::String(String::from("WIN 10,1,85,3"))) },
                                Element { name: String::from("app"), value: Rc::new(Value::String(String::from("app"))) },
                                Element { name: String::from("tcUrl"), value: Rc::new(Value::String(String::from("tc_url"))) },
                                Element { name: String::from("videoFunction"), value: Rc::new(Value::Number(1.0)) },
                                Element { name: String::from("capabilities"), value: Rc::new(Value::Number(239.0)) },
                                Element { name: String::from("pageUrl"), value: Rc::new(Value::String(String::from("page_urk"))) },
                                Element { name: String::from("fpad"), value: Rc::new(Value::Bool(false)) },
                                Element { name: String::from("swfUrl"), value: Rc::new(Value::String(String::from("swf_url"))) },
                                Element { name: String::from("objectEncoding"), value: Rc::new(Value::Number(0.0)) },
                            ],
                            None
                        ))),
                        optional_arguments: None
                    };

                    let res = rtmp_writer.write(RTMPMessageType::AMF0Command(command_message));
                    additional_response = res;
                }

                response.append(&mut additional_response);

                return response;
            },
            Err(error) => panic!("Handshake Consume returned an error: {:?}", error)
        }
    }

    pub fn on_connect(&mut self) -> Vec<u8> {
        if !self.negotiator.is_some() {
            panic!("Handshake Negotiator is None")
        };

        self.negotiate_handshake(None)
    }

    pub fn on_message(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
        if self.negotiator.is_some() {
            let negotiation = self.negotiate_handshake(Some(data));
            return Some(negotiation)
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::io::{Read, Write};

    #[test]
    fn test_connect() {
        // Replace with appropriate server address and port
        let server_addr = "127.0.0.1:1935";

        // Establish a TCP connection to the RTMP server
        let mut stream = TcpStream::connect(server_addr).unwrap();

        // Create an RTMPConnectionArgs instance
        let connection_args = RTMPConnectionArgs {
            tc_url: String::new(),
            page_url: String::new(),
            swf_url: String::new(),
            app: String::new(),
        };

        // Create the RTMPClient instance
        let mut rtmp_client = RTMPClient::new(connection_args);

        // Perform the initial handshake
        let handshake_response = rtmp_client.on_connect();

        // Send the handshake response to the server
        stream.write_all(&handshake_response).unwrap();

        // Buffer to store incoming data
        let mut buffer = vec![0; 4096];

        loop {
            // Read data from the server
            let bytes_read = stream.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                continue;
            }

            // Process the incoming message
            if let Some(response) = rtmp_client.on_message(buffer[..bytes_read].to_vec()) {
                // Send the response back to the server if there's any
                stream.write_all(&response).unwrap();
            }

            buffer.clear();
        }
    }
}