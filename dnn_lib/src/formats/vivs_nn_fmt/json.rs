

pub trait IJsonNodeDecoder {
    fn decode(&self, json: serde_json::Value) -> std::io::Result<Box<dyn INode>>;
}

pub struct JsonNodeDecoderDefault {}

impl JsonNodeDecoderDefault {
    pub fn new() -> Self {
        Self {}
    }
}

impl IJsonNodeDecoder for JsonNodeDecoderDefault {
    fn decode(&self, json: serde_json::Value) -> std::io::Result<Box<dyn INode>> {
        let node_type = json["node_type"].as_str().unwrap();
        match node_type {
            "value_node" => {
                let node = ValueNode::new();
                Ok(Box::new(node))
            }
            "generic_layer" => {
                let node = GenericLayer::new();
                Ok(Box::new(node))
            }
            _ => {
                let err = std::io::Error::new(std::io::ErrorKind::Other, "Unknown node type");
                Err(err)
            }
        }
    }
}

pub trait IJsonNodeEncoder {
    fn encode(&self, node: &dyn INode) -> serde_json::Value;
}

pub struct JsonNodeEncoderDefault {}

impl JsonNodeEncoderDefault {
    pub fn new() -> Self {
        Self {}
    }
}

impl IJsonNodeEncoder for JsonNodeEncoderDefault {
    fn encode(&self, node: &dyn INode) -> serde_json::Value {
        let mut json = serde_json::Value::new_object();
        json["node_type"] = serde_json::Value::String(node.get_node_type());
        json
    }
}