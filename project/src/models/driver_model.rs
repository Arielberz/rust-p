use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Driver {
    pub id: i64,
    pub f_name: String,
    pub l_name: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDriverRequest {
    pub f_name: String,
    pub l_name: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDriverRequest {
    pub f_name: String,
    pub l_name: String,
    pub phone: String,
}
