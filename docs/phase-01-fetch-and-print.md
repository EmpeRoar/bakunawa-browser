# Phase 1 — Fetch & print

Goal: the smallest thing that deserves to be called a browser. Give it a URL,
it downloads the page and shows you the text.

```powershell
cargo run -- http://example.org/
```

```
HTTP 200 OK
Content-Type: text/html

Example Domain
Example Domain
This domain is for use in illustrative examples in documents. ...
```

(The first "Example Domain" is the `<title>`; we don't know the difference
between the head and the body yet — that comes with the HTML parser in Phase 5.)

## The pipeline

```
"http://example.org/"
        │  url.rs   Url::parse
        ▼
Url { scheme: Http, host: "example.org", port: 80, path: "/" }
        │  http.rs  request  (or fs::read_to_string for file://)
        ▼
"<!doctype html><html><head><title>Example Domain</title>..."
        │  html.rs  extract_text + tidy
        ▼
"Example Domain\nThis domain is for use in..."
        │  main.rs  print!
        ▼
    terminal
```

Read the files in that order: [main.rs](../src/main.rs) → [url.rs](../src/url.rs) →
[http.rs](../src/http.rs) → [html.rs](../src/html.rs). Every file has comments
explaining what's going on.

## Step 1 — Parsing the URL ([url.rs](../src/url.rs))

A URL is split into **scheme**, **host**, **port** and **path**. We store it in a
`struct`, and the scheme in an `enum`:

```rust
pub enum Scheme { Http, File }

pub struct Url {
    pub scheme: Scheme,
    pub host: String,
    pub port: u16,
    pub path: String,
}
```

**Rust concepts:**

- **`struct`** groups related data. **`enum`** is "one of these options". Using an
  enum means the compiler forces you to handle every scheme in a `match` — try
  adding `Https` to the enum and see where `cargo build` complains.
- **`String` vs `&str`** — `String` owns its text (can grow, lives on the heap);
  `&str` is a *borrowed view* into some text. `Url::parse(input: &str)` only needs
  to look at the input, so it borrows. The `Url` must keep its host after the
  input is gone, so it stores an owned `String` (`.to_string()` makes the copy).
- **`Result<T, E>`** — Rust has no exceptions. A function that can fail returns
  either `Ok(value)` or `Err(error)`.
- **The `?` operator** — "if this is an `Err`, return it from my function right
  now; otherwise unwrap the `Ok`". It keeps error handling short.
- **`split_once`, `find`, slicing `&rest[..i]`** — everyday string tools.

## Step 2 — Speaking HTTP ([http.rs](../src/http.rs))

HTTP is just text over a TCP connection. We:

1. Open a connection: `TcpStream::connect(("example.org", 80))`. This also does the DNS lookup.
2. Write the request:
   ```
   GET / HTTP/1.0
   Host: example.org
   User-Agent: bakunawa/0.1

   ```
   Lines end with `\r\n`, and an empty line ends the headers.
3. Read everything with `read_to_end` until the server hangs up. (With HTTP/1.0 the
   server closes the connection after one response, which keeps this simple.)
4. Split the reply on the first `\r\n\r\n`: before is the status line + headers,
   after is the body (the HTML).

**Rust concepts:**

- **Traits `Read` and `Write`** — `TcpStream` implements them, which is why
  `write_all` and `read_to_end` exist on it. You must `use std::io::{Read, Write}`
  to call those methods.
- **`Vec<u8>`** — the network sends *bytes*, not text. `String::from_utf8_lossy`
  converts bytes to text.
- **`HashMap`** — headers are stored as `name → value`, with lowercase names.
- **`Box<dyn Error>`** — "any kind of error". Handy when one function can fail
  in many ways (network errors, parse errors, our own messages).

## Step 3 — "Rendering" ([html.rs](../src/html.rs))

We loop over every character and keep a tiny bit of **state**:

- `in_tag`: are we between `<` and `>`? If so, don't print.
- `skipping`: are we inside `<script>` or `<style>`? Their contents are code, so skip them too.

This is a **state machine** — the same idea (with many more states) powers the
real HTML tokenizer you'll write in Phase 5.

`tidy` then trims lines and collapses blank lines so the output is readable.

## Step 4 — Wiring it together ([main.rs](../src/main.rs))

- `mod url;` etc. include the other files as **modules**.
- `env::args()` reads the command line. No argument → we open `http://example.org/`.
- `run` returns a `Result` so it can use `?`; `main` prints the error and exits with code 1.
- Status info goes to **stderr** (`eprintln!`) and page text to **stdout** (`print!`),
  so `cargo run -- http://example.org/ > page.txt` saves just the text.

## Step 5 — Tests

Each module ends with a `#[cfg(test)] mod tests` block. Run them all:

```powershell
cargo test
```

Tests are the fastest way to check an idea: write a small input, assert on the output.

## Known limitations (on purpose)

| Limitation | Fixed in |
|------------|----------|
| No `https://` — most real sites fail | Phase 2 |
| Redirects (`301`, `302`) aren't followed | Phase 2 |
| `&amp;`, `&lt;` etc. are printed literally | Phase 2 |
| A `>` inside an attribute value (`title="a>b"`) confuses the tag stripper | Phase 5 |
| `<title>` text is printed as if it were page content | Phase 5 |
| It's a terminal program, not a window | Phase 3 |

## Exercises

Try these before moving on — they're small and each teaches something.

1. **Entities:** make `extract_text` replace `&lt;` with `<`, `&gt;` with `>` and `&amp;` with `&`. Add a test. *(Hint: `str::replace`, but think about the order of replacements.)*
2. **Show the headers:** add a `--headers` flag that prints every response header. *(Hint: iterate a `HashMap` with `for (name, value) in &response.headers`.)*
3. **view-source:** support `view-source:http://example.org/` — print the raw HTML instead of the text.
4. **Status errors:** if the status isn't `200`, print a friendly message. What does `http://example.org/nope` return?
5. **Local server:** run `python -m http.server 8000` inside `pages/`, then `cargo run -- http://localhost:8000/hello.html`. Now you've tested custom ports.

When you're ready, move on to **Phase 2 — Real-world networking** in the [roadmap](ROADMAP.md).
