use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};

use crate::models::events_model::{Event, CreateEventRequest, UpdateEventRequest, EventWithDriverClient};

pub async fn init_events_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            "from" TEXT NOT NULL,
            to_city TEXT NOT NULL,
            price INTEGER NOT NULL,
            id_drive INTEGER NOT NULL,
            client_id INTEGER NOT NULL,
            FOREIGN KEY (id_drive) REFERENCES drivers(id),
            FOREIGN KEY (client_id) REFERENCES clients(id)
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[post("/events")]
pub async fn create_event(
    pool: web::Data<SqlitePool>,
    event: web::Json<CreateEventRequest>,
) -> impl Responder {
    match sqlx::query("SELECT id FROM drivers WHERE id = ?")
        .bind(event.id_drive)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Driver does not exist"),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    match sqlx::query("SELECT id FROM clients WHERE id = ?")
        .bind(event.client_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Client does not exist"),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    match sqlx::query(
        r#"INSERT INTO events ("from", to_city, price, id_drive, client_id) VALUES (?, ?, ?, ?, ?)"#
    )
    .bind(&event.from_)
    .bind(&event.to_city)
    .bind(event.price)
    .bind(event.id_drive)
    .bind(event.client_id)
    .execute(&**pool)
    .await
    {
        Ok(result) => HttpResponse::Created().json(Event {
            id: result.last_insert_rowid(),
            from_: event.from_.clone(),
            to_city: event.to_city.clone(),
            price: event.price,
            id_drive: event.id_drive,
            client_id: event.client_id,
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/events")]
pub async fn get_events(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query(
        r#"
        SELECT
            e.id,
            e."from" AS "from",
            e.to_city,
            e.price,
            e.id_drive,
            e.client_id,
            d.f_name AS driver_f_name,
            d.l_name AS driver_l_name,
            d.phone AS driver_phone,
            c.f_name AS client_f_name,
            c.l_name AS client_l_name,
            c.phone AS client_phone,
            c.address AS client_address
        FROM events e
        JOIN drivers d ON e.id_drive = d.id
        JOIN clients c ON e.client_id = c.id
        ORDER BY e.id
        "#
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter().map(|row| EventWithDriverClient {
                id: row.get("id"),
                from_: row.get("from"),
                to_city: row.get("to_city"),
                price: row.get("price"),
                id_drive: row.get("id_drive"),
                client_id: row.get("client_id"),
                driver_f_name: row.get("driver_f_name"),
                driver_l_name: row.get("driver_l_name"),
                driver_phone: row.get("driver_phone"),
                client_f_name: row.get("client_f_name"),
                client_l_name: row.get("client_l_name"),
                client_phone: row.get("client_phone"),
                client_address: row.get("client_address"),
            }).collect::<Vec<_>>()
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/events/{id}")]
pub async fn get_event(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query(
        r#"
        SELECT
            e.id,
            e."from" AS "from",
            e.to_city,
            e.price,
            e.id_drive,
            e.client_id,
            d.f_name AS driver_f_name,
            d.l_name AS driver_l_name,
            d.phone AS driver_phone,
            c.f_name AS client_f_name,
            c.l_name AS client_l_name,
            c.phone AS client_phone,
            c.address AS client_address
        FROM events e
        JOIN drivers d ON e.id_drive = d.id
        JOIN clients c ON e.client_id = c.id
        WHERE e.id = ?
        "#
    )
    .bind(id)
    .fetch_optional(&**pool)
    .await
    {
        Ok(Some(row)) => HttpResponse::Ok().json(EventWithDriverClient {
            id: row.get("id"),
            from_: row.get("from"),
            to_city: row.get("to_city"),
            price: row.get("price"),
            id_drive: row.get("id_drive"),
            client_id: row.get("client_id"),
            driver_f_name: row.get("driver_f_name"),
            driver_l_name: row.get("driver_l_name"),
            driver_phone: row.get("driver_phone"),
            client_f_name: row.get("client_f_name"),
            client_l_name: row.get("client_l_name"),
            client_phone: row.get("client_phone"),
            client_address: row.get("client_address"),
        }),
        Ok(None) => HttpResponse::NotFound().body("Event not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/events/{id}")]
pub async fn update_event(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateEventRequest>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("SELECT id FROM drivers WHERE id = ?")
        .bind(updated.id_drive)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Driver does not exist"),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    match sqlx::query("SELECT id FROM clients WHERE id = ?")
        .bind(updated.client_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Client does not exist"),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    match sqlx::query(
        r#"
        UPDATE events
        SET "from" = ?, to_city = ?, price = ?, id_drive = ?, client_id = ?
        WHERE id = ?
        "#
    )
    .bind(&updated.from_)
    .bind(&updated.to_city)
    .bind(updated.price)
    .bind(updated.id_drive)
    .bind(updated.client_id)
    .bind(id)
    .execute(&**pool)
    .await
    {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().json(Event {
            id,
            from_: updated.from_.clone(),
            to_city: updated.to_city.clone(),
            price: updated.price,
            id_drive: updated.id_drive,
            client_id: updated.client_id,
        }),
        Ok(_) => HttpResponse::NotFound().body("Event not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/events/{id}")]
pub async fn delete_event(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM events WHERE id = ?")
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Event not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/events/driver/{id_drive}")]
pub async fn get_events_by_driver(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id_drive = path.into_inner();

    match sqlx::query(
        r#"
        SELECT
            e.id,
            e."from" AS "from",
            e.to_city,
            e.price,
            e.id_drive,
            e.client_id,
            d.f_name AS driver_f_name,
            d.l_name AS driver_l_name,
            d.phone AS driver_phone,
            c.f_name AS client_f_name,
            c.l_name AS client_l_name,
            c.phone AS client_phone,
            c.address AS client_address
        FROM events e
        JOIN drivers d ON e.id_drive = d.id
        JOIN clients c ON e.client_id = c.id
        WHERE e.id_drive = ?
        ORDER BY e.id
        "#
    )
    .bind(id_drive)
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter().map(|row| EventWithDriverClient {
                id: row.get("id"),
                from_: row.get("from"),
                to_city: row.get("to_city"),
                price: row.get("price"),
                id_drive: row.get("id_drive"),
                client_id: row.get("client_id"),
                driver_f_name: row.get("driver_f_name"),
                driver_l_name: row.get("driver_l_name"),
                driver_phone: row.get("driver_phone"),
                client_f_name: row.get("client_f_name"),
                client_l_name: row.get("client_l_name"),
                client_phone: row.get("client_phone"),
                client_address: row.get("client_address"),
            }).collect::<Vec<_>>()
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/events/client/{client_id}")]
pub async fn get_events_by_client(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let client_id = path.into_inner();

    match sqlx::query(
        r#"
        SELECT
            e.id,
            e."from" AS "from",
            e.to_city,
            e.price,
            e.id_drive,
            e.client_id,
            d.f_name AS driver_f_name,
            d.l_name AS driver_l_name,
            d.phone AS driver_phone,
            c.f_name AS client_f_name,
            c.l_name AS client_l_name,
            c.phone AS client_phone,
            c.address AS client_address
        FROM events e
        JOIN drivers d ON e.id_drive = d.id
        JOIN clients c ON e.client_id = c.id
        WHERE e.client_id = ?
        ORDER BY e.id
        "#
    )
    .bind(client_id)
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter().map(|row| EventWithDriverClient {
                id: row.get("id"),
                from_: row.get("from"),
                to_city: row.get("to_city"),
                price: row.get("price"),
                id_drive: row.get("id_drive"),
                client_id: row.get("client_id"),
                driver_f_name: row.get("driver_f_name"),
                driver_l_name: row.get("driver_l_name"),
                driver_phone: row.get("driver_phone"),
                client_f_name: row.get("client_f_name"),
                client_l_name: row.get("client_l_name"),
                client_phone: row.get("client_phone"),
                client_address: row.get("client_address"),
            }).collect::<Vec<_>>()
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
