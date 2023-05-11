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
) -> Result<Json<Vec<UserID>>, Status> {
    let conversations = db
        .database("postservice")
        .collection::<Message>("messages")
        .distinct("to", doc! { "from": user_id.as_ref() }, None)
        .await;

    let mut users = vec![];

    for conversation in conversations.expect("No conversations found") {
        let user: UserID = bson::from_bson(conversation).unwrap();
        users.push(user)
    }

    Ok(Json(users))
}
