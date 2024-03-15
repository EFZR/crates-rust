# Commands

to run in the main server

```shell
cargo watch -q -c -w src/ -w .cargo/ -x run
```

to run the tests developements

```shell
cargo watch -q -c -w examples/ -x "run --example quick_dev.rs"
```
