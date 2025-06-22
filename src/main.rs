use axum::{
    Router,
    routing::{delete, get, post, put},
};

mod handlers {
    pub mod incidents;
}
use handlers::incidents::read_by_id;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get("Hello, world!"))
        .route("/incidents/{id}", get(read_by_id));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
