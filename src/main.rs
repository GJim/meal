use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserializer, Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use sqlx::types::chrono::NaiveDateTime;
// use sqlx::types::chrono::format::ParseError;

async fn add_product(
    State(state): State<DBState>,
    Json(data): Json<ProductNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, CreateNew>("INSERT INTO products (name, price) VALUES ($1, $2) RETURNING no")
        .bind(&data.name)
        .bind(&data.price)
        .fetch_one(&state.pool)
        .await
    {
        Ok(new) => Ok((StatusCode::CREATED, Json(new))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_all_products(
    State(state): State<DBState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Product>("SELECT no, name, price FROM products where deleted = false")
        .fetch_all(&state.pool)
        .await
    {
        Ok(results) => Ok((StatusCode::OK, Json(results))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add_label(
    State(state): State<DBState>,
    Json(data): Json<LabelNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, CreateNew>("INSERT INTO labels (name, initial, finish) VALUES ($1, $2, $3) RETURNING no")
        .bind(&data.name)
        .bind(&data.initial)
        .bind(&data.finish)
        .fetch_one(&state.pool)
        .await
    {
        Ok(new) => Ok((StatusCode::CREATED, Json(new))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_all_labels(
    State(state): State<DBState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Label>("SELECT no, name, initial, finish FROM labels where deleted = false")
        .fetch_all(&state.pool)
        .await
    {
        Ok(results) => Ok((StatusCode::OK, Json(results))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add_ingredient(
    State(state): State<DBState>,
    Json(data): Json<IngredientNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, CreateNew>("INSERT INTO ingredients (name, product, quantity, unit) VALUES ($1, $2, $3, $4) RETURNING no")
        .bind(&data.name)
        .bind(&data.product)
        .bind(&data.quantity)
        .bind(&data.unit)
        .fetch_one(&state.pool)
        .await
    {
        Ok(new) => Ok((StatusCode::CREATED, Json(new))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_all_ingredients(
    State(state): State<DBState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Ingredient>("SELECT no, name, product, quantity, unit FROM ingredients where deleted = false")
        .fetch_all(&state.pool)
        .await
    {
        Ok(results) => Ok((StatusCode::OK, Json(results))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add_order(
    State(state): State<DBState>,
    Json(data): Json<OrderNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, CreateNew>("INSERT INTO orders (product, quantity, price, labels, time) VALUES ($1, $2, $3, $4, now()) RETURNING no")
        .bind(&data.product)
        .bind(&data.quantity)
        .bind(&data.price)
        .bind(&data.labels)
        .fetch_one(&state.pool)
        .await
    {
        Ok(new) => Ok((StatusCode::CREATED, Json(new))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_range_orders(
    State(state): State<DBState>,
    Json(data): Json<DateTimeRange>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Order>("SELECT no, product, quantity, price, labels FROM orders where time >= $1 and time <= $2 and deleted = false")
        .bind(&data.start)
        .bind(&data.end)
        .fetch_all(&state.pool)
        .await
    {
        Ok(results) => Ok((StatusCode::OK, Json(results))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../dist/index.html"))
}

#[derive(Clone)]
struct DBState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let corsl = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    let state = DBState { pool };
    let router = Router::new()
        .route("/", get(index))
        .route("/product/add", post(add_product))
        .route("/product/all", get(get_all_products))
        .route("/label/add", post(add_label))
        .route("/label/all", get(get_all_labels))
        .route("/ingredient/add", post(add_ingredient))
        .route("/ingredient/all", get(get_all_ingredients))
        .route("/order/add", post(add_order))
        .route("/order/range", post(get_range_orders))
        .layer(corsl)
        .with_state(state);

    Ok(router.into())
}

#[derive(Serialize, FromRow)]
struct CreateNew {
    no: i32
}

#[derive(Deserialize)]
struct ProductNew {
    pub name: String,
    pub price: i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub no: i32,
    pub name: String,
    pub price: i32,
}

fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}


#[derive(Deserialize)]
struct LabelNew {
    pub name: String,
    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub initial: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub finish: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Label {
    pub no: i32,
    pub name: String,
    pub initial: NaiveDateTime,
    pub finish: NaiveDateTime,
}

#[derive(Deserialize)]
struct IngredientNew {
    pub name: String,
    pub product: i32,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub no: i32,
    pub name: String,
    pub product: i32,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Deserialize)]
struct OrderNew {
    pub product: i32,
    pub quantity: i32,
    pub price: i32,
    pub labels: String,
}

#[derive(Deserialize)]
struct DateTimeRange {
    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub start: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_naive_date_time")]
    pub end: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Order {
    pub no: i32,
    pub product: i32,
    pub quantity: i32,
    pub price: i32,
    pub labels: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Settings {
    pub redundancy: f32
}
