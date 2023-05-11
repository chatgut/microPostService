#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use micro_post_service::connections::db_connection::MessagesDatabase;
use micro_post_service::connections::rabbitmq::RabbitConnection;
use micro_post_service::endpoints::conversations::get_conversations;
use micro_post_service::endpoints::delete::delete;
use micro_post_service::endpoints::get_by_id::get_by_id;
use micro_post_service::endpoints::get_chat::get_chat_messages;
use micro_post_service::endpoints::health_check::health_check;
use micro_post_service::endpoints::new_message::new_message;
use micro_post_service::models::cors;
use rocket_db_pools::Database;

#[launch]
pub async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(MessagesDatabase::init())
        .manage(RabbitConnection::init().await)
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                health_check,
                new_message,
                get_by_id,
                get_chat_messages,
                get_conversations,
                delete,
            ],
        )
}
