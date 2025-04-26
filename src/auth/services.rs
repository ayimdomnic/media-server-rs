use crate::{
    auth::models::{AuthResponse, Claims, LoginRequest, RegisterRequest},
    errors::{AppError, Result},
    state::AppState,
};
use axum::extract::State;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use entity::profile::{
    ActiveModel, Column as ProfileColumn, Entity as ProfileEntity, Model as ProfileModel,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use uuid::Uuid;

pub async fn register_user(
    State(state): State<AppState>,
    req: RegisterRequest,
) -> Result<ProfileModel> {
    let db = &state.conn;

    // Check if email already exists
    let existing_user = ProfileEntity::find()
        .filter(ProfileColumn::Email.eq(&req.email))
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?;

    if existing_user.is_some() {
        return Err(AppError::ValidationError("Email already registered".into()));
    }

    let hashed_password = hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_e| AppError::InternalServerError("Password hashing failed".into()))?;

    let new_user = ActiveModel {
        id: Set(Uuid::new_v4()),
        parent_id: Set(None), // For initial user, no parent
        email: Set(req.email.clone()),
        password: Set(Some(hashed_password)),
        phone: Set(req.phone.clone()),
        name: Set(req.name.clone()),
        avatar: Set(None),
        pin: Set(req.pin.clone()),
        use_pin: Set(req.use_pin.clone()), //handle the Option
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let user = new_user.insert(db).await.map_err(AppError::DatabaseError)?;

    Ok(user)
}

pub async fn login_user(State(state): State<AppState>, req: LoginRequest) -> Result<AuthResponse> {
    let db = &state.conn;

    let user = ProfileEntity::find()
        .filter(ProfileColumn::Email.eq(&req.email))
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::AuthenticationError)?;

    let hashed_password = user.password.ok_or(AppError::AuthenticationError)?;

    if !verify(&req.password, &hashed_password)
        .map_err(|_e| AppError::InternalServerError("Password verification failed".into()))?
    {
        return Err(AppError::AuthenticationError);
    }

    let now = Utc::now();
    let access_token_expiration = now + Duration::hours(24);
    let refresh_token_expiration = now + Duration::days(30); // Example: 30 days

    let access_claims = Claims {
        sub: user.id,
        exp: access_token_expiration.timestamp(),
        iat: now.timestamp(),
        iss: "smartinis_media_server".to_string(), // Replace with your issuer
        aud: "user".to_string(),                   // Define your audience
        jti: Uuid::new_v4().to_string(),
        role: "user".to_string(), // Get from DB
    };

    let refresh_claims = Claims {
        sub: user.id,
        exp: refresh_token_expiration.timestamp(),
        iat: now.timestamp(),
        iss: "smartinis_media_server".to_string(),
        aud: "user".to_string(),
        jti: Uuid::new_v4().to_string(),
        role: "user".to_string(), //  Get from DB
    };

    let encoding_key = EncodingKey::from_secret(state.jwt_secret.as_bytes());
    let access_token = encode(&Header::default(), &access_claims, &encoding_key)
        .map_err(|e| AppError::InternalServerError(format!("JWT encoding failed: {}", e)))?;
    let refresh_token =
        encode(&Header::default(), &refresh_claims, &encoding_key).map_err(|e| {
            AppError::InternalServerError(format!("Refresh JWT encoding failed: {}", e))
        })?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        expires_in: access_token_expiration.timestamp() - now.timestamp(),
        token_type: "Bearer".to_string(),
        scope: "read write".to_string(), // Define your scope
    })
}

pub async fn verify_jwt(State(state): State<AppState>, token: String) -> Result<Claims> {
    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(&token, &decoding_key, &validation)
        .map(|decoded| decoded.claims)
        .map_err(|_| AppError::AuthenticationError)
}

pub async fn forgot_password(State(state): State<AppState>, email: String) -> Result<()> {
    let db = &state.conn;

    // 1. Check if the user with the given email exists
    let user = ProfileEntity::find()
        .filter(ProfileColumn::Email.eq(&email))
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::NotFound)?;

    // 2. Generate a unique password reset token (you might want to store this in the database
    //    with an expiry timestamp)
    let reset_token = Uuid::new_v4().to_string();
    // TODO: Store reset_token and expiry in the database associated with the user

    // 3. Send an email to the user with the reset token and a link to the reset password page
    // TODO: Implement email sending logic

    println!(
        "Password reset initiated for user: {}, token: {}",
        user.email, reset_token
    ); // Placeholder

    Ok(())
}

pub async fn reset_password(
    State(state): State<AppState>,
    _token: String, // The reset token from the email link
    new_password: String,
) -> Result<()> {
    let db = &state.conn;

    // 1. Verify the reset token against the stored token in the database (and check expiry)
    // TODO: Implement token verification logic against the database

    // 2. If the token is valid, hash the new password
    let hashed_password = hash(&new_password, bcrypt::DEFAULT_COST)
        .map_err(|_e| AppError::InternalServerError("Password hashing failed".into()))?;

    // 3. Update the user's password in the database
    let update_result = ProfileEntity::update_many()
        // TODO: Filter by the user associated with the token.  Important security check!
        .filter(ProfileColumn::Id.eq(Uuid::new_v4())) // Replace with actual user ID from token
        .set(ActiveModel {
            password: Set(Some(hashed_password)),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        })
        .exec(db)
        .await
        .map_err(AppError::DatabaseError)?;

    if update_result.rows_affected == 0 {
        return Err(AppError::ValidationError(
            "Invalid or expired reset token".into(),
        ));
    }

    // 4. Optionally, invalidate the reset token in the database
    Ok(())
}

pub async fn set_pin(State(state): State<AppState>, claims: Claims, pin: String) -> Result<()> {
    let db = &state.conn;
    let hashed_pin = hash(&pin, bcrypt::DEFAULT_COST)
        .map_err(|_e| AppError::InternalServerError("Pin hashing failed".into()))?;

    let _update_result = ProfileEntity::update(ActiveModel {
        id: Set(claims.sub),
        pin: Set(Some(hashed_pin)),
        use_pin: Set(Some(true)),
        updated_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    })
    .exec(db)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn forgot_pin(State(state): State<AppState>, email: String) -> Result<()> {
    let db = &state.conn;

    // 1. Check if the user with the given email exists and has a pin set
    let user = ProfileEntity::find()
        .filter(ProfileColumn::Email.eq(&email))
        .filter(ProfileColumn::Pin.is_not_null())
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::NotFound)?;

    // 2. Generate a unique pin reset token (you might want to store this in the database
    //    with an expiry timestamp)
    let reset_token = Uuid::new_v4().to_string();
    // TODO: Store reset_token and expiry in the database associated with the user

    // 3. Send an email (or other method) to the user with the reset token and instructions
    // TODO: Implement communication logic

    println!(
        "Pin reset initiated for user: {}, token: {}",
        user.email, reset_token
    ); // Placeholder

    Ok(())
}

pub async fn reset_pin(
    State(state): State<AppState>,
    _token: String, // The reset token
    new_pin: String,
) -> Result<()> {
    let db = &state.conn;

    // 1. Verify the pin reset token against the stored token in the database (and check expiry)
    // TODO: Implement token verification logic

    // 2. If the token is valid, hash the new pin
    let hashed_pin = hash(&new_pin, bcrypt::DEFAULT_COST)
        .map_err(|_e| AppError::InternalServerError("Pin hashing failed".into()))?;

    // 3. Update the user's pin in the database
    let update_result = ProfileEntity::update_many()
        // TODO: Filter by the user associated with the token
        .filter(ProfileColumn::Id.eq(Uuid::new_v4())) // Replace with actual user ID from token
        .set(ActiveModel {
            pin: Set(Some(hashed_pin)),
            use_pin: Set(Some(true)),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        })
        .exec(db)
        .await
        .map_err(AppError::DatabaseError)?;

    if update_result.rows_affected == 0 {
        return Err(AppError::ValidationError(
            "Invalid or expired reset token".into(),
        ));
    }

    // 4. Optionally, invalidate the reset token
    Ok(())
}

pub async fn delete_account(State(state): State<AppState>, claims: Claims) -> Result<()> {
    let db = &state.conn;

    let delete_result = ProfileEntity::delete(ActiveModel {
        id: Set(claims.sub),
        ..Default::default()
    })
    .exec(db)
    .await
    .map_err(AppError::DatabaseError)?;

    if delete_result.rows_affected == 0 {
        return Err(AppError::NotFound); // User not found
    }

    Ok(())
}

pub async fn update_profile(
    State(state): State<AppState>,
    claims: Claims,
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    avatar: Option<String>,
) -> Result<ProfileModel> {
    let db = &state.conn;

    let user = ProfileEntity::find_by_id(claims.sub)
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?;

    let mut active_model = match user {
        Some(user_model) => user_model.into_active_model(),
        None => return Err(AppError::NotFound),
    };

    if let Some(n) = name {
        active_model.name = Set(n);
    }
    if let Some(e) = email {
        // Optionally check if the new email is already in use
        let existing_user = ProfileEntity::find()
            .filter(ProfileColumn::Email.eq(&e))
            .filter(ProfileColumn::Id.ne(claims.sub)) // Don't check against the current user
            .one(db)
            .await
            .map_err(AppError::DatabaseError)?;
        if existing_user.is_some() {
            return Err(AppError::ValidationError("Email already in use".into()));
        }
        active_model.email = Set(e);
    }
    if let Some(p) = phone {
        active_model.phone = Set(Some(p));
    }
    if let Some(a) = avatar {
        active_model.avatar = Set(Some(a));
    }
    active_model.updated_at = Set(Utc::now().naive_utc());

    let updated_user = active_model
        .update(db)
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(updated_user)
}

pub async fn add_child_profile(
    State(state): State<AppState>,
    claims: Claims, // JWT of the parent account
    name: String,
) -> Result<ProfileModel> {
    let db = &state.conn;

    let new_child = ActiveModel {
        id: Set(Uuid::new_v4()),
        parent_id: Set(Some(claims.sub)),       // Set the parent ID
        email: Set(Uuid::new_v4().to_string()), // Generate a unique dummy email
        password: Set(None),
        phone: Set(None),
        name: Set(name),
        avatar: Set(None),
        pin: Set(None),
        use_pin: Set(Some(false)),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let child_profile = new_child
        .insert(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(child_profile)
}
