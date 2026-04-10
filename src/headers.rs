use anyhow::Result;
use std::collections::HashMap;
use std::convert::AsRef;
use std::io::Write;

#[derive(Debug)]
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
        let key = key.into().to_ascii_lowercase();
        self.headers.entry(key).or_default().push(value.into());
    }

    pub fn write_to(&self, buffer: &mut impl Write) -> Result<()> {
        for (k, values) in &self.headers {
            if k.eq_ignore_ascii_case("set-cookie") {
                for v in values {
                    write!(buffer, "{}: {}\r\n", k, v)?;
                }
            } else {
                // write!(buffer, "{}: {}\r\n", k, values.join(","))?;  // join allocate String -
                // NOT GOOD HERE
                write!(buffer, "{}:", k)?;
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(buffer, ",")?;
                    }
                    write!(buffer, "{}", v)?;
                }
                write!(buffer, "\r\n")?;
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers
            .insert(key.into().to_ascii_lowercase(), vec![value.into()]);
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&Vec<String>> {
        self.headers.get(key.as_ref())
    }
}
