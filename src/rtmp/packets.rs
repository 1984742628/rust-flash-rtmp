use flash_lso::types::Value;
use std::rc::Rc;
use crate::chunk::packets::MessageTypeId;

pub struct CommandMessage {
    /// Name of the remote procedure that is
    /// called.
    pub procedure_name: String,

    /// If a response is expected we give a
    /// transaction Id. Else we pass a value of
    /// 0
    pub transaction_id: f64,

    /// If there exists any command info this
    /// is set, else this is set to null type.
    pub command_object: Option<Rc<Value>>,

    /// Any optional arguments to be provided 
    pub optional_arguments: Vec<Rc<Value>>
}

pub enum RTMPMessageType {
    AMF0Command(CommandMessage),
}

pub struct RTMPMessage {
    pub timestamp: u32,
    pub message_type_id: MessageTypeId,
    pub message_stream_id: u32,
    pub payload: Vec<u8>,
}