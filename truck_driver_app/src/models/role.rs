use serde::{Deserialize};

#[derive(Deserialize)]
pub struct RoleRequest {
    pub user_id: String,
    pub role: String,
}
