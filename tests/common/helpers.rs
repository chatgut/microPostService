use micro_post_service::db_connection::MessagesDatabase;
use micro_post_service::endpoints::new_message::rocket_uri_macro_new_message;
use rocket::http::Header;
use rocket::local::blocking::{Client, LocalResponse};
use rocket::{routes, uri};
use rocket_db_pools::{Config, Database};

use micro_post_service::endpoints::chat::get_chat_messages;
use micro_post_service::endpoints::get_by_id::get_by_id;
use micro_post_service::endpoints::health_check::health_check;
use micro_post_service::endpoints::new_message::new_message;
use micro_post_service::models::new_message::NewMessage;

pub fn create_test_rocket(db_port: u16) -> Client {
    let figment = rocket::Config::figment().merge((
        "databases.postservice",
        Config {
            url: format!("mongodb://localhost:{}", db_port).into(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: Some(120),
        },
    ));

    let rocket = rocket::custom(figment)
        .attach(MessagesDatabase::init())
        .mount(
            "/",
            routes![
                health_check,
                new_message,
                get_by_id,
                get_chat_messages
            ],
        );

    Client::tracked(rocket).expect("valid rocket instance")
}

pub fn insert_test_message_to_user_id_2(server: &Client) -> LocalResponse {
    let test_message = test_message_user_id_2();

    server
        .post(uri!(new_message))
        .json(&test_message)
        .header(Header::new("userID", "1"))
        .dispatch()
}

pub fn test_message_user_id_2() -> NewMessage {
    NewMessage {
        to: 2.to_string(),
        message: "Test message".to_string(),
    }
}
