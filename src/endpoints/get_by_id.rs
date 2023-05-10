use crate::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::object_id::MessageId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::Connection;

#[get("/posts/<id>")]
pub async fn get_by_id(
    db: Connection<MessagesDatabase>,
    id: MessageId,
) -> Result<Json<Message>, Status> {
    let message = db
        .database("postservice")
        .collection::<Message>("messages")
        .find_one(doc! {"_id": id.as_ref()}, None)
        .await;
    match message {
        Ok(message) => Ok(Json(message.expect("Message not found"))),
        Err(_) => Err(Status::NoContent),
    }
}
