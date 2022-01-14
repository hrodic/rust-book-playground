# Rust book

## Quick start

Download
```
git clone https://github.com/hrodic/rust-book-playground.git
cd rust-book-playground
```

Run (use bin names from Cargo.toml file)
```
cargo run --bin=hello_world
cargo run --bin=guessing_game
cargo run --bin=variables
...
```

## Testing

Docs

* https://doc.rust-lang.org/book/ch11-03-test-organization.html
* https://doc.rust-lang.org/book/ch11-02-running-tests.html

Unit
```
cargo test --bin=writing_tests -- --test-threads=1
```

Integration (only possible for libraries lib.rs - aka public API)
```
cargo test --test integration_test -- --show-output
```




