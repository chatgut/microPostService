use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_db_pools::mongodb::bson::doc;
use crate::connections::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::object_id::MessageId;

#[delete("/posts/<id>")]
pub async fn delete(
    db: Connection<MessagesDatabase>,
    id: MessageId,
) -> Result<Json<u64>, Status> {
    let message = db.database("postservice")
        .collection::<Message>("messages")
        .delete_one(doc! {"id": id.as_ref()}, None)
        .await;

    match message {
        Ok(message) => Ok(Json(message.deleted_count)),
        Err(_) => Err(Status::NoContent)
    }
}
