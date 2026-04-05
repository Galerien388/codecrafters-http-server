use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "OK"),
            StatusCode::NotFound => write!(f, "Not Found"),
        }
    }
}

pub struct Response {
    pub code: StatusCode,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self { code }
    }
}

impl Response {
    pub fn to_http_bytes(&self) -> Vec<u8> {
        let nr = self.code.clone() as u16;
        let respone = format!("HTTP/1.1 {} {}\r\n\r\n", nr, &self.code);
        respone.as_bytes().to_vec()
    }
}
