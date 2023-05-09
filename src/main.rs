#[macro_use]
extern crate rocket;

use std::future::Future;
use lapin::{ConnectionProperties, Connection, Queue, types::FieldTable, BasicProperties, Channel};
use lapin::options::*;
use micro_post_service::db_connection::MessagesDatabase;
use micro_post_service::endpoints::chat::get_chat_messages;
use micro_post_service::endpoints::conversations::get_conversations;
use micro_post_service::endpoints::get_by_id::get_by_id;
use micro_post_service::endpoints::health_check::health_check;
use micro_post_service::endpoints::new_message::new_message;
use micro_post_service::models::cors;
use rocket_db_pools::Database;

#[launch]
pub async fn rocket() -> _ {
    let rabbit = connect_rabbitMQ().await;
    publish_message(rabbit).await;


    rocket::build()
        .attach(MessagesDatabase::init())
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                health_check,
                new_message,
                get_by_id,
                get_chat_messages,
                get_conversations
            ],
        )
}

async fn publish_message(rabbit: Channel) {
    rabbit.basic_publish("",
                         "hello",
                         BasicPublishOptions::default(),
                         "hellorabbit".as_ref(),
                         BasicProperties::default(), ).await.expect("Failed to publish").await.expect("Npe");
}

async fn connect_rabbitMQ() -> Channel {
    // let addr = std::env::var("ROCKET_RABBIT_HOST").expect("Rocket adress not found.");


    let rabbit = Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await.expect("Rabbitmq failed to connect");
    println!("Rabbit connectec");
    let channel = rabbit.create_channel().await.expect("Couldnot create queue");
    channel.queue_declare(
        "hello",
        QueueDeclareOptions::default(),
        FieldTable::default(), )
        .await.expect("Could not create queue");
    channel
}
