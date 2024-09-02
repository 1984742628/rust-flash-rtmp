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
        if let Some(ref optional_arguments) = command.optional_arguments {
            write_value(buffer, optional_arguments);
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
            optional_arguments: Some(Rc::new(Value::StrictArray(vec![
                Rc::new(Value::Number(1f64))
            ]))),
        };

        let mut rtmp_writer = RTMPWriter::new();
        rtmp_writer.write_amf0_command(&mut encoded_message, command_message);

        println!("{:?}", encoded_message);
    }
}