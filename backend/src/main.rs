mod db;

#[tokio::main]
async fn main() {
    let pool = db::init_pool().await.expect("Failed to initialize database");
    println!("Database initialized successfully");

    // Server setup will be added in future issues
    drop(pool);
}
