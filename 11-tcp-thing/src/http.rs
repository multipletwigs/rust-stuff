//! Minimal HTTP/1.1 response modeling for our toy server.
//!
//! Design goal: *encapsulation*. The fiddly wire format (status line, headers,
//! CRLFs) lives in exactly one place — the `Display` impl — and the rest of the
//! program just builds a value and prints it.

use std::fmt::{self, Display, Formatter};

/// The subset of HTTP status codes this server knows how to return.
///
/// Bundling the numeric code and its reason phrase into one type makes an
/// invalid pairing — like `400 Not Found` — *impossible to construct*. That's
/// "make illegal states unrepresentable": the type enforces the invariant, so
/// the bug you hit earlier (a 400 labeled "NOT FOUND") simply can't happen.
///
/// It's a *private* enum — callers never name it. They go through the
/// `ok` / `not_found` constructors, which is the whole point of an abstraction:
/// the outside world doesn't see the moving parts.
enum Status {
    Ok,
    NotFound,
}

impl Status {
    /// The numeric status code, e.g. `200`.
    fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::NotFound => 404,
        }
    }

    /// The reason phrase that *always* pairs with this code, e.g. `"OK"`.
    ///
    /// `&'static str` because every reason is a string literal baked into the
    /// binary — no heap allocation, and it outlives any caller.
    fn reason(&self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::NotFound => "Not Found",
        }
    }
}

/// A complete HTTP/1.1 response: a status paired with a body.
///
/// The fields are **private**. You don't build one with a struct literal; you
/// use a constructor and then format it. That keeps the internals free to
/// change without breaking any caller.
pub struct HttpResponse {
    status: Status,
    body: String,
}

impl HttpResponse {
    /// The only protocol version this server speaks. An *associated const* —
    /// it belongs to the type (reach it as `Self::HTTP_VERSION`), not to any
    /// instance. `&'static str` so the struct needs no lifetime parameter.
    const HTTP_VERSION: &'static str = "HTTP/1.1";

    /// Build a `200 OK` response wrapping `body`.
    pub fn ok(body: String) -> Self {
        Self {
            status: Status::Ok,
            body,
        }
    }

    /// Build a `404 Not Found` response wrapping `body`.
    pub fn not_found(body: String) -> Self {
        Self {
            status: Status::NotFound,
            body,
        }
    }
}

/// Renders the response in the exact HTTP/1.1 wire format — the single source
/// of truth for the framing. Read the format string as four CRLF-separated
/// pieces:
///
/// ```text
/// HTTP/1.1 200 OK\r\n        <- status line, terminated by CRLF
/// Content-Length: 42\r\n     <- one header, terminated by CRLF
/// \r\n                       <- the blank line: "headers are done"
/// <body>                     <- the body, exactly Content-Length bytes long
/// ```
///
/// `Content-Length` is *derived* from `self.body.len()` every time, so it can
/// never disagree with the actual body. The trailing `\` on each source line is
/// a line-continuation: it eats the newline + leading whitespace, letting us
/// lay the string out readably without putting real newlines into it.
///
/// Implementing `Display` is what lets `main` do `write!(stream, "{response}")`
/// and `format!`/`to_string()` everywhere — one method, the whole formatting
/// ecosystem speaks our type.
impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{version} {code} {reason}\r\n\
             Content-Length: {length}\r\n\
             \r\n\
             {body}",
            version = Self::HTTP_VERSION,
            code = self.status.code(),
            reason = self.status.reason(),
            length = self.body.len(),
            body = self.body,
        )
    }
}
