use warp::Filter;

mod handlers;
mod routers;

// Removed #[tokio::main] from here
pub async fn run() {
    let add_person = routers::add_person_route();
    let get_persons = routers::get_persons_route();

    let routes = add_person.or(get_persons);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
