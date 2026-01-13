use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};

use crate::models::client_model::{Client, CreateClientRequest, UpdateClientRequest};
use crate::controllers::{driver_controller, events_controller};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    init_clients_table(&pool).await?;
    driver_controller::init_drivers_table(&pool).await?;
    events_controller::init_events_table(&pool).await?;

    Ok(pool)
}

pub async fn init_clients_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clients (
            id INTEGER NOT NULL UNIQUE,
            f_name TEXT NOT NULL,
            l_name TEXT NOT NULL,
            address TEXT NOT NULL,
            phone TEXT NOT NULL UNIQUE,
            PRIMARY KEY(id AUTOINCREMENT)
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[post("/clients")]
pub async fn create_client(
    pool: web::Data<SqlitePool>,
    client: web::Json<CreateClientRequest>,
) -> impl Responder {
    match sqlx::query("INSERT INTO clients (f_name, l_name, address, phone) VALUES (?, ?, ?, ?)")
        .bind(&client.f_name)
        .bind(&client.l_name)
        .bind(&client.address)
        .bind(&client.phone)
        .execute(&**pool)
        .await
    {
        Ok(result) => HttpResponse::Created().json(Client {
            id: result.last_insert_rowid(),
            f_name: client.f_name.clone(),
            l_name: client.l_name.clone(),
            address: client.address.clone(),
            phone: client.phone.clone(),
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/clients")]
pub async fn get_clients(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, f_name, l_name, address, phone FROM clients ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter().map(|row| Client {
                id: row.get("id"),
                f_name: row.get("f_name"),
                l_name: row.get("l_name"),
                address: row.get("address"),
                phone: row.get("phone"),
            }).collect::<Vec<_>>()
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/clients/{id}")]
pub async fn get_client(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("SELECT id, f_name, l_name, address, phone FROM clients WHERE id = ?")
        .bind(id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => HttpResponse::Ok().json(Client {
            id: row.get("id"),
            f_name: row.get("f_name"),
            l_name: row.get("l_name"),
            address: row.get("address"),
            phone: row.get("phone"),
        }),
        Ok(None) => HttpResponse::NotFound().body("Client not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/clients/{id}")]
pub async fn update_client(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateClientRequest>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("UPDATE clients SET f_name = ?, l_name = ?, address = ?, phone = ? WHERE id = ?")
        .bind(&updated.f_name)
        .bind(&updated.l_name)
        .bind(&updated.address)
        .bind(&updated.phone)
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().json(Client {
            id,
            f_name: updated.f_name.clone(),
            l_name: updated.l_name.clone(),
            address: updated.address.clone(),
            phone: updated.phone.clone(),
        }),
        Ok(_) => HttpResponse::NotFound().body("Client not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/clients/{id}")]
pub async fn delete_client(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("DELETE FROM clients WHERE id = ?")
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Client not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
