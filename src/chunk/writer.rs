
use std::io::Write;

use crate::chunk::packets::{ChunkBasicHeader, ChunkMessageHeader, RTMPChunk, ExtendedTimestamp};
// use crate::rtmp::packets::RTMPMessage;

pub struct ChunkWriter {}

impl ChunkWriter {
    pub fn new() -> ChunkWriter {
        ChunkWriter { }
    }

    fn write_basic_header(&self, basic_header: ChunkBasicHeader, buffer: &mut Vec<u8>) {
        let format = basic_header.chunk_header_format;
        let chunk_stream_id = basic_header.chunk_stream_id;

        if chunk_stream_id >= 64 + 255 {
            buffer.push(format << 6 | 1);
            buffer.push(((chunk_stream_id - 64) >> 8) as u8);
            buffer.push((chunk_stream_id - 64) as u8);
        } else if chunk_stream_id >= 64 {
            buffer.push(format << 6);
            buffer.push((chunk_stream_id - 64) as u8);
        } else {
            buffer.push(format << 6 | chunk_stream_id as u8);
        }
    }

    fn write_message_header(&self, message_header: ChunkMessageHeader, buffer: &mut Vec<u8>) {
        match message_header {
            ChunkMessageHeader::Type0 { 
                absolute_timestamp,
                message_length, 
                message_type_id, 
                message_stream_id 
            } => {
                // TODO: maybe move to a util?
                // write timestamp as u24
                buffer.push((absolute_timestamp >> 16) as u8);
                buffer.push((absolute_timestamp >> 8) as u8);
                buffer.push(absolute_timestamp as u8);  

                // same goes for length
                buffer.push((message_length >> 16) as u8);
                buffer.push((message_length >> 8) as u8);
                buffer.push(message_length as u8);  

                buffer.push(message_type_id as u8);

                // uses little endian
                buffer.extend_from_slice(&message_stream_id.to_le_bytes());
            },
            _ => {}
        }
    } 

    // pub fn write_chunks(&self, rtmp_message: RTMPMessage, chunk_stream_id: u32) -> Vec<u8> {
    //     let mut buffer: Vec<u8> = Vec::new();
    //     let mut rtmp_chunks: Vec<RTMPChunk> = Vec::new();

    //     let payload_chunks = rtmp_message.payload.chunks(self.chunk_size as usize);
    //     let mut remaining = rtmp_message.payload.len();
    
    //     vec![]
    // }
}