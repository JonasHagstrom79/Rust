use super::handlers;
use warp::Filter;

pub fn add_person_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::post()
        .and(warp::path("add_person"))
        .and(warp::body::json())
        .and_then(handlers::handle_add_person)
}

pub fn get_persons_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("get_persons"))
        .and_then(handlers::handle_get_persons)
}
