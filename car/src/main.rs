use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

#[derive(Serialize)]
struct Car {
    id: i64,
    color: String,
    model: String,
}

#[derive(Deserialize)]
struct CreateCarRequest {
    color: String,
    model: String,
}

async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // create_if_missing ensures the file is created instead of failing to open
    let database_url = "sqlite://./src/car.db";
    let connect_opts = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS car (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            color TEXT NOT NULL,
            model TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?; 

    Ok(pool)
}
#[post("/cars")]
async fn create_car(pool: web::Data<SqlitePool>, car: web::Json<CreateCarRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO car (color, model) VALUES (?, ?)")
        .bind(&car.color)
        .bind(&car.model)
        .execute(&**pool)
        .await
    { 
        Ok(result) => {
            let new_car = Car {
                id: result.last_insert_rowid(),
                color: car.color.clone(),
                model: car.model.clone(),
            };
            HttpResponse::Created().json(new_car)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
#[get("/cars")]
async fn get_cars(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, color, model FROM car ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let cars: Vec<Car> = rows
                .iter()
                .map(|row| Car {
                    id: row.get("id"),
                    color: row.get("color"),
                    model: row.get("model"),
                })
                .collect();
            HttpResponse::Ok().json(cars)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await.expect("Failed to initialize database");
    println!("🚀 Server running at http://127.0.0.1:4001");
    println!("📊 SQLite database initialized at src/car.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_car)
            .service(get_cars)
    })
    .bind(("127.0.0.1", 4001))?
    .run()
    .await
}