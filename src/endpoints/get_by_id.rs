use crate::db_connection::MessagesDatabase;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::Connection;
use crate::models::message::Message;

#[get("/message/<id>")]
pub async fn get_by_id(
    db: Connection<MessagesDatabase>,
    id: &str,
) -> Result<Json<Message>, Status> {
    let obj_id = ObjectId::parse_str(id);

    match obj_id {
        Ok(obj_id) => obj_id,
        Err(_) => return Err(Status::NoContent),
    };

    let message = db
        .database("postservice")
        .collection::<Message>("messages")
        .find_one(doc! {"_id": obj_id.unwrap()}, None)
        .await;
    match message {
        Ok(message) => Ok(Json(message.expect("Message not found"))),
        Err(_) => Err(Status::NoContent),
    }
}
