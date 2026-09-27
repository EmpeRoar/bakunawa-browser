//! A tiny HTTP/1.0 client built directly on a TCP socket.
//!
//! HTTP is just text sent over a network connection. A request looks like:
//!
//!   GET /index.html HTTP/1.0\r\n
//!   Host: example.org\r\n
//!   \r\n                          ← an empty line means "end of headers"
//!
//! and the server replies with:
//!
//!   HTTP/1.0 200 OK\r\n            ← status line
//!   Content-Type: text/html\r\n    ← headers
//!   \r\n
//!   <html>...</html>               ← body
//!
//! We use HTTP/1.0 on purpose: the server closes the connection when it is
//! done, so "read until the connection closes" gives us the whole body.

use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::url::Url;

pub struct Response {
    pub status: u16,
    pub reason: String,
    /// Header names are stored lowercase, because HTTP header names are case-insensitive.
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// Send a GET request for `url` and return the server's response.
pub fn request(url: &Url) -> Result<Response, Box<dyn Error>> {
    // 1. Open a TCP connection. Rust looks up the host's IP address (DNS) for us.
    let mut stream = TcpStream::connect((url.host.as_str(), url.port))?;

    // 2. Write the request text.
    let request = format!(
        "GET {path} HTTP/1.0\r\nHost: {host}\r\nUser-Agent: bakunawa/0.1\r\n\r\n",
        path = url.path,
        host = url.host,
    );
    stream.write_all(request.as_bytes())?;

    // 3. Read every byte until the server closes the connection.
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;

    // 4. Split the bytes into status, headers and body.
    parse_response(&raw)
}

fn parse_response(raw: &[u8]) -> Result<Response, Box<dyn Error>> {
    // Pages aren't always valid UTF-8; "lossy" swaps bad bytes for '�' instead of failing.
    let text = String::from_utf8_lossy(raw);

    let (head, body) = text
        .split_once("\r\n\r\n")
        .ok_or("malformed response: no blank line after headers")?;

    let mut lines = head.split("\r\n");

    // Status line: "HTTP/1.0 200 OK"
    let status_line = lines.next().ok_or("empty response")?;
    let mut parts = status_line.splitn(3, ' ');
    let _version = parts.next().ok_or("missing HTTP version")?;
    let status: u16 = parts.next().ok_or("missing status code")?.parse()?;
    let reason = parts.next().unwrap_or("").to_string();

    // Remaining lines: "Name: value"
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_lowercase(), value.trim().to_string());
        }
    }

    Ok(Response {
        status,
        reason,
        headers,
        body: body.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_basic_response() {
        let raw = b"HTTP/1.0 200 OK\r\nContent-Type: text/html\r\n\r\n<p>Hi</p>";
        let response = parse_response(raw).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.reason, "OK");
        assert_eq!(response.headers["content-type"], "text/html");
        assert_eq!(response.body, "<p>Hi</p>");
    }

    #[test]
    fn rejects_response_without_blank_line() {
        assert!(parse_response(b"HTTP/1.0 200 OK\r\n").is_err());
    }
}
