use flate2::{write::GzEncoder, Compression};
use std::{
    fs,
    io::{BufReader, Write},
    net::TcpStream,
    path::PathBuf,
};

use crate::{request::Request, response::Response};

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("gzip write failed");
    encoder.finish().expect("gzip finish failed")
}

pub fn handle(stream: TcpStream, base_dir: PathBuf) {
    let mut reader = BufReader::new(&stream);
    let Some(req) = Request::parse(&mut reader) else { return };

    let response = route(&req, &base_dir);
    (&stream)
        .write_all(&response.into_bytes())
        .expect("failed to write response");
}

fn route(req: &Request, base_dir: &PathBuf) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        (_, "/") => Response::ok(),

        (_, path) if path.starts_with("/echo/") => {
            let s = path.strip_prefix("/echo/").unwrap();
            let accepts_gzip = req
                .header("accept-encoding")
                .map(|v| v.split(',').any(|enc| enc.trim() == "gzip"))
                .unwrap_or(false);

            if accepts_gzip {
                let compressed = gzip(s.as_bytes());
                Response::ok()
                    .header("Content-Type", "text/plain")
                    .header("Content-Encoding", "gzip")
                    .header("Content-Length", compressed.len())
                    .body(compressed)
            } else {
                Response::ok().text(s)
            }
        }

        (_, "/user-agent") => {
            let ua = req.header("user-agent").unwrap_or("");
            Response::ok().text(ua)
        }

        ("GET", path) if path.starts_with("/files/") => {
            let filename = path.strip_prefix("/files/").unwrap();
            match fs::read(base_dir.join(filename)) {
                Ok(contents) => Response::ok().octets(contents),
                Err(_) => Response::not_found(),
            }
        }

        ("POST", path) if path.starts_with("/files/") => {
            let filename = path.strip_prefix("/files/").unwrap();
            match fs::write(base_dir.join(filename), &req.body) {
                Ok(_) => Response::created(),
                Err(_) => Response::internal_error(),
            }
        }

        _ => Response::not_found(),
    }
}
