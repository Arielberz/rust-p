use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::driver_model::{Driver, CreateDriverRequest, UpdateDriverRequest};

pub async fn init_drivers_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS drivers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            f_name TEXT NOT NULL,
            l_name TEXT NOT NULL,
            phone TEXT NOT NULL UNIQUE
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[post("/drivers")]
pub async fn create_driver(
    pool: web::Data<SqlitePool>,
    driver: web::Json<CreateDriverRequest>,
) -> impl Responder {
    match sqlx::query("INSERT INTO drivers (f_name, l_name, phone) VALUES (?, ?, ?)")
        .bind(&driver.f_name)
        .bind(&driver.l_name)
        .bind(&driver.phone)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_driver = Driver {
                id: result.last_insert_rowid(),
                f_name: driver.f_name.clone(),
                l_name: driver.l_name.clone(),
                phone: driver.phone.clone(),
            };
            HttpResponse::Created().json(new_driver)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/drivers")]
pub async fn get_drivers(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, f_name, l_name, phone FROM drivers ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let drivers: Vec<Driver> = rows
                .iter()
                .map(|row| Driver {
                    id: row.get("id"),
                    f_name: row.get("f_name"),
                    l_name: row.get("l_name"),
                    phone: row.get("phone"),
                })
                .collect();
            HttpResponse::Ok().json(drivers)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/drivers/{id}")]
pub async fn get_driver(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("SELECT id, f_name, l_name, phone FROM drivers WHERE id = ?")
        .bind(&id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let driver = Driver {
                id: row.get("id"),
                f_name: row.get("f_name"),
                l_name: row.get("l_name"),
                phone: row.get("phone"),
            };
            HttpResponse::Ok().json(driver)
        }
        Ok(None) => HttpResponse::NotFound().body("Driver not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/drivers/{id}")]
pub async fn update_driver(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateDriverRequest>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query(
        "UPDATE drivers SET f_name = ?, l_name = ?, phone = ? WHERE id = ?"
    )
    .bind(&updated.f_name)
    .bind(&updated.l_name)
    .bind(&updated.phone)
    .bind(&id)
    .execute(&**pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_driver = Driver {
                    id,
                    f_name: updated.f_name.clone(),
                    l_name: updated.l_name.clone(),
                    phone: updated.phone.clone(),
                };
                HttpResponse::Ok().json(updated_driver)
            } else {
                HttpResponse::NotFound().body("Driver not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/drivers/{id}")]
pub async fn delete_driver(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM drivers WHERE id = ?")
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Deleted")
            } else {
                HttpResponse::NotFound().body("Driver not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
