// src/main.rs

// This file serves as the entry point for a Rust application. It's where the execution of the program begins.
// The structure of this file is typical for a Rust application that includes asynchronous operations,
// particularly for web servers or applications that perform IO-bound tasks.

mod backend; // This line declares the `backend` module. Rust modules are a way to organize code into namespaces,
             // allowing for better modularity and code reuse. The `backend` module is expected to be defined in a file
             // named `backend.rs` in the same directory as this `main.rs` file, or in a `backend` directory with a `mod.rs` file.
mod error;
use crate::backend::DbConnection; // This line imports the `DbConnection` struct from the `backend` module.

#[tokio::main] // This attribute macro indicates that the async main function is the entry point of the program,
               // and it uses Tokio as the runtime for executing asynchronous tasks. Tokio is a popular asynchronous
               // runtime in the Rust ecosystem, designed for building fast and scalable network applications.

async fn main() {
    env_logger::init();
    // The `main` function is marked as `async`, which means it can perform asynchronous operations.
    // This is the starting point of the application's execution.
    let db_connection = DbConnection::new().await;
    let first_name = "Johan";
    match db_connection.get_person_by_first_name(first_name).await {
        Ok(Some(person)) => println!("Found person: {:?}", person),
        Ok(None) => println!("No person with the name '{}' exists.", first_name),
        Err(e) => println!("Error occurred: {}", e),
    }
}
