use sea_orm::DatabaseConnection;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub jwt_secret: String,
    pub media_root: String,
    pub allow_register: bool,
    pub allow_anonymous: bool,
    pub allow_peer_to_peer: bool,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret".to_string());
        let media_root = env::var("MEDIA_ROOT").unwrap_or_else(|_| "/media".to_string());
        let allow_register =
            env::var("ALLOW_REGISTER").unwrap_or_else(|_| "false".to_string()) == "true";
        let allow_anonymous =
            env::var("ALLOW_ANONYMOUS").unwrap_or_else(|_| "false".to_string()) == "true";
        let allow_peer_to_peer =
            env::var("ALLOW_PEER_TO_PEER").unwrap_or_else(|_| "false".to_string()) == "true";

        AppState {
            conn,
            jwt_secret,
            media_root,
            allow_register,
            allow_anonymous,
            allow_peer_to_peer,
        }
    }
}
