use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SOSRequest {
    pub driver_id: String,
    pub location: String,
}
