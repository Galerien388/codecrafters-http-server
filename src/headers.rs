use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;

pub struct Headers {
    headers: HashMap<String, Vec<String>>,
}

impl Headers {
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

    pub fn write_to(&self, mut buffer: impl Write) -> Result<()> {
        for (k, values) in &self.headers {
            if k.eq_ignore_ascii_case("set-cookie") {
                for v in values {
                    write!(buffer, "{}: {}\r\n", k, v)?;
                }
            } else {
                write!(buffer, "{}: {}\r\n", k, values.join(","))?;
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}
