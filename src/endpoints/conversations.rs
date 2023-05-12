use std::collections::HashSet;
use crate::connections::db_connection::MessagesDatabase;
use crate::models::message::Message;
use crate::models::user_id::UserID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::bson;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::Connection;

#[get("/posts/conversations")]
pub async fn get_conversations(
    db: Connection<MessagesDatabase>,
    user_id: UserID,
) -> Result<Json<HashSet<UserID>>, Status> {
    let conversations_to = db
        .database("postservice")
        .collection::<Message>("messages")
        .distinct("from", doc! { "to": user_id.as_ref() }, None)
        .await;
    let conversations_from = db
        .database("postservice")
        .collection::<Message>("messages")
        .distinct("to", doc! { "from": user_id.as_ref() }, None)
        .await;

    let mut users = HashSet::new();

    for conversation in conversations_to.expect("No conversations found") {
        let user: UserID = bson::from_bson(conversation).unwrap();
        users.insert(user);
    }
    for conversation in conversations_from.expect("No conversations found") {
        let user: UserID = bson::from_bson(conversation).unwrap();
        users.insert(user);
    }

    Ok(Json(users))
}
