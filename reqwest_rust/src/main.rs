
use reqwest::Client;
use serde_json::json;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let client = Client::new();

    rt.block_on(async {
        // Perform a GET request
        match client.get("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(text) => println!("GET Response:\n{}", text),
                Err(e) => eprintln!("Error reading GET response: {}", e),
            },
            Err(e) => eprintln!("Error sending GET request: {}", e),
        }

        // Perform a POST request
        match client.post("https://jsonplaceholder.typicode.com/posts")
            .json(&json!({
                "title": "Rust",
                "body": "Learning reqwest",
                "userId": 1
            }))
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(text) => println!("POST Response:\n{}", text),
                Err(e) => eprintln!("Error reading POST response: {}", e),
            },
            Err(e) => eprintln!("Error sending POST request: {}", e),
        }
    });
}

