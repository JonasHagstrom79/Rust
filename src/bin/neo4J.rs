// Import everything from the neo4rs crate, which is likely used for Neo4j database interactions.
use neo4rs::*;
// Import the `env` module from the standard library for environment variable access.
use std::env;
// Import the StreamExt trait to enable additional methods on Stream types, likely for asynchronous stream processing.
use tokio_stream::StreamExt;

// This attribute marks the main function to be run in an asynchronous context provided by Tokio.
#[tokio::main]
async fn main() {
    // Attempt to load environment variables from a `.env` file, ignoring any errors if the file does not exist.
    dotenv::dotenv().ok();

    // Attempt to retrieve the value of the environment variable "MY_SECRET_KEY".
    // If it's not set, the program will panic with the message "Expected a secret key".
    let my_secret_key = std::env::var("MY_SECRET_KEY").expect("Expected a secret key");

    // Match on the result of attempting to retrieve "MY_SECRET_KEY" again, for demonstration.
    // If the variable is found, print its value. If not, print an error message.
    match env::var("MY_SECRET_KEY") {
        Ok(value) => println!("Värdet av MY_SECRET_KEY är: {}", value),
        Err(e) => println!("Kunde inte läsa MY_SECRET_KEY: {}", e),
    }
}
