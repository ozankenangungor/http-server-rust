use std::{
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

fn handle_connection(stream: TcpStream, base_dir: PathBuf) {
    let mut reader = BufReader::new(&stream);

    // Read request line: "METHOD /path HTTP/1.1"
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .expect("failed to read request line");

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_owned();
    let path = parts.next().unwrap_or("/").to_owned();

    // Read headers until blank line
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .expect("failed to read header line");
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some((k, v)) = trimmed.split_once(": ") {
            headers.insert(k.to_ascii_lowercase(), v.to_owned());
        }
    }

    // reader is now positioned at the body
    let response = match (method.as_str(), path.as_str()) {
        (_, "/") => "HTTP/1.1 200 OK\r\n\r\n".to_string(),

        (_, path) if path.starts_with("/echo/") => {
            let s = path.strip_prefix("/echo/").unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                s.len(),
                s
            )
        }

        (_, "/user-agent") => {
            let ua = headers.get("user-agent").map(String::as_str).unwrap_or("");
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                ua.len(),
                ua
            )
        }

        ("GET", path) if path.starts_with("/files/") => {
            let filename = path.strip_prefix("/files/").unwrap();
            match fs::read(base_dir.join(filename)) {
                Ok(contents) => format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    String::from_utf8_lossy(&contents)
                ),
                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            }
        }

        ("POST", path) if path.starts_with("/files/") => {
            let filename = path.strip_prefix("/files/").unwrap();
            let content_length: usize = headers
                .get("content-length")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).expect("failed to read body");

            match fs::write(base_dir.join(filename), &body) {
                Ok(_) => "HTTP/1.1 201 Created\r\n\r\n".to_string(),
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
            }
        }

        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    (&stream)
        .write_all(response.as_bytes())
        .expect("failed to write response");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir = args
        .windows(2)
        .find(|w| w[0] == "--directory")
        .map(|w| PathBuf::from(&w[1]))
        .unwrap_or_else(|| PathBuf::from("/tmp"));

    let listener = TcpListener::bind("127.0.0.1:4221").expect("failed to bind to port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let dir = base_dir.clone();
                thread::spawn(|| handle_connection(stream, dir));
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
