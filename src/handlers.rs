use super::db;
use super::models::Person;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn handle_add_person(person: Person) -> Result<impl Reply, Rejection> {
    db::add_person(person).await.map_err(|_e| warp::reject())?;
    Ok(StatusCode::CREATED)
}
