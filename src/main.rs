#[macro_use]
extern crate rocket;

use micro_post_service::db_connection::MessagesDatabase;
use micro_post_service::endpoints::chat::get_chat_messages;
use micro_post_service::endpoints::get_by_id::get_by_id;
use micro_post_service::endpoints::health_check::health_check;
use micro_post_service::endpoints::new_message::new_message;
use rocket_db_pools::Database;

#[launch]
pub fn rocket() -> _ {
    rocket::build().attach(MessagesDatabase::init()).mount(
        "/",
        routes![health_check, new_message, get_by_id, get_chat_messages],
    )
}
