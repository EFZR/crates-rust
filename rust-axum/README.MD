# 🦀 Rust Axum Production code 🚀

## Done

- Create the basics of the Axum Server
- Tracing for logs with the `tracing` and `tracing-subscriber` crates
- Set configuration for the server with the `.cargo/config.toml` file for environment variables and `config.rs` file for the configuration
- Crate a DB test with the `sqlx` crate to check the connection to the database in the `_dev_utils` module
- Create model layer with the store in the `models` module with some unit tests
- Create the base CRUD operations with Generics in the `base` module using the `DbBmc` struct to know the enitity to work with and the `sqlb` crate to build the queries and manipulate the fields with each query.
- Create the user login, encryption and validation with the crates: `Hmac`, `sha2`, `base64-url`
- Create the token generation and validation with the `base64-url` and `time` crates, being able to store an identifier, expiration time and a signature and also adding the the token into the response header.
- Create the routes to the server to manipulte the *task* entity using the api architecture of **json rpc** with the `axum` and `serde_json` crates in the `rpc` module and the `rpc_task` module to handle the requests.

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

to generate a key of base64-url

```shell
cargo run --example gen_key
```

## Things to fix

- Dockerize the application
