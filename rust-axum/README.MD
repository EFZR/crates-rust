# 🦀 Rust Axum Production code 🚀

## Done

- Create the basics of the Axum Server
- Tracing for logs with the `tracing` and `tracing-subscriber` crates
- Set configuration for the server with the `.cargo/config.toml` file for environment variables and `config.rs` file for the configuration
- Crate a DB test with the `sqlx` crate to check the connection to the database in the `_dev_utils` module
- Create model layer with the store in the `models` module with some unit tests
- Create the base CRUD operations with Generics in the `base` module using the `DbBmc` struct to know the enitity to work with and the `sqlb` crate to build the queries and manipulate the fields with each query.
- Create the user login, encryption and validation with the crates: `Hmac`, `sha2`, `base64-url`

## Commands

to run in the main server

```shell
cargo watch -q -c -w src/ -w .cargo/ -x run
```

to run the deploy some requests to the server

```shell
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

to run tests

```shell
cargo watch -q -c -x test
```

## Things to fix

- Dockerize the application
