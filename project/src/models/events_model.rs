use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    #[serde(rename = "from")]
    pub from_: String,
    pub to_city: String,
    pub price: i64,
    pub id_drive: i64,
    pub client_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    #[serde(rename = "from")]
    pub from_: String,
    pub to_city: String,
    pub price: i64,
    pub id_drive: i64,
    pub client_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    #[serde(rename = "from")]
    pub from_: String,
    pub to_city: String,
    pub price: i64,
    pub id_drive: i64,
    pub client_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventWithDriverClient {
    pub id: i64,
    #[serde(rename = "from")]
    pub from_: String,
    pub to_city: String,
    pub price: i64,
    pub id_drive: i64,
    pub client_id: i64,
    pub driver_f_name: String,
    pub driver_l_name: String,
    pub driver_phone: String,
    pub client_f_name: String,
    pub client_l_name: String,
    pub client_phone: String,
    pub client_address: String,
}
