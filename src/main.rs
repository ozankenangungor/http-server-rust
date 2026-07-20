mod handler;
mod request;
mod response;

use std::{env, net::TcpListener, path::PathBuf, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir = args
        .windows(2)
        .find(|w| w[0] == "--directory")
        .map(|w| PathBuf::from(&w[1]))
        .unwrap_or_else(|| PathBuf::from("/tmp"));

    let listener = TcpListener::bind("127.0.0.1:4221").ejxpect("failed to bind to port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let dir = base_dir.clone();
                thread::spawn(|| handler::handle(stream, dir));
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
