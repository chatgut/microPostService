use crate::db_connection::MessagesDatabase;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket_db_pools::Connection;

use crate::models::message::Message;
use rocket::serde::json::Json;

use crate::models::new_message::NewMessage;
use crate::models::user_id::UserID;

#[post("/posts", format = "json", data = "<new_message>")]
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
        "/posts/{}",
        added_message.inserted_id.as_object_id().unwrap()
    )))
}
