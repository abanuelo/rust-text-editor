use std::string::String;

pub struct Buffer {
    pub buffer: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        let mut buffer = Vec::new();
        buffer.push(String::from("Hello, World!"));
        Self { buffer }
    }
}
