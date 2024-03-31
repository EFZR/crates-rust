use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Getting index.html from .web-folder
    hc.do_get("/index.html").await?.print().await?;

    // Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome",
        }),
    );

    req_login.await?.print().await?;

    // Create Task
    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_task",
            "params": {
                "data": {
                    "title": "taskAAA"
                }
            }
        }),
    );

    req_create_task.await?.print().await?;

    // Update Task
    let req_update_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_task",
            "params": {
                "id": 1000,
                "data": {
                    "title": "taskBBB"
                }
            }
        }),
    );

    req_update_task.await?.print().await?;

    // Delete Task
    let req_delete_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_task",
            "params": {
                "id": 1001
            }
        }),
    );

    req_delete_task.await?.print().await?;

    // List Tasks
    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_tasks"
        }),
    );

    req_list_tasks.await?.print().await?;

    // Logoff
    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true,
        }),
    );

    req_logoff.await?.print().await?;

    Ok(())
}
