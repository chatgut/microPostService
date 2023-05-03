use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

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
            None => Outcome::Failure((Status::Unauthorized, "No userId in header")),
        }
    }
}
