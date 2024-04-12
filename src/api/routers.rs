// src/api/routers.rs

// This file is part of the API module of a Rust web application using the warp web framework. Its primary purpose is to define the HTTP routes
// for the application's API, specifically routes related to operations on 'persons'. It uses warp's powerful combinators to succinctly define
// how requests to these routes are handled.

use super::controller; // Imports the handlers module from the parent module. This module contains the logic for handling requests to each route.
use crate::models::Person; // Add this line to import the `Person` struct
use serde::Deserialize; // Add this line at the beginning of your file
use warp::Filter; // Add this line at the beginning of your file to import the `Filter` trait
                  // Imports the Filter trait from the warp crate, which is essential for creating request routes in warp.
use crate::db::DbConnection; // Add this line to import DbConnection

// Define a struct that matches your expected query parameters
#[derive(Deserialize)]
struct YourType {
    // Define fields according to the expected query parameters
    field_name: String, // Example field, replace with actual query parameter names and types
}

// Defines a function that creates and returns a warp route for adding a person.
// This route listens for POST requests to the "/add_person" URL path.
/* pub fn add_person_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::post()
        .and(warp::path("add_person"))
        .and(warp::body::json::<Person>())
        .and_then(|person: Person| async move {
            DbConnection::new().await // Convert any Err variant to a warp rejection
                                      // You might want to use `person` inside this block to interact with the database
        })
} */

// Defines a function that creates and returns a warp route for retrieving persons.
// This route listens for GET requests to the "/get_persons" URL path.
pub fn get_persons_route(
    db_connection: DbConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("persons"))
        .and(warp::any().map(move || db_connection.clone())) // Clone the DbConnection
        .and(warp::query::<YourType>()) // Extract query parameters into YourType
        .and_then(move |db_connection: DbConnection, query_params: YourType| {
            let db_clone = db_connection.clone(); // Clone the DbConnection
            controller::handle_get_person_by_first_name(db_clone, query_params.field_name)
        })
}
