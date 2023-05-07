use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod helpers;
// use helpers::meta::{get_data, get_paths, write_file};

#[tokio::main]
async fn main() {
    //    write_file(get_data(
    //        get_paths("static/music"),
    //        get_paths("static/covers"),
    //    ));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = Router::new()
        .route("/tracks", get(get_tracks))
        .route("/tracks/:id", get(get_track))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/add_favourite", post(add_favourite))
        .route("/delete_favourite", delete(delete_favourite))
        .route("/get_favourite", get(get_favourite));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_tracks() {}
async fn get_track() {}
async fn register() {}
async fn login() {}
async fn logout() {}
async fn add_favourite() {}
async fn delete_favourite() {}
async fn get_favourite() {}
