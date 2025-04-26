use crate::{
    auth::handlers::{
        forgot_password_handler, forgot_pin_handler, login_handler, register_handler,
        reset_password_handler, reset_pin_handler,
    },
    state::AppState,
};
use axum::{Router, routing::post};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/forgot-password", post(forgot_password_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/forgot-pin", post(forgot_pin_handler))
        .route("/reset-pin", post(reset_pin_handler))
        .with_state(state)
}
