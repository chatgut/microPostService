mod message;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use crate::message::{Message, NewMessage, UserID};

#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

#[post("/newmessage", format = "json", data = "<new_message>")]
fn new_message(new_message: Json<NewMessage>, user_id: UserID) -> Created<Json<Message>> {
    let message = Message::new(new_message.into_inner(), user_id);

    //TODO to and message cannot be empty

    println!("{:?}", message);
    Created::new("").body(Json(message))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check, new_message])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Header, Status};
    use crate::message::{NewMessage};

    #[test]
    fn health_check_return_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_check)).dispatch();
        assert_eq!(response.status(), Status::Ok)
    }


    #[test]
    fn new_message_return_without_user_id_returns_401() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let message = NewMessage { to: 2.to_string(), message: "Test".to_string() };

        let response = client.post(uri!(super::new_message))
            .json(&message).dispatch();
        assert_eq!(response.status(), Status::Unauthorized)
    }

    #[test]
    fn new_message_return_created() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let message = NewMessage { to: 2.to_string(), message: "Test".to_string() };

        let response = client.post(uri!(super::new_message))
            .json(&message).header(Header::new("userID", "1")).dispatch();
        assert_eq!(response.status(), Status::Created)
    }
}
