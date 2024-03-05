# Using Serde for Serailization and Deserialization in Rust

This example demonstrates how to leverage the `serde` library to serialize and deserialize a struct in Rust. Additionally, we'll utilize the `tokio` library for handling asynchronous code and `reqwest` for making HTTP requests to a server.

## The `Struct`: Todo

Rust is a statically typed language, and understanding how to work with structs is crucial. Let's use a struct to represent a `Todo` task:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
```

The `#[derive(Serialize, Deserialize, Debug)]` attributes tell the `serde` library to handle encoding and decoding for this struct.

Assuming we have a JSON like this:

```json
{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}
```

We can `deserialize` it into a `Todo` struct:

```rust
let todo: Todo = serde_json::from_str(r#"
{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}
"#).unwrap();
```

Or `serialize` a `Todo` struct into JSON:

```rust
let todo = Todo {
    user_id: 1,
    id: Some(1),
    title: "delectus aut autem".to_string(),
    completed: false,
};

let json = serde_json::to_string(&todo).unwrap();
```

## Making a `GET` Request to a Server

In this example, we use the `reqwest` crate to make a GET request to a server and `tokio` to handle asynchronous code:

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Create a new HTTP client and send a GET request to fetch Todo items for a specific user
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    // Print the fetched Todo items
    println!("{:#?}", todos);

    Ok(())
}
```

The result is stored in a `Vec<Todo>` representing an array of `Todo` items.

## Making a `POST` Request to a Server to Send JSON

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
 // Define a new Todo item
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: String::from("Learning how to use serde"),
        completed: true,
    };

    // Send a POST request to create a new Todo item and parse the response into a Todo struct
    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    // Print the created Todo item
    println!("{:#?}", new_todo);

    Ok(())
}
```

Here, we create a new `Todo` item and send it to the server using the POST method.

## Handling Arbitrary JSON with `serde_json::Value`

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Define a new Todo item as a JSON value
    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Learning how to use serde".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    // Print the created Todo item
    println!("{:#?}", new_todo);
}
```

In this code, we use the `serde_json::Value` to define a new JSON value without the need to define a struct. This is useful when we don't have a struct to represent the JSON. We also receive the response as a `serde_json::Value` and then print the result.
