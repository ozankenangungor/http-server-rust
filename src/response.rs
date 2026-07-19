pub struct Response {
    status: &'static str,
    headers: Vec<String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status: &'static str) -> Self {
        Self { status, headers: Vec::new(), body: Vec::new() }
    }

    pub fn ok() -> Self             { Self::new("HTTP/1.1 200 OK") }
    pub fn created() -> Self        { Self::new("HTTP/1.1 201 Created") }
    pub fn not_found() -> Self      { Self::new("HTTP/1.1 404 Not Found") }
    pub fn internal_error() -> Self { Self::new("HTTP/1.1 500 Internal Server Error") }

    pub fn header(mut self, key: &str, value: impl std::fmt::Display) -> Self {
        self.headers.push(format!("{key}: {value}"));
        self
    }

    pub fn body(mut self, data: Vec<u8>) -> Self {
        self.body = data;
        self
    }

    /// Convenience: set Content-Type + Content-Length + body for plain text.
    pub fn text(self, s: &str) -> Self {
        self.header("Content-Type", "text/plain")
            .header("Content-Length", s.len())
            .body(s.as_bytes().to_vec())
    }

    /// Convenience: set Content-Type + Content-Length + body for binary data.
    pub fn octets(self, data: Vec<u8>) -> Self {
        let len = data.len();
        self.header("Content-Type", "application/octet-stream")
            .header("Content-Length", len)
            .body(data)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(self.status.as_bytes());
        out.extend_from_slice(b"\r\n");
        for h in &self.headers {
            out.extend_from_slice(h.as_bytes());
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&self.body);
        out
    }
}
