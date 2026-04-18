use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub version: String,
}
