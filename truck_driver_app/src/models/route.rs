use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub driver_id: String,
    pub route_details: String,
}
