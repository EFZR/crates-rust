use serde::{Deserialize, Serialize};

// Define a struct to represent a Todo item
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

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

    Ok(())
}