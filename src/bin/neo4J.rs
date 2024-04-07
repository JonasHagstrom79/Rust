// Import necessary crates and modules
mod db;
mod handlers;
mod models;
mod routes;
use models::Person;
use neo4rs::*;
use std::env;
use tokio_stream::StreamExt;

// Entry point for the async application
#[tokio::main]
async fn main() {
    // Load variables from the .env file
    dotenv::dotenv().ok();

    // Read username and password from environment variables
    let username = env::var("NEO4J_USERNAME").unwrap_or("Not Found".to_string());
    let password = env::var("NEO4J_PW").unwrap_or("Not Found".to_string());

    // Create a Graph connection using the read values
    let graph = Graph::new("bolt://localhost:7687", &username, &password)
        .await
        .expect("Could not connect to the database");

    // Run the query through a separate method
    run_query(&graph).await;

    // Example usage of the new method
    // let first_name = "Alice";
    // get_person_by_first_name(&graph, first_name).await;

    // Example usage of add_person function
    let id = uuid::Uuid::new_v4().to_string();
    let first_name = "John";
    let last_name = "Doe";
    add_person(&graph, &id, first_name, last_name)
        .await
        .unwrap();
}

use neo4rs::*;

async fn add_person(graph: &Graph, person: Person) -> Result<(), neo4rs::Error> {
    let query = query("CREATE (p:Person {id: $id, firstName: $firstName, lastName: $lastName})")
        .param("id", person.id)
        .param("firstName", person.first_name)
        .param("lastName", person.last_name);

    graph.run(query).await?;

    Ok(())
}

/* async fn add_person(
    graph: &Graph,
    id: &str,
    first_name: &str,
    last_name: &str,
) -> Result<(), neo4rs::Error> {
    let query = query("CREATE (p:Person {id: $id, firstName: $firstName, lastName: $lastName})")
        .param("id", id)
        .param("firstName", first_name)
        .param("lastName", last_name);

    graph.run(query).await?;

    Ok(())
} */

// Separate asynchronous method to run the Cypher query
async fn run_query(graph: &Graph) {
    // Your Cypher query
    let query = r#"
    MATCH (n)
    OPTIONAL MATCH (n)-[r]->(m)
    RETURN n, r, m
    "#;

    // Execute the query
    let mut result = graph
        .execute(query.into())
        .await
        .expect("Could not run the query");

    // Iterate over the results
    while let Ok(Some(row)) = result.next().await {
        let n: Node = row.get("n").unwrap();
        let r: Option<Relation> = row.get("r").unwrap();
        let m: Option<Node> = row.get("m").unwrap();

        println!("Node: {:?}", n);
        if let Some(relation) = r {
            println!("Relation: {:?}", relation);
            if let Some(target_node) = m {
                println!("Target Node: {:?}", target_node);
            }
        }
    }
}

// Asynchronous method to get a person by their first name from the database
async fn get_person_by_first_name(graph: &Graph, first_name: &str) {
    // Format the Cypher query to search for a person by first name
    let query = format!(
        r#"
        MATCH (p:Person {{firstName: '{}'}})
        RETURN p
        "#,
        first_name
    );

    // Execute the query
    let mut result = graph
        .execute(query.into())
        .await
        .expect("Failed to execute query");

    // Debug: Check how many results we're getting
    let mut found = false;

    // Iterate over the results
    while let Ok(Some(row)) = result.next().await {
        found = true;
        let person: Node = row.get("p").unwrap();
        println!("Found Person: {:?}", person);
        break; // If you only want the first match, ensure to break here
    }

    // If no person was found, print a message
    if !found {
        println!("No one with the name '{}' exists.", first_name);
    }
}
