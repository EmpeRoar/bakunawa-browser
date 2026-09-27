//! Bakunawa — Phase 1: a "browser" that fetches a page and prints its text.
//!
//! The whole pipeline is:
//!
//!   URL string ──parse──▶ Url ──load──▶ HTML string ──extract──▶ plain text ──▶ terminal
//!
//! Run it with:
//!   cargo run -- http://example.org/
//!   cargo run -- file://pages/hello.html

// Each `mod` line tells Rust "there is a file src/<name>.rs, include it".
mod html;
mod http;
mod url;

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use url::{Scheme, Url};

/// The page we open when no URL is given on the command line.
const DEFAULT_URL: &str = "http://example.org/";

fn main() {
    // env::args() yields the program name first, then the arguments.
    // `cargo run -- http://x.com/` makes args[1] == "http://x.com/".
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).map(String::as_str).unwrap_or(DEFAULT_URL);

    // `main` can't use `?` nicely, so we delegate to `run` and report errors here.
    if let Err(err) = run(input) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

/// The browser pipeline: parse → load → extract text → print.
fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let url = Url::parse(input)?;
    let body = load(&url)?;
    let text = html::tidy(&html::extract_text(&body));
    print!("{text}");
    Ok(())
}

/// Get the raw HTML for a URL, either from disk or over the network.
fn load(url: &Url) -> Result<String, Box<dyn Error>> {
    match url.scheme {
        Scheme::File => Ok(fs::read_to_string(&url.path)?),
        Scheme::Http => {
            let response = http::request(url)?;
            // Status info goes to stderr so stdout contains only the page text.
            eprintln!("HTTP {} {}", response.status, response.reason);
            if let Some(content_type) = response.headers.get("content-type") {
                eprintln!("Content-Type: {content_type}");
            }
            eprintln!();
            Ok(response.body)
        }
    }
}
