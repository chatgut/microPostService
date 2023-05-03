use crate::db_connection::MessagesDatabase;
use crate::message::{Message, UserID};
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::Connection;

#[get("/chat?<to>")]
pub async fn get_chat_messages(
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
        .collection::<Message>("messages")
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
