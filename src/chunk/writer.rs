
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

    fn write_basic_header(&self, basic_header: ChunkBasicHeader, buffer: &mut Vec<u8>) {}

    fn write_message_header(&self, message_header: ChunkMessageHeader, buffer: &mut Vec<u8>) {} 

    pub fn write_chunks(&self, rtmp_message: RTMPMessage, chunk_stream_id: u32) -> Vec<u8> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_write_basic_header() {
        let header = ChunkBasicHeader {
            chunk_header_format: 1, // Example format value
            chunk_stream_id: 3,     // Example stream ID
        };
    
        let chunk_writer = ChunkWriter::new();
        let mut vec = Vec::new();
        chunk_writer.write_basic_header(header, &mut vec);
        println!("{:?}", vec); // Output the result for debugging
    }

    #[test]
    pub fn test_write_chunks() {
        let chunk_writer = ChunkWriter::new();
        let bytes = chunk_writer.write_chunks(RTMPMessage {
            timestamp: 0,
            message_type_id: crate::chunk::packets::MessageTypeId::CommandAMF0,
            message_stream_id: 0,
            payload: vec![2, 0, 7, 99, 111, 110, 110, 101, 99, 116, 0, 63, 240, 0, 0, 0, 0, 0, 0, 3, 0, 11, 118, 105, 100, 101, 111, 67, 111, 100, 101, 99, 115, 0, 64, 111, 128, 0, 0, 0, 0, 0, 0, 11, 97, 117, 100, 105, 111, 67, 111, 100, 101, 99, 115, 0, 64, 168, 238, 0, 0, 0, 0, 0, 0, 8, 102, 108, 97, 115, 104, 86, 101, 114, 2, 0, 13, 87, 73, 78, 32, 49, 48, 44, 49, 44, 56, 53, 44, 51, 0, 3, 97, 112, 112, 2, 0, 3, 97, 112, 112, 0, 5, 116, 99, 85, 114, 108, 2, 0, 6, 116, 99, 95, 117, 114, 108, 0, 13, 118, 105, 100, 101, 111, 70, 117, 110, 99, 116, 105, 111, 110, 0, 63, 240, 0, 0, 0, 0, 0, 0, 0, 12, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 0, 64, 109, 224, 0, 0, 0, 0, 0, 0, 7, 112, 97, 103, 101, 85, 114, 108, 2, 0, 8, 112, 97, 103, 101, 95, 117, 114, 107, 0, 4, 102, 112, 97, 100, 1, 0, 0, 6, 115, 119, 102, 85, 114, 108, 2, 0, 7, 115, 119, 102, 95, 117, 114, 108, 0, 14, 111, 98, 106, 101, 99, 116, 69, 110, 99, 111, 100, 105, 110, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 1, 0, 63, 240, 0, 0, 0, 0, 0, 0]
        }, 3);

        println!("{:?}", bytes);
    }
}