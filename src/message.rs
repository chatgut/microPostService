use chrono::{DateTime, Utc};
use rocket::{Request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
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
            from: id.as_ref().clone(),
            to: new_message.to,
            message: new_message.message,
            date: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewMessage {
    pub to: String,
    pub message: String,
}

pub struct UserID(String);

impl AsRef<String> for UserID {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_id = request.headers().get_one("userID");


        match user_id {
            Some(user_id) => {
                return if !user_id.trim().is_empty() {
                    Outcome::Success(UserID(user_id.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, "userID is empty"))
                };
            }
            None => Outcome::Failure((Status::Unauthorized, "No userId in header"))
        }
    }
}


