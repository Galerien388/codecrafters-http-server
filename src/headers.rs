use std::collections::HashMap;

pub struct Header {
    headers: HashMap<String, Vec<String>>,
}

impl Header {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers
            .entry(key.into())
            .or_default()
            .push(value.into());
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        for (k, values) in &self.headers {
            if k.to_lowercase() == "set-cookie" {
                for v in values {
                    buffer.extend(format!("{}: {}\r\n", k, v).bytes());
                }
            } else {
                let s = values.join(",");
                buffer.extend(format!("{}: {}\r\n", k, s).bytes());
            }
        }
        buffer.extend("\r\n".as_bytes());
        buffer
    }

    pub fn is_empty(&self) -> bool {
        return self.headers.is_empty();
    }
}
