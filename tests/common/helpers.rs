use micro_post_service::connections::db_connection::MessagesDatabase;
use micro_post_service::connections::rabbitmq::RabbitConnection;
use micro_post_service::endpoints::new_message::rocket_uri_macro_new_message;
use rocket::http::Header;
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket::{routes, uri};
use rocket_db_pools::{Config, Database};

use micro_post_service::endpoints::conversations::get_conversations;
use micro_post_service::endpoints::delete::delete;
use micro_post_service::endpoints::get_by_id::get_by_id;
use micro_post_service::endpoints::get_chat::get_chat_messages;
use micro_post_service::endpoints::health_check::health_check;
use micro_post_service::endpoints::new_message::new_message;
use micro_post_service::models::new_message::NewMessage;

pub async fn create_test_rocket(db_port: u16) -> Client {
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
        .manage(RabbitConnection::init().await)
        .mount(
            "/",
            routes![
                health_check,
                new_message,
                get_by_id,
                get_chat_messages,
                get_conversations,
                delete
            ],
        );

    Client::tracked(rocket)
        .await
        .expect("valid rocket instance")
}

pub async fn insert_test_message(server: &Client, from: String, to: String) -> LocalResponse {
    let test_message = test_message(to);

    server
        .post(uri!(new_message))
        .json(&test_message)
        .header(Header::new("userID", from))
        .dispatch()
        .await
}

pub fn test_message(to: String) -> NewMessage {
    NewMessage {
        to,
        message: "Test message".to_string(),
    }
}
