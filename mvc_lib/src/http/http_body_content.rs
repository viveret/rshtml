

#[derive(Clone)]
pub struct ContentType {
    pub mime_type: String,
    pub options: String,
}

impl ContentType {
    pub fn parse(mime: &str) -> Self {
        Self {
            mime_type: mime.to_string(),
            options: "".to_string(),
        }
    }

    fn new(mime: &str) -> ContentType {
        Self {
            mime_type: mime.to_string(),
            options: "".to_string(),
        }
    }
}

pub trait IBodyContent {
    fn get_self_type(self: &Self) -> ContentType;
    fn get_content_type(self: &Self) -> ContentType;
    fn get_content_length(self: &Self) -> usize;
    // fn get_body_raw(self: &Self) -> &Vec<u8>;

    fn data(self: &Self) -> &dyn std::any::Any;

    // returns a string representation of the body content for debugging.
    // do not use this for decoding the body content, instead use the get_body_raw method.
    fn to_string(self: &Self) -> String;
}


pub struct GenericBodyContent {
    pub content_type: ContentType,
    pub content_length: usize,
    pub body_raw: Vec<u8>,
}

impl GenericBodyContent {
    pub fn new(content_type: ContentType, content_length: usize, body_raw: Vec<u8>) -> Self {
        Self {
            content_type,
            content_length,
            body_raw,
        }
    }
}

impl IBodyContent for GenericBodyContent {
    fn get_content_type(self: &Self) -> ContentType {
        self.content_type.clone()
    }

    fn get_content_length(self: &Self) -> usize {
        self.content_length
    }

    // fn get_body_raw(self: &Self) -> &Vec<u8> {
    //     self.body_raw.as_ref()
    // }

    fn data(self: &Self) -> &dyn std::any::Any {
        &self.body_raw
    }

    fn get_self_type(self: &Self) -> ContentType {
        ContentType::new("GenericBodyContent")
    }

    fn to_string(self: &Self) -> String {
        format!("GenericBodyContent ({}): {}", self.content_type.mime_type, String::from_utf8_lossy(self.body_raw.as_ref()))
    }
}