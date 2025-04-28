pub mod auth;
pub mod errors;
pub mod media;
pub mod routes;
pub mod state;

use crate::state::AppState;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Initialize the database connection
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(32)
        .min_connections(8)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    // Initialize the application state
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let media_root = env::var("MEDIA_ROOT").unwrap_or_else(|_| "/media".to_string());
    let allow_register =
        env::var("ALLOW_REGISTER").unwrap_or_else(|_| "false".to_string()) == "true";
    let allow_anonymous =
        env::var("ALLOW_ANONYMOUS").unwrap_or_else(|_| "false".to_string()) == "true";
    let allow_peer_to_peer =
        env::var("ALLOW_PEER_TO_PEER").unwrap_or_else(|_| "false".to_string()) == "true";
    let state = AppState {
        conn: db,
        jwt_secret,
        media_root,
        allow_register,
        allow_anonymous,
        allow_peer_to_peer,
    };

    // Initialize the routes
    let auth_routes = routes::auth_routes(state.clone());

    // Create the main router with the /v1 prefix for the auth routes
    let app = Router::new()
        .route("/", get(index))
        .nest("/v1/auth", auth_routes);

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Listen on 0.0.0.0:3000

    println!("Listening on {}", addr);

    //Use the axum::serve with a listener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to Smartinis Media API</h1><p>API Version: v1</p>")
}
