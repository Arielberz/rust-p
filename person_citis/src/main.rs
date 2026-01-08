use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct City {
    id: i64,
    name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    id: i64,
    f_name: String,
    l_name: String,
}

#[derive(Debug, Deserialize)]
struct CreateCityRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateCityRequest {
    name: String,
}

async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            f_name TEXT NOT NULL,
            l_name TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

#[post("/cities")]
async fn create_city(pool: web::Data<SqlitePool>, city: web::Json<CreateCityRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO cities (name) VALUES (?)")
        .bind(&city.name)
        .execute(&**pool)
        .await
    { 
        Ok(result) => {
            let new_city = City {
                id: result.last_insert_rowid(),
                name: city.name.clone(),
            };
            HttpResponse::Created().json(new_city)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
#[get("/cities")]
async fn get_cities(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, name FROM cities ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let cities: Vec<City> = rows
                .iter()
                .map(|row| City {
                    id: row.get("id"),
                    name: row.get("name"),
                })
                .collect();
            HttpResponse::Ok().json(cities)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[post("/persons")]
async fn create_person(pool: web::Data<SqlitePool>, person: web::Json<Person>) -> impl Responder {
    match sqlx::query("INSERT INTO persons (f_name, l_name) VALUES (?, ?)")
        .bind(&person.f_name)
        .bind(&person.l_name)
        .execute(&**pool)
        .await
    { 
        Ok(result) => {
            let new_person = Person {
                id: result.last_insert_rowid(),
                f_name: person.f_name.clone(),
                l_name: person.l_name.clone(),
            };
            HttpResponse::Created().json(new_person)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
#[get("/persons")]
async fn get_persons(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, f_name, l_name FROM persons ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let persons: Vec<Person> = rows
                .iter()
                .map(|row| Person {
                    id: row.get("id"),      
                    f_name: row.get("f_name"),
                    l_name: row.get("l_name"),
                })
                .collect();
            HttpResponse::Ok().json(persons)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await.expect("Failed to initialize database");
    println!("🚀 Server running at http://127.0.0.1:4001");
    println!("📊 SQLite database initialized at src/mydb.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_city)
            .service(get_cities)
            .service(create_person)
            .service(get_persons)
    })
    .bind(("127.0.0.1", 4001))?
    .run()
    .await
}