use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub name: String,
    pub email: String,
}
