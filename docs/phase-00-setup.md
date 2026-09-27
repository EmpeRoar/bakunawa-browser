# Phase 0 — Setup

Goal: get Rust installed and understand what's in this repository.

## 1. Install Rust (Windows)

1. Go to https://rustup.rs and download `rustup-init.exe`.
2. Run it and accept the defaults. If it asks for the **Visual Studio C++ Build Tools**, let it install them — Rust needs the Microsoft linker on Windows.
3. Close and reopen your terminal (and VS Code) so the new `PATH` is picked up.
4. Check it worked:

   ```powershell
   rustc --version
   cargo --version
   ```

5. In VS Code, install the **rust-analyzer** extension. It gives you autocomplete, inline type hints and error squiggles — very helpful while learning.

## 2. What the tools are

| Tool | What it does |
|------|--------------|
| `rustup` | Installs and updates Rust itself. `rustup update` gets the newest version. |
| `rustc` | The compiler. You rarely call it directly. |
| `cargo` | Build tool + package manager. You'll use it for everything. |

Everyday `cargo` commands:

```powershell
cargo build          # compile (debug mode) into target/debug/
cargo run            # compile and run
cargo run -- ARGS    # everything after -- is passed to YOUR program
cargo test           # run all #[test] functions
cargo check          # fast "does it compile?" without producing a binary
cargo fmt            # auto-format the code
cargo clippy         # extra lint warnings with helpful suggestions
```

## 3. Project layout

```
bakunawa-browser/
├── Cargo.toml        ← project name, version, dependencies ("crates")
├── Cargo.lock        ← exact dependency versions (created by cargo; commit it)
├── src/
│   ├── main.rs       ← program entry point: fn main()
│   ├── url.rs        ← turns "http://host/path" into a Url struct
│   ├── http.rs       ← talks to web servers
│   └── html.rs       ← turns HTML into plain text
├── pages/
│   └── hello.html    ← a local test page (works without internet)
├── docs/             ← you are here
└── target/           ← build output (ignored by git)
```

## 4. First run

```powershell
cargo run -- file://pages/hello.html
```

The first build takes a moment. You should see:

```
Hello, Bakunawa

Hello from a local file!
If you can read this in your terminal, Phase 1 works.
The bold tags are gone, and the CSS above was skipped.
```

Then run the tests:

```powershell
cargo test
```

If both work, your setup is complete. Continue to [Phase 1](phase-01-fetch-and-print.md).

## Troubleshooting

- **`cargo` is not recognized** — reopen the terminal; if still failing, make sure `%USERPROFILE%\.cargo\bin` is in your `PATH`.
- **`link.exe not found`** — the C++ Build Tools are missing. Install "Desktop development with C++" from the Visual Studio Installer.
