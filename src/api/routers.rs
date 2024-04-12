// src/api/routers.rs

// This file is part of the API module of a Rust web application using the warp web framework. Its primary purpose is to define the HTTP routes
// for the application's API, specifically routes related to operations on 'persons'. It uses warp's powerful combinators to succinctly define
// how requests to these routes are handled.

use super::handlers; // Imports the handlers module from the parent module. This module contains the logic for handling requests to each route.
use warp::Filter; // Imports the Filter trait from the warp crate, which is essential for creating request routes in warp.

// Defines a function that creates and returns a warp route for adding a person.
// This route listens for POST requests to the "/add_person" URL path.
pub fn add_person_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::post() // Specifies that this route is for POST requests.
        .and(warp::path("add_person")) // Specifies the URL path that this route will match.
        .and(warp::body::json()) // Specifies that the request body should be parsed as JSON, expecting the structure of a 'Person'.
        .and_then(handlers::handle_add_person) // Specifies the handler function to be called when a request to this route is received.
}

// Defines a function that creates and returns a warp route for retrieving persons.
// This route listens for GET requests to the "/get_persons" URL path.
pub fn get_persons_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get() // Specifies that this route is for GET requests.
        .and(warp::path("get_persons")) // Specifies the URL path that this route will match.
        .and_then(handlers::handle_get_persons) // Specifies the handler function to be called when a request to this route is received.
}
