# 📝 TODOs Example with `sqlx` 🚀

This mini program is a simple yet powerful command line application that allows you to manage your todos. It's a great way to understand the basic concepts of `sqlx`.

The project uses several Rust libraries:

- `sqlx` 🗄️ for interacting with the database
- `anyhow` ❗ for handling errors
- `structopt` 📜 for parsing command line arguments
- `dotenv` 🌍 to load environment variables from a `.env` file
- `futures` 🔄 to handle asynchronous code
- `tokio` ⏱️ as the runtime

## 🛠️ Setup

1. Create the database 🗄️:

```bash
sqlx db create
```

2. Run the migrations 🚀:

```bash
sqlx migrate run
```

## 🎯 Usage

1. Add a todo 📝:

```bash
cargo run -- add "todo example: Buy milk"
```

2. List all todos 📋:

```bash
cargo run
```

3. Complete a todo ✅:

```bash
cargo run -- complete 1
```
