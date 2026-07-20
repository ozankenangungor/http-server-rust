# http-server-rust

A from-scratch HTTP/1.1 server in Rust. No frameworks, no external HTTP libraries — just `std` and a couple of small crates.

Did this as part of the [CodeCrafters HTTP Server challenge](https://app.codecrafters.io/courses/http-server/overview). Each commit is a new stage.

## Features

- `GET /echo/{str}` — echoes back the string
- `GET /user-agent` — returns the User-Agent header
- `GET /files/{filename}` and `POST /files/{filename}` — read/write files from a directory passed via `--directory`
- gzip compression when the client sends `Accept-Encoding: gzip`
- Keep-alive connections — handles multiple requests on the same TCP connection
- Concurrent — each connection gets its own thread

## How to run

```bash
cargo build --release
./target/release/codecrafters-http-server --directory /some/dir
```

Try it:

```bash
curl http://localhost:4221/echo/hello
curl -H "Accept-Encoding: gzip" http://localhost:4221/echo/hello | gunzip
curl -X POST -d "hello" http://localhost:4221/files/test.txt
curl http://localhost:4221/files/test.txt
```
