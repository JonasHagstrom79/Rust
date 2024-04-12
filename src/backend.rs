use warp::Filter;

mod handlers;
mod routers;

#[tokio::main]
async fn main() {
    // Importera dina routes
    let add_person = routers::add_person_route();
    let get_persons = routers::get_persons_route();

    // Kombinera dina routes
    let routes = add_person.or(get_persons);

    // Starta servern med de kombinerade routes på port 3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
