use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::{json, Value};
use tera::Context;
use tracing::error;

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::neo::{CreateDateRange, CreateNeo, Neo, NeoId};
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};

use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();

    let template_name = if let Some(claims_data) = claims {
        let admin = am_database.check_admin(claims_data.email.clone());
        if admin {
            context.insert("admin_logged_in", &true);
        }
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the page data
        let page_packages = am_database.get_all_neo_pages().await?;
        context.insert("page_packages", &page_packages);

        "pages.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

#[allow(dead_code)]
pub async fn register_page(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        // Get all the page data
        let page_packages = am_database.get_all_neo_pages().await?;
        context.insert("page_packages", &page_packages);

        "pages.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "register.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

pub async fn register(
    State(database): State<Store>,
    Form(mut credentials): Form<UserSignup>,
) -> Result<Json<Value>, AppError> {
    // We should also check to validate other things at some point like email address being in right format

    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    // Check to see if there is already a user in the database with the given email address
    let existing_user = database.get_user(&credentials.email).await;

    if let Ok(_) = existing_user {
        return Err(AppError::UserAlreadyExists);
    }

    // Here we're assured that our credentials are valid and the user doesn't already exist
    // hash their password
    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        // If you'd like unique salts per-user, simply pass &[] and argon will generate them for you
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };

    credentials.password = hashed_password;

    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;

    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }

    // at this point we've authenticated the user's identity
    // create JWT to return
    let claims = Claims {
        id: 0,
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}

pub async fn get_neos(State(am_database): State<Store>) -> Result<Json<Vec<Neo>>, AppError> {
    let all_neos = am_database.get_all_neos().await?;

    Ok(Json(all_neos))
}

pub async fn create_neo(
    State(mut am_database): State<Store>,
    Json(neo): Json<CreateNeo>,
) -> Result<Json<Neo>, AppError> {
    let neo = am_database
        .add_neo(
            neo.api_id,
            neo.designation,
            neo.diameter_min,
            neo.diameter_max,
            neo.hazardous_asteroid,
            neo.approach_date,
            neo.velocity,
            neo.miss_distance,
            neo.orbiting_body,
        )
        .await?;

    Ok(Json(neo))
}

pub async fn get_neo_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/neo/5
) -> Result<Json<Vec<Neo>>, AppError> {
    let neo = am_database.get_neo_by_id(NeoId(query)).await?;
    Ok(Json(neo))
}

pub async fn get_neo_by_date(
    State(mut am_database): State<Store>,
    Json(neo): Json<CreateDateRange>,
) -> Result<Json<Vec<Neo>>, AppError> {
    let neo = am_database
        .get_neo_by_date(neo.begin_date, neo.end_date)
        .await?;
    Ok(Json(neo))
}
