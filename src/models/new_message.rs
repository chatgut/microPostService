use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewMessage {
    pub to: String,
    pub message: String,
}
