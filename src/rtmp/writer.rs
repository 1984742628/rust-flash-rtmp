use crate::chunk::packets::MessageTypeId;
use crate::chunk::writer::ChunkWriter;
use crate::rtmp::packets::{RTMPMessageType, CommandMessage, RTMPMessage};
use flash_lso::types::{Value, Element};
use flash_lso::amf0::write::write_value;
use std::rc::Rc;

#[derive(Debug)]
pub struct RTMPWriter { }

impl RTMPWriter {
    pub fn new() -> RTMPWriter {
        RTMPWriter { }
    }

    pub fn write_amf0_command(&mut self, buffer: &mut Vec<u8>, command: CommandMessage) {
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