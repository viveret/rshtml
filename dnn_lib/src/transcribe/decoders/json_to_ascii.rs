use crate::transcribe::itranscriber::{PeekableByteStream, ByteWriter};



pub trait IJsonToAscii {
    fn json_to_ascii(&self, data_src: serde_json::Value, data_dest: &dyn ByteWriter) -> usize;
}

pub struct JsonToAsciiDefault {}

impl IJsonToAscii for JsonToAsciiDefault {
    fn json_to_ascii(&self, data_src: serde_json::Value, data_dest: &dyn ByteWriter) -> usize {
        let s = data_src.to_string();
        let b = s.as_bytes();
        data_dest.write(b);
        b.len()
    }
}

pub struct JsonToAscii {
    pub json_to_ascii: Box<dyn IJsonToAscii>,
}

impl JsonToAscii {
    pub fn new(json_to_ascii: Box<dyn IJsonToAscii>) -> Self {
        Self {
            json_to_ascii: json_to_ascii,
        }
    }

    pub fn default() -> Self {
        Self {
            json_to_ascii: Box::new(JsonToAsciiDefault {}),
        }
    }
}