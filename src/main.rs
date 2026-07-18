use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("failed to bind to port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                let n = stream.read(&mut buf).expect("failed to read");
                let request = std::str::from_utf8(&buf[..n]).unwrap_or("");

                // "GET /path HTTP/1.1\r\n..."
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("");

                let response = match path {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                stream.write_all(response.as_bytes()).expect("failed to write response");
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
