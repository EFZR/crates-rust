# 🦀 Rust Axum Production code 🚀

## Done

- Create the basics of the Axum Server
- Tracing for logs with the `tracing` and `tracing-subscriber` crates
- Set configuration for the server with the `.cargo/config.toml` file for environment variables and `config.rs` file for the configuration
- Crate a DB test with the `sqlx` crate to check the connection to the database in the `_dev_utils` module

## Commands

to run in the main server

```shell
cargo watch -q -c -w src/ -w .cargo/ -x run
```

to run the tests developements

```shell
cargo watch -q -c -w examples/ -x "run --example quick_dev.rs"
```

## Things to Learn

- Dockerize the application
