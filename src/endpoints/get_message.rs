use crate::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::user_id::UserID;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::options::FindOptions;
use rocket_db_pools::Connection;

#[get("/message?<to>&<fromMessageId>&<numberOfMessages>")]
pub async fn get_message(
    db: Connection<MessagesDatabase>,
    user_id: UserID,
    to: &str,
    fromMessageId: &str,
    numberOfMessages: i64,
) -> Result<Json<Vec<Message>>, Status> {
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
