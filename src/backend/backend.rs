// src/backend/backend.rs

// This file is part of the backend module of a Rust web application. It uses the warp web framework to define and serve HTTP routes.
// The purpose of this file is to set up and start the web server, defining how the application responds to various HTTP requests.

use warp::Filter; // Imports the Filter trait from the warp crate, which is used to compose web server routes.

mod handlers; // Declares the handlers module, which is expected to contain functions that handle incoming HTTP requests.
mod routers; // Declares the routers module, which is expected to define the mapping of URLs to their corresponding handler functions.

// This is an asynchronous function named `run` that, when called, starts the web server.
// It does not return a value and is intended to be run as part of the application's main entry point.
pub async fn run() {
    // Calls a function from the routers module to get the route for adding a person.
    // This route is defined elsewhere, likely associating a specific URL path and HTTP method with a handler function.
    let add_person = routers::add_person_route();

    // Similarly, calls another function from the routers module to get the route for retrieving persons.
    // This also associates a URL path and HTTP method with a handler, but for fetching data instead.
    let get_persons = routers::get_persons_route();

    // Combines the two routes using the `.or` combinator. This allows the server to handle requests for both adding and
    // retrieving persons, routing each request to the appropriate handler based on the request's URL and method.
    let routes = add_person.or(get_persons);

    // Starts the warp server on localhost at port 3030, serving the combined routes.
    // The `run` method is an asynchronous operation that will keep running until the server is shut down.
    // It listens for incoming HTTP requests and dispatches them to the appropriate route handlers defined above.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
