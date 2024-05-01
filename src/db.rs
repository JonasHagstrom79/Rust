use crate::error::AppError;
use crate::models::Person;
use anyhow::{self, Result};
use neo4rs::{Graph, Node, Relation};
use std::env; // Added import for Graph
use std::fmt;
use std::sync::Arc; // Added for Arc support

// Define a new struct to act as your database connection
pub struct DbConnection {
    graph: Arc<Graph>,
}

impl DbConnection {
    // Method to create a new DbConnection
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let username = env::var("NEO4J_USERNAME").unwrap_or_else(|_| "Not Found".to_string());
        let password = env::var("NEO4J_PW").unwrap_or_else(|_| "Not Found".to_string());

        let graph = Graph::new("bolt://localhost:7687", &username, &password)
            .await
            .expect("Could not connect to the database");

        DbConnection {
            graph: Arc::new(graph),
        }
    }

    // You can add more methods here to interact with the database using the `graph` field
    pub async fn get_person_by_first_name(
        &self,
        first_name: &str,
    ) -> Result<Option<Node>, AppError> {
        let query = neo4rs::query("MATCH (p:Person {firstName: $firstName}) RETURN p")
            .param("firstName", first_name);

        let mut result = self.graph.execute(query).await.map_err(AppError::from)?;
        if let Some(row) = result.next().await.map_err(AppError::from)? {
            let person_result = row
                .get("p")
                .map_err(|e| neo4rs::Error::new(format!("Failed to get person: {}", e)))
                .map_err(AppError::from);
            match person_result {
                Ok(person) => Ok(Some(person)),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }
}

impl Clone for DbConnection {
    fn clone(&self) -> Self {
        DbConnection {
            graph: self.graph.clone(),
        }
    }
}

#[derive(Debug)]
struct MyErrorType {
    message: String,
}

impl fmt::Display for MyErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyErrorType {}

impl warp::reject::Reject for MyErrorType {}

// Function to convert MyErrorType into warp::Rejection
fn to_rejection(error: MyErrorType) -> warp::Rejection {
    warp::reject::custom(error)
}

pub async fn create_graph() -> Graph {
    dotenv::dotenv().ok();
    let username = env::var("NEO4J_USERNAME").unwrap_or("Not Found".to_string());
    let password = env::var("NEO4J_PW").unwrap_or("Not Found".to_string());

    Graph::new("bolt://localhost:7687", &username, &password)
        .await
        .expect("Kunde inte ansluta till databasen")
}

pub async fn execute_query(graph: &Graph) {
    let query = r#"
    MATCH (n)
    OPTIONAL MATCH (n)-[r]->(m)
    RETURN n, r, m
    "#;

    let mut result = graph
        .execute(query.into())
        .await
        .expect("Kunde inte köra frågan");

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

pub async fn add_person(graph: &Graph, person: Person) -> Result<(), neo4rs::Error> {
    let query =
        neo4rs::query("CREATE (p:Person {id: $id, firstName: $firstName, lastName: $lastName})")
            .param("id", person.id)
            .param("firstName", person.firstName)
            .param("lastName", person.lastName);

    graph.run(query).await?;

    Ok(())
}

// Asynchronous method to get a person by their first name from the database
pub async fn get_person_by_first_name(graph: &Graph, first_name: &str) {
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
