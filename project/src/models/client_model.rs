use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: i64,
    pub f_name: String,
    pub l_name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub f_name: String,
    pub l_name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientRequest {
    pub f_name: String,
    pub l_name: String,
    pub address: String,
    pub phone: String,
}
