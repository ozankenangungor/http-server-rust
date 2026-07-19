use std::{
    collections::HashMap,
    env,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

fn handle_connection(stream: TcpStream, base_dir: PathBuf) {
    let lines: Vec<String> = {
        let reader = BufReader::new(&stream);
        reader
            .lines()
            .map(|l| l.expect("failed to read line"))
            .take_while(|line| !line.is_empty())
            .collect()
    };

    if lines.is_empty() {
        return;
    }

    // "GET /path HTTP/1.1"
    let path = lines[0]
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    // Parse headers with case-insensitive keys
    let headers: HashMap<String, String> = lines[1..]
        .iter()
        .filter_map(|line| {
            let (k, v) = line.split_once(": ")?;
            Some((k.to_ascii_lowercase(), v.to_owned()))
        })
        .collect();

    let response = if path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if let Some(s) = path.strip_prefix("/echo/") {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            s.len(),
            s
        )
    } else if path == "/user-agent" {
        let ua = headers.get("user-agent").map(String::as_str).unwrap_or("");
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            ua.len(),
            ua
        )
    } else if let Some(filename) = path.strip_prefix("/files/") {
        let file_path = base_dir.join(filename);
        match fs::read(&file_path) {
            Ok(contents) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                String::from_utf8_lossy(&contents)
            ),
            Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
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
