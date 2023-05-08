use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Duration;
use database::users;
use database::users::Entity as Users;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};
use uuid::Uuid;
mod database;
mod helpers;
// use helpers::meta::{get_data, get_paths, write_file};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    iat: u64,
}

#[derive(Serialize, Deserialize)]
struct ResponseUser {
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u16,
    username: String,
    password: String,
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    let db = Database::connect(dotenv!("DATABASE_URL")).await.unwrap();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
    //    write_file(get_data(
    //        get_paths("static/music"),
    //        get_paths("static/covers"),
    //    ));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/tracks", get(get_tracks))
        .route("/tracks/:id", get(get_track))
        .route("/add_favourite", post(add_favourite))
        .route("/delete_favourite", delete(delete_favourite))
        .route("/get_favourite", get(get_favourite))
        // .route("/temp", get(get_users))
        .nest_service("/static", ServeDir::new("static"))
        .layer(cors)
        .with_state(db);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn get_users(State(db): State<DatabaseConnection>) -> Result<Json<Vec<User>>, StatusCode> {
//     let people = Users::find().all(&db).await;
//     match people {
//         Ok(data) => {
//             let mut response = vec![];
//             data.iter().for_each(|item| {
//                 let person = User {
//                     id: item.id as u16,
//                     username: item.username.clone(),
//                     password: item.password.clone(),
//                     token: item.token.clone(),
//                 };
//                 response.push(person);
//             });
//             Ok(Json(response))
//         }
//         Err(_) => Err(StatusCode::NOT_FOUND),
//     }
// }

async fn register(
    State(db): State<DatabaseConnection>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<String>, (StatusCode, Json<String>)> {
    let password = bcrypt::hash(credentials.password, 14).unwrap();
    let user = Users::find()
        .filter(users::Column::Username.eq(&credentials.login))
        .one(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Something went wrong, try again later".to_owned()),
            )
        })?;
    if let Some(_) = user {
        return Err((
            StatusCode::CONFLICT,
            Json("Such user already exists".to_owned()),
        ));
    } else {
        let new_user = users::ActiveModel {
            username: Set(credentials.login),
            password: Set(password),
            ..Default::default()
        };

        let new_user = new_user.insert(&db).await.unwrap();

        let answer = format!(
            "Congratulations! You are: id - {}, name - {}, password - {}, token - {}",
            new_user.id,
            new_user.username,
            new_user.password,
            new_user.token.unwrap_or("No token provided".to_owned())
        );
        Ok(Json(answer))
    }
}
async fn login(
    State(db): State<DatabaseConnection>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<ResponseUser>, (StatusCode, Json<String>)> {
    let user = Users::find()
        .filter(users::Column::Username.eq(credentials.login))
        .one(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Something went wrong, try again later".to_owned()),
            )
        })?;
    if let Some(user) = user {
        let check = bcrypt::verify(credentials.password, &user.password).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Something went wrong, try again later".to_owned()),
            )
        })?;
        if check {
            let now = chrono::Local::now();
            let at = now + Duration::minutes(10);
            let claims = Claims {
                exp: at.timestamp() as u64,
                iat: now.timestamp() as u64,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(Uuid::new_v4().as_bytes()),
            )
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Something went wrong, try again later".to_owned()),
                )
            })?;
            let mut user = user.into_active_model();
            user.token = Set(Some(token));
            let updated_user = user.update(&db).await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Something went wrong, try again later".to_owned()),
                )
            })?;
            let answer = ResponseUser {
                username: updated_user.username,
                token: updated_user.token.unwrap(),
            };
            return Ok(Json(answer));
        } else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json("Wrong username and/or password".to_owned()),
            ));
        }
    } else {
        return Err((StatusCode::NOT_FOUND, Json("No such user".to_owned())));
    }
}
async fn logout() {}
async fn get_tracks() {}
async fn get_track() {}
async fn add_favourite() {}
async fn delete_favourite() {}
async fn get_favourite() {}
