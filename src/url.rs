//! URL parsing.
//!
//! A URL like `http://example.org:8080/index.html` has four parts we care about:
//!
//!   scheme  = "http"          (how to fetch it)
//!   host    = "example.org"   (which computer)
//!   port    = 8080            (which "door" on that computer; http defaults to 80)
//!   path    = "/index.html"   (which resource on that computer)

/// The protocols this browser understands. Adding `Https` is part of Phase 2.
#[derive(Debug, Clone, PartialEq)]
pub enum Scheme {
    Http,
    File,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    pub scheme: Scheme,
    pub host: String,
    pub port: u16,
    pub path: String,
}

impl Url {
    /// Turn a string into a `Url`, or explain why we can't.
    pub fn parse(input: &str) -> Result<Url, String> {
        // "http://example.org/a" → ("http", "example.org/a")
        let (scheme_str, rest) = input
            .split_once("://")
            .ok_or_else(|| format!("missing '://' in URL: {input}"))?;

        let scheme = match scheme_str {
            "http" => Scheme::Http,
            "file" => Scheme::File,
            "https" => {
                return Err("https is not supported yet (see Phase 2 in docs/ROADMAP.md)".into())
            }
            other => return Err(format!("unsupported scheme: {other}")),
        };

        if scheme == Scheme::File {
            // "file:///C:/page.html" gives rest = "/C:/page.html"; Windows wants "C:/page.html".
            let bytes = rest.as_bytes();
            let path = if bytes.len() > 2 && bytes[0] == b'/' && bytes[2] == b':' {
                &rest[1..]
            } else {
                rest
            };
            return Ok(Url {
                scheme,
                host: String::new(),
                port: 0,
                path: path.to_string(),
            });
        }

        // "example.org:8080/a/b" → ("example.org:8080", "/a/b")
        let (host_and_port, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, "/"),
        };

        // "example.org:8080" → ("example.org", 8080)
        let (host, port) = match host_and_port.split_once(':') {
            Some((h, p)) => (h, p.parse::<u16>().map_err(|_| format!("invalid port: {p}"))?),
            None => (host_and_port, 80),
        };

        if host.is_empty() {
            return Err(format!("URL has no host: {input}"));
        }

        Ok(Url {
            scheme,
            host: host.to_string(),
            port,
            path: path.to_string(),
        })
    }
}

// `cargo test` runs everything marked #[test].
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_http_url() {
        let url = Url::parse("http://example.org/index.html").unwrap();
        assert_eq!(url.scheme, Scheme::Http);
        assert_eq!(url.host, "example.org");
        assert_eq!(url.port, 80);
        assert_eq!(url.path, "/index.html");
    }

    #[test]
    fn missing_path_defaults_to_root() {
        let url = Url::parse("http://example.org").unwrap();
        assert_eq!(url.path, "/");
    }

    #[test]
    fn parses_custom_port() {
        let url = Url::parse("http://localhost:8000/test").unwrap();
        assert_eq!(url.host, "localhost");
        assert_eq!(url.port, 8000);
    }

    #[test]
    fn parses_windows_file_url() {
        let url = Url::parse("file:///C:/pages/hello.html").unwrap();
        assert_eq!(url.scheme, Scheme::File);
        assert_eq!(url.path, "C:/pages/hello.html");
    }

    #[test]
    fn rejects_unknown_scheme() {
        assert!(Url::parse("ftp://example.org/").is_err());
        assert!(Url::parse("not a url").is_err());
    }
}
