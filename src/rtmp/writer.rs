use crate::chunk::packets::MessageTypeId;
use crate::chunk::writer::ChunkWriter;
use crate::rtmp::packets::{RTMPMessageType, AMF0CommandMessage, RTMPMessage};
use flash_lso::types::{Value, Element};
use flash_lso::amf0::write::write_value;
use std::rc::Rc;

pub struct RTMPWriter { }

impl RTMPWriter {
    pub fn new() -> RTMPWriter {
        RTMPWriter { }
    }

    pub fn write_amf0_command(&mut self, buffer: &mut Vec<u8>, command: AMF0CommandMessage) {
        write_value(buffer, &Rc::new(Value::String(command.procedure_name)));
        write_value(buffer, &Rc::new(Value::Number(command.transaction_id)));

        // Write command object to buffer
        if let Some(ref command_object) = command.command_object {
            write_value(buffer, command_object);
        }
    
        // Write the optional_arguments to the buffer
        for optional_argument in command.optional_arguments {
            write_value(buffer, &optional_argument);
        }
    } 

    pub fn write(&mut self, message: RTMPMessageType) -> Vec<u8> {
        let mut encoded_message: Vec<u8> = Vec::new();
        let message_type_id: MessageTypeId;

        match message {
            RTMPMessageType::AMF0Command(command) => {
                message_type_id = MessageTypeId::CommandAMF0;
                self.write_amf0_command(&mut encoded_message, command)
            }
        };

        let chunk_writer = ChunkWriter::new();
        chunk_writer.write_chunks(
            RTMPMessage {
                timestamp: 0,
                message_type_id,
                message_stream_id: 0,
                payload: encoded_message
            }, 
        3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_amf0_command() {
        let mut encoded_message: Vec<u8> = Vec::new();
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
            optional_arguments: vec![Rc::new(Value::Number(1f64))],
        };

        let mut rtmp_writer = RTMPWriter::new();
        rtmp_writer.write_amf0_command(&mut encoded_message, command_message);

        assert_eq!(encoded_message, vec![2, 0, 7, 99, 111, 110, 110, 101, 99, 116, 0, 63, 240, 0, 0, 0, 0, 0, 0, 3, 0, 11, 118, 105, 100, 101, 111, 67, 111, 100, 101, 99, 115, 0, 64, 111, 128, 0, 0, 0, 0, 0, 0, 11, 97, 117, 100, 105, 111, 67, 111, 100, 101, 99, 115, 0, 64, 168, 238, 0, 0, 0, 0, 0, 0, 8, 102, 108, 97, 115, 104, 86, 101, 114, 2, 0, 13, 87, 73, 78, 32, 49, 48, 44, 49, 44, 56, 53, 44, 51, 0, 3, 97, 112, 112, 2, 0, 3, 97, 112, 112, 0, 5, 116, 99, 85, 114, 108, 2, 0, 6, 116, 99, 95, 117, 114, 108, 0, 13, 118, 105, 100, 101, 111, 70, 117, 110, 99, 116, 105, 111, 110, 0, 63, 240, 0, 0, 0, 0, 0, 0, 0, 12, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 0, 64, 109, 224, 0, 0, 0, 0, 0, 0, 7, 112, 97, 103, 101, 85, 114, 108, 2, 0, 8, 112, 97, 103, 101, 95, 117, 114, 107, 0, 4, 102, 112, 97, 100, 1, 0, 0, 6, 115, 119, 102, 85, 114, 108, 2, 0, 7, 115, 119, 102, 95, 117, 114, 108, 0, 14, 111, 98, 106, 101, 99, 116, 69, 110, 99, 111, 100, 105, 110, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 63, 240, 0, 0, 0, 0, 0, 0]);
    }
}