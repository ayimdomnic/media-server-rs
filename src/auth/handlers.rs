use crate::{
    auth::{
        models::{Claims, LoginRequest, RegisterRequest},
        services::{
            add_child_profile, delete_account, forgot_password, forgot_pin, login_user,
            register_user, reset_password, reset_pin, set_pin, update_profile, verify_jwt,
        },
    },
    errors::Result,
    state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

// Helper function for consistent error responses
fn error_response(_status: StatusCode, message: &str) -> Response {
    Json(json!({
        "error": message,
    }))
    .into_response()
}

// Handler for user registration
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    let user = register_user(State(state), req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

// Handler for user login
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let auth_response = login_user(State(state), req).await?;
    Ok((StatusCode::OK, Json(auth_response)))
}

// Handler for JWT verification (protected endpoint example)
pub async fn protected_handler(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<impl IntoResponse> {
    // In a real application, you would use the claims to authorize the user
    // for the specific action they are trying to perform.
    let verified_claims = verify_jwt(State(state), claims.clone().jti).await?; // added jti
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Protected resource accessed", "claims": verified_claims })),
    ))
}

// Handler for forgot password
pub async fn forgot_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>, // Changed to ForgotPasswordRequest
) -> Result<impl IntoResponse> {
    forgot_password(State(state), payload.email).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Password reset initiated. Check your email." })),
    ))
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

// Handler for reset password
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>, // Changed to ResetPasswordRequest
) -> Result<impl IntoResponse> {
    reset_password(State(state), payload.token, payload.new_password).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Password reset successfully." })),
    ))
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

// Handler for set pin
pub async fn set_pin_handler(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<SetPinRequest>, // Changed to SetPinRequest
) -> Result<impl IntoResponse> {
    set_pin(State(state), claims, payload.pin).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Pin set successfully." })),
    ))
}

#[derive(Deserialize)]
pub struct SetPinRequest {
    pub pin: String,
}

// Handler for forgot pin
pub async fn forgot_pin_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPinRequest>, // Changed to ForgotPinRequest.
) -> Result<impl IntoResponse> {
    forgot_pin(State(state), payload.email).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Pin reset initiated. Check your email." })),
    ))
}

#[derive(Deserialize)]
pub struct ForgotPinRequest {
    pub email: String,
}

// Handler for reset pin
pub async fn reset_pin_handler(
    State(state): State<AppState>,
    Json(payload): Json<ResetPinRequest>, // Changed to ResetPinRequest
) -> Result<impl IntoResponse> {
    reset_pin(State(state), payload.token, payload.new_pin).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Pin reset successfully." })),
    ))
}

#[derive(Deserialize)]
pub struct ResetPinRequest {
    pub token: String,
    pub new_pin: String,
}

// Handler for delete account
pub async fn delete_account_handler(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<impl IntoResponse> {
    delete_account(State(state), claims).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Account deleted successfully." })),
    ))
}

// Handler for update profile
pub async fn update_profile_handler(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UpdateProfileRequest>, // Changed to UpdateProfileRequest
) -> Result<impl IntoResponse> {
    let updated_user = update_profile(
        State(state),
        claims,
        payload.name,
        payload.email,
        payload.phone,
        payload.avatar,
    )
    .await?;
    Ok((StatusCode::OK, Json(updated_user)))
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
}

// Handler for add child profile
pub async fn add_child_profile_handler(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<AddChildProfileRequest>, // Changed to AddChildProfileRequest
) -> Result<impl IntoResponse> {
    let child_profile = add_child_profile(State(state), claims, payload.name).await?;
    Ok((StatusCode::CREATED, Json(child_profile)))
}

#[derive(Deserialize)]
pub struct AddChildProfileRequest {
    pub name: String,
}
