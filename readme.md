- See https://www.youtube.com/watch?v=BU1LYFkpJuk
- Clippy in VSCode settings
- Cargo watch for live recompiling
```bash
cargo add cargo-watch
```
```bash
cargo watch -q -c -w src/ -x 'run -q'
```
- See the CI step
- Interesting crates
    - `rayon`: for concurrency/parallel programming
    - `serde`/`serde_json`: serializing and deserializing data types
    - `thiserror`/`anyhow`: for error handling
    - `tokio`: for asynchronous programming
    - `blessed.rs`: good curated source of crates

-----

- Also, see the following videos for inspiration: https://www.youtube.com/watch?v=JOgQMjpGum0

# Testing

- Running tests with output
```bash
cargo test -- --nocapture
```