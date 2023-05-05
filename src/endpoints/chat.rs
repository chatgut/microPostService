use crate::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::user_id::UserID;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::bson::{doc, Document};
use rocket_db_pools::mongodb::options::FindOptions;
use rocket_db_pools::Connection;

#[get("/chat?<to>&<messageId>&<limit>")]
pub async fn get_chat_messages(
    db: Connection<MessagesDatabase>,
    user_id: UserID,
    to: &str,
    messageId: Option<&str>,
    limit: Option<i64>,
) -> Result<Json<Vec<Message>>, Status> {
    let filter = get_filter(user_id, to, messageId);
    let options = get_options(limit);
    let messages = get_messages(db, filter, options).await;

    if messages.is_empty() {
        return Err(Status::NoContent);
    }
    Ok(Json(messages))
}

async fn get_messages(
    db: Connection<MessagesDatabase>,
    filter: Document,
    options: Option<FindOptions>,
) -> Vec<Message> {
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
    messages
}

fn get_filter(user_id: UserID, to: &str, messageId: Option<&str>) -> Document {
    let filter = match messageId {
        None => {
            doc! {
            "from": {"$in": [user_id.as_ref(), to]},
            "to": {"$in": [user_id.as_ref(), to]}}
        }

        Some(messageId) => {
            dbg!("messageId found");
            let obj_id = ObjectId::parse_str(messageId).expect("Error parsing ObjectID");
            doc! {
            "_id": {"$gt": obj_id},
            "from": user_id.as_ref(),
            "to": to }
        }
    };
    filter
}

fn get_options(limit: Option<i64>) -> Option<FindOptions> {
    let newest_first = doc! {"date": - 1};
    let options = match limit {
        Some(limit) => Some(
            FindOptions::builder()
                .sort(newest_first)
                .limit(limit)
                .build(),
        ),
        None => Some(FindOptions::builder().sort(newest_first).build()),
    };
    options
}
