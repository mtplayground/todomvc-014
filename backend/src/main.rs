mod db;
mod handlers;
mod routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = db::init_pool().await.expect("Failed to initialize database");
    let app = routes::create_router(pool);

    let addr = "0.0.0.0:8080";
    println!("Server listening on {addr}");
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
