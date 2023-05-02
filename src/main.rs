#[macro_use]
extern crate rocket;

use rocket::futures::{TryStreamExt};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::options::FindOptions;
use rocket_db_pools::{Connection, Database};

use crate::db_connection::MessagesDatabase;
use crate::message::{Message, NewMessage, UserID};

mod db_connection;
mod message;

#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

#[get("/message/chat?<to>")]
async fn get_chat_messages(
    db: Connection<MessagesDatabase>,
    user_id: UserID,
    to: &str,
) -> Result<Json<Vec<Message>>, Status> {
    let filter = doc! {
        "from": {"$in": [user_id.as_ref(), to]},
        "to": {"$in": [user_id.as_ref(), to]}
    };
    let mut cursor = db
        .database("postservice")
        .collection("messages")
        .find(filter, None)
        .await
        .expect("Failed to connect to database");

    let mut messages = vec![];

    while let Some(message) = cursor
        .try_next()
        .await
        .expect("Failed to get message from stream")
    {
        messages.push(message);
    }

    if messages.is_empty() {
        return Err(Status::NoContent);
    }
    Ok(Json(messages))
}

#[get("/message?<to>&<fromMessageId>&<numberOfMessages>")]
async fn get_message(
    db: Connection<MessagesDatabase>,
    user_id: UserID,
    to: &str,
    fromMessageId: &str,
    numberOfMessages: i64,
) -> Result<Json<Vec<Message>>, Status> {
    //-> Result<Json<Message>, Status> {

    let obj_id = ObjectId::parse_str(fromMessageId).expect("Error parsing ObjectID");

    let filter = doc! { "_id": {"$gt": obj_id}, "from": user_id.as_ref(), "to": to };

    let options = FindOptions::builder().limit(numberOfMessages).build();

    let mut cursor = db
        .database("postservice")
        .collection::<Message>("messages")
        .find(filter, options)
        .await
        .expect("Failed to connect to database");

    let mut messages = vec![];

    while let Some(message) = cursor
        .try_next()
        .await
        .expect("Failed to get message from stream")
    {
        messages.push(message);
    }
    if messages.is_empty() {
        return Err(Status::NoContent);
    }
    Ok(Json(messages))
}

#[get("/message/<id>")]
async fn get_by_id(db: Connection<MessagesDatabase>, id: &str) -> Result<Json<Message>, Status> {
    let obj_id = ObjectId::parse_str(id);

    match obj_id {
        Ok(obj_id) => obj_id,
        Err(_) => return Err(Status::NoContent),
    };

    let message = db
        .database("postservice")
        .collection("messages")
        .find_one(doc! {"_id": obj_id.unwrap()}, None)
        .await;
    match message {
        Ok(message) => Ok(Json(message.expect("Message not found"))),
        Err(_) => Err(Status::NoContent),
    }
}

#[post("/newmessage", format = "json", data = "<new_message>")]
async fn new_message(
    db: Connection<MessagesDatabase>,
    new_message: Json<NewMessage>,
    user_id: UserID,
) -> Result<Created<Json<Message>>, Status> {
    let message = Message::new(new_message.into_inner(), user_id);
    let added_message = db
        .database("postservice")
        .collection("messages")
        .insert_one(message, None)
        .await
        .expect("Unable to insert message");
    //TODO to and message cannot be empty

    Ok(Created::new(format!(
        "/message/{}",
        added_message.inserted_id.as_object_id().unwrap()
    )))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(MessagesDatabase::init()).mount(
        "/",
        routes![
            health_check,
            new_message,
            get_by_id,
            get_message,
            get_chat_messages
        ],
    )
}

#[cfg(test)]
mod test {
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    use crate::message::NewMessage;

    use super::rocket;

    #[test]
    fn health_check_return_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_check)).dispatch();
        assert_eq!(response.status(), Status::Ok)
    }

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

    //TODO fix test
    // #[test]
    // fn new_message_return_created() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let message = NewMessage { to: 2.to_string(), message: "Test".to_string() };
    //
    //     let response = client.post(uri!(super::new_message))
    //         .json(&message).header(Header::new("userID", "1")).dispatch();
    //     assert_eq!(response.status(), Status::Created)
    // }
}
