use crate::connections::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::object_id::MessageId;
use rocket::http::Status;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::Connection;

#[delete("/posts/<id>")]
pub async fn delete(db: Connection<MessagesDatabase>, id: MessageId) -> Result<(), Status> {
    let message = db
        .database("postservice")
        .collection::<Message>("messages")
        .delete_one(doc! {"_id": id.as_ref()}, None)
        .await;

    match message {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::NoContent),
    }
}
