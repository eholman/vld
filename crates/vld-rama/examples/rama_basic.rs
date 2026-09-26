//! Rama + vld extractors example.
//!
//! Run:
//! ```sh
//! cargo run -p vld-rama --example rama_basic
//! ```

use rama::http::service::web::response::Json;
use rama::http::service::web::WebService;
use rama::http::{Body, Method, Request, StatusCode};
use rama::Service;
use serde::Serialize;
use vld_rama::{VldHeaders, VldJson, VldQuery};

vld::schema! {
    #[derive(Debug)]
    pub struct CreateUser {
        pub name: String => vld::string().min(2).max(50),
        pub email: String => vld::string().email(),
    }
}

vld::schema! {
    #[derive(Debug, Serialize)]
    pub struct CreateUserResponse {
        pub status: String => vld::string(),
        pub name: String => vld::string(),
        pub email: String => vld::string(),
    }
}

vld::schema! {
    #[derive(Debug)]
    pub struct Search {
        pub q: String => vld::string().min(1),
        pub page: Option<i64> => vld::number().int().min(1).optional(),
    }
}

vld::schema! {
    #[derive(Debug)]
    pub struct AuthHeaders {
        pub authorization: String => vld::string().min(1),
    }
}

async fn create_user(VldJson(req): VldJson<CreateUser>) -> Json<CreateUserResponse> {
    Json(CreateUserResponse {
        status: "created".into(),
        name: req.name,
        email: req.email,
    })
}

async fn search(VldQuery(q): VldQuery<Search>) -> String {
    format!("q={} page={}", q.q, q.page.unwrap_or(1))
}

async fn authed(
    VldHeaders(h): VldHeaders<AuthHeaders>,
    VldJson(user): VldJson<CreateUser>,
) -> String {
    format!("auth={} name={}", h.authorization, user.name)
}

#[tokio::main]
async fn main() {
    let svc = WebService::default()
        .with_post("/users", create_user)
        .with_get("/search", search)
        .with_post("/authed", authed);

    // --- create user ---
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://example.com/users")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"Alice","email":"alice@example.com"}"#,
        ))
        .unwrap();
    let resp = svc.serve(req).await.unwrap();
    println!("POST /users => {}", resp.status());
    assert_eq!(resp.status(), StatusCode::OK);

    // --- search ---
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://example.com/search?q=hello&page=2")
        .body(Body::empty())
        .unwrap();
    let resp = svc.serve(req).await.unwrap();
    println!("GET /search => {}", resp.status());
    assert_eq!(resp.status(), StatusCode::OK);

    // --- validation error ---
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://example.com/users")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"A","email":"bad"}"#))
        .unwrap();
    let resp = svc.serve(req).await.unwrap();
    println!("POST /users (invalid) => {}", resp.status());
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    println!("ok");
}
