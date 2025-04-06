 use std::io;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;

use std::io::Write;


fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        let ollama = Ollama::new("http://localhost".to_string(), 11434);
        let model = String::from("llama3.2:1b");

        // Ask for user input
        print!("Enter your prompt: ");
        io::stdout().flush().unwrap(); // Flush output to ensure prompt appears before input

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).unwrap(); // Read input from user
        let prompt = prompt.trim().to_string(); // Trim to remove newline

        let request = GenerationRequest::new(model, prompt);
        
        let result = ollama.generate(request).await;
        match result {
            Ok(response) => println!("\nOllama's Response:\n{}", response.response),
            Err(error) => eprintln!("Error: {}", error),
        }
    });
}

