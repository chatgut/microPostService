use crate::models::new_message::NewMessage;
use crate::models::user_id::UserID;
use chrono::{DateTime, Utc};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub from: String,
    pub to: String,
    pub message: String,
    pub date: DateTime<Utc>,
}

impl Message {
    pub fn new(new_message: NewMessage, id: UserID) -> Self {
        Self {
            id: None,
            from: id.as_ref().clone(),
            to: new_message.to,
            message: new_message.message,
            date: Utc::now(),
        }
    }
}
