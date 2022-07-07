#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused)]
#![allow(non_snake_case)]
use actix_web::{delete, get, post, put, web, web::Json, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use user_controller::filter_model::Filter;
use user_controller::model::Claims;
use user_controller::model::User;
use uuid::Uuid;
#[path = "../app/modules/user/index.rs"]
mod user_controller;
#[path = "../app/models/user.rs"]
mod model;
const JWT_SECRET: &[u8] = b"secret";
// routes
#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Rust microservice alive!")
}
#[get("/get-user/{id}")]
pub async fn get_user(
    client: web::Data<Client>,
    id: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let headerToken = req
        .headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let token = headerToken[7..headerToken.len()].to_string();
    if token.len() > 0 {
        let user_details = user_controller::get_user(client, id).await;
        match user_details {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::InternalServerError().body("authorization failed")
    }
}
#[post("/create-user")]
pub async fn create_user(client: web::Data<Client>, req: Json<User>) -> HttpResponse {
    let uid = Uuid::new_v4();
    let request_data = User {
        id: uid.to_string(),
        first_name: req.first_name.to_owned(),
        last_name: req.last_name.to_owned(),
        username: req.username.to_owned(),
        email: req.email.to_owned(),
    };
    let response = user_controller::create_user(client, request_data).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[put("/update-user")]
pub async fn update_user(client: web::Data<Client>, req: Json<User>) -> HttpResponse {
    let request_data = User {
        id: req.id.to_owned(),
        first_name: req.first_name.to_owned(),
        last_name: req.last_name.to_owned(),
        username: req.username.to_owned(),
        email: req.email.to_owned(),
    };
    let response =
        user_controller::update_user(client, req.id.to_owned(), request_data, req.id.to_owned())
            .await;
    match response {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/get-all-users")]
pub async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
    let response = user_controller::get_all_users(client).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/delete-user/{id}")]
pub async fn delete_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let _id = id.into_inner();
    let response = user_controller::delete_user(client, _id).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
// JWT APIs ---------------------
#[post("/create-jwt-token")]
pub async fn create_jwt_token(req_data: Json<Claims>) -> HttpResponse {
    let request_data = req_data.into_inner();
    let token_detail = user_controller::create_jwt_token(request_data).await;
    match token_detail {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[post("/search_user")]
pub async fn search_users(client: web::Data<Client>, new_filter: Json<Filter>) -> HttpResponse {
    let data = Filter {
        filter_field: new_filter.filter_field.to_owned(),
        filter_key: new_filter.filter_key.to_owned(),
    };
    let user_details = user_controller::get_search_user(client, data).await;
    match user_details {
        Ok(user_details) => HttpResponse::Ok().json(user_details),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}