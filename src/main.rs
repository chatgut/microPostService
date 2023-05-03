#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

use crate::db_connection::MessagesDatabase;

use endpoints::chat::get_chat_messages;
use endpoints::get_by_id::get_by_id;
use endpoints::get_message::get_message;
use endpoints::health_check::health_check;
use endpoints::new_message::new_message;

mod db_connection;
mod endpoints;
mod models;

#[launch]
pub fn rocket() -> _ {
    rocket::build().attach(MessagesDatabase::init()).mount(
        "/",
        routes![
            health_check,
            new_message,
            get_by_id,
            get_message,
            get_chat_messages
        ],
    )
}
