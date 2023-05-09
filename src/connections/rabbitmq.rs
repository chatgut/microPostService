use lapin::{ConnectionProperties, Connection, Queue, types::FieldTable, BasicProperties, Channel};
use lapin::options::*;
use rocket::serde::json;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use crate::models::message::Message;

pub struct RabbitConnection(pub Connection);


impl RabbitConnection {
    pub async fn init() -> Self {
        Self {
            0: Connection::connect("amqp://localhost:5672",
                                   ConnectionProperties::default())
                .await.expect("Could not connect to RabbitMQ")
        }
    }

    pub async fn create_channel(connection: &Self, channel: &Channel) -> Queue {
        channel.queue_declare(
            "messages",
            QueueDeclareOptions::default(),
            FieldTable::default(), )
            .await.expect("Could not create queue")
    }

   pub async fn publish_message(rabbit: &Channel, message: &Message) {
        rabbit.basic_publish("",
                             "messages",
                             BasicPublishOptions::default(),
                             json::serde_json::to_string_pretty(&message).unwrap().as_bytes(),
                             BasicProperties::default(), ).await.expect("Failed to publish").await.expect("Npe");
    }

}

// impl Poolable for RabbitConnection{
//     type Manager = (RabbitConnection::Connection::Ma);
//     type Error = ();
//
//     fn pool(config: DatabaseConfig) -> Result<Pool<Self::Manager>, Self::Error> {
//         todo!()
//     }
