use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use warp::Filter;

mod db;
mod error;
mod models;
mod routers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let routes =
        routers::patient::patient_routes(pool.clone()).with(warp::cors().allow_any_origin());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on {}", addr);
    warp::serve(routes).run(addr).await;
}
