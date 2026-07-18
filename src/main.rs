use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("failed to bind to port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => println!("accepted new connection"),
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
