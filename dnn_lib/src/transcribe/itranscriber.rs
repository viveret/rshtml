pub struct PeekableByteStream {
    pub peek_data: Vec<u8>,
    pub peek_index: usize,
    pub peek_offset: usize,
}

pub trait ByteWriter {
    fn write(&self, data: &[u8]);
}

pub trait ITranscriber {
    fn transcribe(&self, data_src: PeekableByteStream, data_dest: &dyn ByteWriter) -> usize;
}

pub trait IEncoder {
    fn encode(&self, data_src: PeekableByteStream, data_dest: &dyn ByteWriter) -> usize;
}

pub trait IDecoder {
    fn decode(&self, data_src: PeekableByteStream, data_dest: &dyn ByteWriter) -> usize;
}