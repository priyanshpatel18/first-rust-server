use axum::{
    extract::{Json as AxumJson, Path}, http::StatusCode, response::{Html, Json}, routing::{get, post}, Router
};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: u32,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn list_users() -> Json<Value> {
    let users = vec![
        User {
            id: Some(1),
            name: "John Doe".to_string(),
            email: "6oL0U@example.com".to_string(),
            age: 30,
        },
        User {
            id: Some(2),
            name: "Jane Doe".to_string(),
            email: "6oL0U@example.com".to_string(),
            age: 25,
        },
    ];

    Json(json!({
        "users": users,
        "total": users.len()
    }))
}

async fn create_user(
    AxumJson(user): AxumJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    if user.name.is_empty() || user.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = User {
        id: Some(44),
        name: user.name,
        email: user.email,
        age: user.age,
    };

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "User created successfully",
            "user": user
        })),
    ))
}

async fn get_user(Path(user_id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    if user_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = User {
        id: Some(user_id),
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
        age: 30,
    };

    Ok(Json(json!({
        "user": user
    })))
}

async fn update_user(
    Path(user_id): Path<u32>,
    AxumJson(payload): AxumJson<CreateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    if user_id == 0 || payload.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = User {
        id: Some(user_id),
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };

    Ok(Json(json!({
        "message": "User updated successfully",
        "user": user
    })))
}

async fn delete_user(Path(user_id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    if user_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(json!({
        "message": format!("User {} deleted successfully", user_id)
    })))
}
