use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::time::error::InvalidVariant;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Clone, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub message: String,
    pub date: DateTime<Utc>,
}

impl Message {
    pub fn new(new_message: NewMessage, id: UserID) -> Self {
        Self {
            from: id.0,
            to: new_message.to,
            message: new_message.message,
            date: Utc::now(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct NewMessage {
    pub to: String,
    pub message: String,
}

pub struct UserID(String);

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = InvalidVariant;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_id = request.headers().get_one("userID");

        match user_id {
            Some(user_id) => Outcome::Success(UserID(user_id.to_string())),
            None => Outcome::Failure((Status::Unauthorized, InvalidVariant))
        }
    }
}


