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
                let _ = stream.read(&mut buf);
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").expect("failed to write response");
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
