use std::{collections::HashMap, io::BufRead};

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse<R: BufRead>(reader: &mut R) -> Option<Self> {
        // Request line: "METHOD /path HTTP/1.1"
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        let mut parts = line.split_whitespace();
        let method = parts.next()?.to_owned();
        let path = parts.next()?.to_owned();

        // Headers until blank line
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).ok()?;
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                break;
            }
            if let Some((k, v)) = trimmed.split_once(": ") {
                headers.insert(k.to_ascii_lowercase(), v.to_owned());
            }
        }

        // Body — read exactly Content-Length bytes if present
        let body = match headers.get("content-length").and_then(|v| v.parse::<usize>().ok()) {
            Some(len) => {
                let mut buf = vec![0u8; len];
                reader.read_exact(&mut buf).ok()?;
                buf
            }
            None => Vec::new(),
        };

        Some(Request { method, path, headers, body })
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(String::as_str)
    }
}
