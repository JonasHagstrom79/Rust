use crate::db::{self, DbConnection};
use crate::models::Person;
use neo4rs::Graph;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn handle_add_person(person: Person, graph: Graph) -> Result<impl Reply, Rejection> {
    db::add_person(&graph, person)
        .await
        .map_err(|_e| warp::reject())?;
    Ok(StatusCode::CREATED)
}

/* pub async fn handle_get_person_by_first_name (
    db_connection: DbConnection, // Ensure this parameter is correctly defined
    first_name: String,
) -> Result<impl Reply, Rejection> {
    // Assuming `get_graph_from_connection` is a function that converts a DbConnection to a Graph
    let graph = get_graph_from_connection(&db_connection).await; // This is a hypothetical function
    let persons = db::get_person_by_first_name(&graph, &first_name)
        .await
        .map_err(|_e| warp::reject())?;
    Ok(warp::reply::json(&persons))
} */

// Antag att detta är en funktion i din controller.rs eller någon annanstans där du hanterar din databasanslutning
pub async fn handle_get_person_by_first_name(
    db_connection: DbConnection,
    first_name: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let graph = convert_to_graph(&db_connection).await;
    let persons = db::get_person_by_first_name(&graph, &first_name).await;
    Ok(warp::reply::json(&persons))
}

pub async fn convert_to_graph(db_connection: &DbConnection) -> Graph {
    // Implementation to convert DbConnection to Graph
    // This is a placeholder. Replace it with actual logic.
    unimplemented!()
}
