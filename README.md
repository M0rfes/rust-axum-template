## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo test quick_dev -- --nocapture
```
