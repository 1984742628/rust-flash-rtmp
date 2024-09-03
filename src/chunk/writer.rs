
use std::io::Write;

use crate::chunk::packets::{ChunkBasicHeader, ChunkMessageHeader, RTMPChunk, ExtendedTimestamp};
use crate::rtmp::packets::RTMPMessage;

pub struct ChunkWriter {
    chunk_size: i32
}

impl ChunkWriter {
    pub fn new() -> ChunkWriter {
        ChunkWriter {
            chunk_size: 128
        }
    }

    pub fn set_chunk_size(&mut self, chunk_size: i32) {
        self.chunk_size = chunk_size;
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

    pub fn write_chunks(&self, rtmp_message: RTMPMessage, chunk_stream_id: u32) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut rtmp_chunks: Vec<RTMPChunk> = Vec::new();

        let payload_chunks = rtmp_message.payload.chunks(self.chunk_size as usize);
        let mut remaining = rtmp_message.payload.len();
        let mut first_chunk = false;

        // quick test
        for payload_chunk in payload_chunks {
            if !first_chunk {
                first_chunk = true;
                rtmp_chunks.push(RTMPChunk {
                    basic_header: ChunkBasicHeader { chunk_header_format: 0, chunk_stream_id },
                    message_header: ChunkMessageHeader::Type0 {
                        absolute_timestamp: 0,
                        message_length: remaining as u32, 
                        message_type_id: crate::chunk::packets::MessageTypeId::CommandAMF0, 
                        message_stream_id: 0
                    },
                    extended_timestamp: None,
                    data: payload_chunk.to_vec()
                }); 
            } else {
                rtmp_chunks.push(RTMPChunk {
                    basic_header: ChunkBasicHeader { chunk_header_format: 3, chunk_stream_id },
                    message_header: ChunkMessageHeader::Type3,
                    extended_timestamp: None,
                    data: payload_chunk.to_vec()
                });  
            }

            remaining -= payload_chunk.len()
        };

        for rtmp_chunk in rtmp_chunks {
            self.write_basic_header(rtmp_chunk.basic_header, &mut buffer);
            self.write_message_header(rtmp_chunk.message_header, &mut buffer);
            // write extended timestamp here
            buffer.extend_from_slice(&rtmp_chunk.data);
        };

        buffer
    }
}

#[cfg(test)]
mod tests {

}