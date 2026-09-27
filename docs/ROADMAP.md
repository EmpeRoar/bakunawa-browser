# Bakunawa Browser — Roadmap

This is the plan for building a web browser from scratch in Rust, one small
phase at a time. Each phase produces something that **runs**, so you always
have a working browser — it just gets smarter every phase.

> **Status:** Phase 0 and Phase 1 are written. Everything after that is the plan.

## How a browser works (the big picture)

Every browser, from Chrome to this one, runs the same pipeline:

```
 URL ──▶ Networking ──▶ HTML ──▶ Parsing ──▶ DOM tree
                                               │
                     CSS ──▶ Styling ◀─────────┘
                                │
                             Layout  (where does every box go?)
                                │
                              Paint  (draw pixels on the screen)
                                │
                     User input (click, scroll, type) ──▶ back to the top
```

Each phase below builds one piece of that pipeline. Early phases do a piece
crudely; later phases come back and do it properly.

## Phases at a glance

| # | Phase | You build | Rust concepts you learn | Status |
|---|-------|-----------|-------------------------|--------|
| 0 | [Setup](phase-00-setup.md) | Toolchain, first `cargo run` | cargo, crates, `main` | ✅ Ready |
| 1 | [Fetch & print](phase-01-fetch-and-print.md) | Download a page, print its text | `String`/`&str`, `Result`, `?`, modules, structs, enums, tests | ✅ Done |
| 2 | Real-world networking | HTTPS, redirects, HTML entities | crates, traits, `Box<dyn Error>`, loops | ⬜ Next |
| 3 | A window | Open a window, draw text into it, scroll | event loops, closures, ownership across callbacks | ⬜ |
| 4 | Text layout | Word wrapping, lines, font sizes, bold/italic | `Vec`, iterators, lifetimes (gently) | ⬜ |
| 5 | HTML parser | Turn HTML into a DOM tree | recursive enums, `Box`, `Rc`/`RefCell` or arenas | ⬜ |
| 6 | Block & inline layout | Boxes inside boxes, a layout tree | trait objects, tree traversal | ⬜ |
| 7 | CSS | Parse CSS, match selectors, cascade styles | parsers, pattern matching, `HashMap` | ⬜ |
| 8 | Links & navigation | Click links, address bar, back/forward, tabs | shared state, application architecture | ⬜ |
| 9 | Forms & input | Text boxes, buttons, submitting forms (POST) | mutable state, event handling | ⬜ |
| 10 | JavaScript (stretch) | Run scripts via an embedded JS engine | FFI-style boundaries, callbacks | ⬜ |
| 11 | Polish & performance | Caching, threads, faster painting | threads, `Arc`, profiling | ⬜ |

## Phase details

### Phase 0 — Setup
Install Rust, learn what `cargo` does, understand the project layout.
→ Full guide: [phase-00-setup.md](phase-00-setup.md)

### Phase 1 — Fetch & print ✅
A command-line program: give it a URL, it connects to the server with a raw
TCP socket, speaks HTTP/1.0 by hand, strips the HTML tags and prints the text.
Zero external crates — only the standard library.
→ Full guide: [phase-01-fetch-and-print.md](phase-01-fetch-and-print.md)

**Done when:** `cargo run -- http://example.org/` prints "Example Domain".

### Phase 2 — Real-world networking
Most websites use HTTPS, redirect you around, and write `&amp;` instead of `&`.

- Add `https://` using the [`rustls`](https://crates.io/crates/rustls) crate (writing TLS yourself is a research project, not a beginner step).
- Follow redirects (status `301`/`302` + `Location` header), with a limit so you don't loop forever.
- Decode HTML entities: `&lt;` `&gt;` `&amp;` `&quot;` `&nbsp;`.
- Handle `view-source:` URLs (print raw HTML instead of text).
- Optional: switch to HTTP/1.1 and keep connections open (`Content-Length`, chunked encoding).

**Done when:** `cargo run -- https://www.rust-lang.org/` prints readable text.

### Phase 3 — A window
Leave the terminal. Open a real window and draw text in it.

- [`winit`](https://crates.io/crates/winit) opens a window and gives you events (resize, keyboard, mouse).
- [`softbuffer`](https://crates.io/crates/softbuffer) gives you a buffer of pixels to paint into.
- [`fontdue`](https://crates.io/crates/fontdue) turns a font file into glyph bitmaps you copy into that buffer.
- Draw the page text character by character, left to right; wrap at the window edge.
- Scroll with the arrow keys / mouse wheel.

**Done when:** a window shows a page's text and you can scroll it.

### Phase 4 — Text layout
Turn "a pile of characters" into proper paragraphs.

- Lay out *words*, not characters; wrap when a word doesn't fit.
- Treat `<b>`, `<i>`, `<br>`, `<p>`, `<h1>` specially (bold, italic, line breaks, bigger text).
- Line height and baseline alignment when sizes are mixed.
- Split the code into a **display list**: layout produces "draw this text at x,y", painting just follows the list.

**Done when:** a page with headings and paragraphs looks like a document.

### Phase 5 — HTML parser
Replace the "strip tags" trick with a real parser.

- **Tokenizer:** turn HTML text into tokens (start tag, end tag, text, attributes).
- **Tree builder:** build a tree of nodes (the **DOM**) — elements with children.
- Handle self-closing tags (`<br>`, `<img>`) and forgiving cases (missing `</p>`, implicit `<html>`/`<body>`).
- Print the tree to check it (`cargo run -- --tree URL`).

**Done when:** you can print the DOM tree of example.org with correct nesting.

### Phase 6 — Block & inline layout
Build a **layout tree** from the DOM.

- Block elements (`<div>`, `<p>`, `<h1>`) stack vertically.
- Inline elements (`<b>`, `<a>`, text) flow horizontally inside a line.
- Every box gets `x`, `y`, `width`, `height`.
- Paint backgrounds (e.g. gray `<pre>` blocks) from the layout tree.

**Done when:** nested `<div>`s and paragraphs render in the right places.

### Phase 7 — CSS
Let the page decide how it looks.

- Parse CSS text into rules (`selector { property: value; }`).
- Selectors: tag (`p`), class (`.note`), id (`#main`), descendant (`div p`).
- The **cascade**: browser defaults → `<style>` sheets → `style="..."` attributes; specificity decides ties.
- Inheritance (`color`, `font-size` pass from parent to child).
- Support a handful of properties: `color`, `background-color`, `font-size`, `font-weight`, `margin`, `padding`, `display`.

**Done when:** changing a `<style>` block visibly changes your rendering.

### Phase 8 — Links & navigation
Make it feel like a browser.

- Click detection: which layout box is under the mouse?
- Follow `<a href>`; resolve relative URLs (`/about`, `../x.html`).
- Browser chrome: an address bar you can type into, back/forward buttons.
- Tabs (a list of pages, one visible).

**Done when:** you can click from one page to another and go back.

### Phase 9 — Forms & input
- Render `<input>` and `<button>`, focus and type into inputs.
- Submit a `<form>` as a `POST` request with URL-encoded fields.
- Write a tiny local test server (Rust, std only) to receive submissions.

**Done when:** you can submit a form to your own server and see the response.

### Phase 10 — JavaScript (stretch goal)
- Embed an existing engine such as [`boa_engine`](https://crates.io/crates/boa_engine) (a JS engine written in Rust).
- Run `<script>` tags; expose a tiny DOM API (`document.querySelectorAll`, `innerHTML`, `addEventListener`).
- Re-run style + layout when JS changes the DOM.

**Done when:** a script can change text on the page when you click a button.

### Phase 11 — Polish & performance
- Cache responses; don't re-layout when only scrolling.
- Move networking off the main thread so the window never freezes.
- Measure first (`cargo build --release`, timing, a profiler), then optimize.

## Suggested way to work through each phase

1. Read the phase guide in `docs/`.
2. Write the code in small steps; run `cargo run` and `cargo test` often.
3. When it works, commit: `git commit -m "Phase N: ..."`.
4. Do the "Exercises" section of the guide — that's where most of the learning happens.

## Learning resources

- **The Rust Book** (free): https://doc.rust-lang.org/book/ — read chapters 1–9 alongside Phases 0–2.
- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/
- **Web Browser Engineering** (free book, Python): https://browser.engineering/ — this roadmap follows a similar order; great for the "why".
- **Let's build a browser engine!** (Rust blog series by Matt Brubeck): https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html
- **HTML spec — parsing:** https://html.spec.whatwg.org/multipage/parsing.html (reference, not light reading)
