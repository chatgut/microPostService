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
    use rocket::http::Status;
    use crate::message::{Message, UserID};

    #[test]
    fn health_check_return_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_check)).dispatch();
        assert_eq!(response.status(), Status::Ok)
    }


    // #[test]
    // fn new_message_return_created() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let response = client.post(uri!(super::new_message)).dispatch();
    //     assert_eq!(response.status(), Status::Created)
    // }
}
