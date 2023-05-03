use rocket::local::blocking::Client;
use rocket_db_pools::{Config, Database};
use testcontainers::{clients, Container};
use testcontainers::images::mongo::Mongo;

use crate::db_connection::MessagesDatabase;
use crate::endpoints::chat::get_chat_messages;
use crate::endpoints::get_by_id::get_by_id;
use crate::endpoints::get_message::get_message;
use crate::endpoints::health_check::health_check;
use crate::endpoints::new_message::new_message;

pub fn create_test_rocket(db_port: u16) -> Client {
    let figment = rocket::Config::figment()
        .merge(("databases.postservice", Config {
            url: format!("mongodb://localhost:{}", db_port).into(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: Some(120),
        }
        ));

    let rocket = rocket::custom(figment)
        .attach(MessagesDatabase::init()).mount(
        "/",
        routes![
            health_check,
            new_message,
            get_by_id,
            get_message,
            get_chat_messages
        ],
    );

    Client::tracked(rocket).expect("valid rocket instance")
}
