use neo4rs::*;
use std::env;
/* use warp::Filter; */

/* mod handlers;
mod routers; */

#[tokio::main]
async fn main() {
    // Ladda variabler från .env-filen
    dotenv::dotenv().ok();

    // Läs in användarnamn och lösenord från miljövariabler
    let username = env::var("NEO4J_USERNAME").unwrap_or("Not Found".to_string());
    let password = env::var("NEO4J_PW").unwrap_or("Not Found".to_string());

    // Skapa en Graph-anslutning med de inlästa värdena
    let graph = Graph::new("bolt://localhost:7687", &username, &password)
        .await
        .expect("Kunde inte ansluta till databasen");

    // Din Cypher-fråga
    let query = r#"
    MATCH (n)
    OPTIONAL MATCH (n)-[r]->(m)
    RETURN n, r, m
    "#;

    // Kör frågan
    let mut result = graph
        .execute(query.into())
        .await
        .expect("Kunde inte köra frågan");

    // Iterera över resultaten
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

    /* let add_person = routers::add_person_route();
    let get_persons = routers::get_persons_route(); */

    /* let routes = add_person.or(get_persons); */

    /*  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await; */
}
