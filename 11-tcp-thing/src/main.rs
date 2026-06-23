use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use tcp_thing::http::HttpResponse;

fn main() {
    // `expect` over `unwrap`: if binding fails, the message says *why*.
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("could not bind to 127.0.0.1:7878 — is the port already in use?");
    println!("listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            // A single failed connection shouldn't take the whole server down:
            // log it and keep accepting the next one.
            Err(e) => eprintln!("connection failed: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // We only need the first line — "GET / HTTP/1.1" — to route the request.
    //   `.lines()` yields `io::Result<String>`  (reading a line can fail)
    //   `.next()`  wraps that in `Option`        (the stream might be empty)
    // ...so the type is `Option<Result<String, _>>`. We peel both layers in one
    // pattern; on anything else there's nothing to reply to, so we bail out.
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return,
    };

    // `if` is an expression, so the chosen response flows straight into the
    // binding — no mutable variable, no repeated code.
    let response = if request_line == "GET / HTTP/1.1" {
        let body = fs::read_to_string("./hello.html").unwrap_or_default();
        HttpResponse::ok(body)
    } else {
        HttpResponse::not_found("<h1>404 Not Found</h1>".to_string())
    };

    // Because `HttpResponse` implements `Display`, we can format it straight
    // into the socket — no intermediate `Vec<u8>` needed. `write!` returns an
    // `io::Result`; we log instead of panicking, since a client hanging up
    // mid-write is normal.
    if let Err(e) = write!(stream, "{response}") {
        eprintln!("failed to send response: {e}");
    }
}
