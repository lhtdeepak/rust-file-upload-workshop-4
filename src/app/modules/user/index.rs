use actix_web::web;
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[path = "../../constants/index.rs"]
mod constants;
#[path = "../../models/user.rs"]
pub(crate) mod model;
use model::Claims;
use model::User;

#[derive(Error, Debug)]
pub enum Error_JWT {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}

type Result_JWT<T> = std::result::Result<T, Error_JWT>;
const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    id: String,
    pass: String,
    exp: usize,
}

pub async fn create_user(
    client: web::Data<Client>,
    request_data: User,
) -> Result<InsertOneResult, Error> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let new_doc = User {
        id: request_data.id,
        first_name: request_data.first_name,
        last_name: request_data.last_name,
        username: request_data.username,
        email: request_data.email,
    };

    let user = collection
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creating user");
    Ok(user)
}

pub async fn get_user(client: web::Data<Client>, id: web::Path<String>) -> Result<User, Error> {
    let _id = id.into_inner();

    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let user_detail = collection
        .find_one(doc! {"id": _id}, None)
        .await
        .ok()
        .expect("Error getting user's detail");

    Ok(user_detail.unwrap())
}

pub async fn update_user(
    client: web::Data<Client>,
    user_id: String,
    request_data: User,
    uid: String,
) -> Result<User, Error> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let update_id = user_id;
    let filter = doc! {"id": update_id};

    let new_doc = doc! {
        "$set": {
            "id": request_data.id,
            "first_name": request_data.first_name,
            "last_name": request_data.last_name,
            "username": request_data.username,
            "email": request_data.email,
        }
    };

    collection
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Error updating user");

    let updated_doc = collection
        .find_one(doc! {"id": uid}, None)
        .await
        .ok()
        .expect("Error getting user's detail");

    Ok(updated_doc.unwrap())
}

pub async fn get_all_users(client: web::Data<Client>) -> Result<Vec<User>, Error> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let mut users: Vec<User> = Vec::new();

    let mut result = collection
        .find(None, None)
        .await
        .ok()
        .expect("Error fetching user details");

    while let Some(user) = result
        .try_next()
        .await
        .ok()
        .expect("Error mapping through result")
    {
        users.push(user)
    }

    Ok(users)
}

pub async fn delete_user(client: web::Data<Client>, id: String) -> Result<DeleteResult, Error> {
    let collection: Collection<User> = client
        .database(constants::DB_NAME)
        .collection(constants::USER_COLLECTION);

    let filter = doc! {"id": id};

    let mut result = collection
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Error deleting user");

    Ok(result)
}

// JWT Implementation ---------------------

pub async fn create_jwt_token(request_data: Claims) -> Result_JWT<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Info {
        id: request_data.username.to_owned(),
        pass: request_data.password.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error_JWT::JWTTokenCreationError);

    return token;
}
