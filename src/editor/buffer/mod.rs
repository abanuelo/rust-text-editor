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
        let buffer = Vec::new();
        Self { buffer }
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    pub fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        self.buffer.clear();
        let file_contents = std::fs::read_to_string(path)?;
        for line in file_contents.lines() {
            self.buffer.push(String::from(line));
        }
        Ok(())
    }
}
