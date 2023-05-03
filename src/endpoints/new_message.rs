use crate::db_connection::MessagesDatabase;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket_db_pools::Connection;

use rocket::serde::json::Json;
use crate::models::message::Message;


use crate::models::new_message::NewMessage;
use crate::models::user_id::UserID;

#[post("/newmessage", format = "json", data = "<new_message>")]
pub async fn new_message(
    db: Connection<MessagesDatabase>,
    new_message: Json<NewMessage>,
    user_id: UserID,
) -> Result<Created<Json<Message>>, Status> {
    let message = Message::new(new_message.into_inner(), user_id);
    let added_message = db
        .database("postservice")
        .collection::<Message>("messages")
        .insert_one(message, None)
        .await
        .expect("Unable to insert message");
    //TODO to and message cannot be empty

    Ok(Created::new(format!(
        "/message/{}",
        added_message.inserted_id.as_object_id().unwrap()
    )))
}

#[cfg(test)]
mod test {
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use testcontainers::{clients};
    use testcontainers::images::mongo::Mongo;

    use crate::endpoints::helpers;



    use crate::models::new_message::NewMessage;
    use crate::rocket;

    #[test]
    fn new_message_return_with_empty_user_id_returns_401() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let message = NewMessage {
            to: 2.to_string(),
            message: "Test".to_string(),
        };

        let response = client
            .post(uri!(super::new_message))
            .header(Header::new("userID", " "))
            .json(&message)
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized)
    }

    #[test]
    fn new_message_return_without_user_id_returns_401() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let message = NewMessage {
            to: 2.to_string(),
            message: "Test".to_string(),
        };

        let response = client
            .post(uri!(super::new_message))
            .json(&message)
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized)
    }


    #[test]
    fn new_message_return_created() {
        let docker = clients::Cli::docker();
        let node = docker.run(Mongo);

        let mongo_port = node.get_host_port_ipv4(27017);
        let server = helpers::create_test_rocket(mongo_port);

        let message = NewMessage { to: 2.to_string(), message: "Test".to_string() };

        let response = server.post(uri!(super::new_message))
            .json(&message).header(Header::new("userID", "1")).dispatch();
        assert_eq!(response.status(), Status::Created)
    }
}
