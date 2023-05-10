use lapin::{ConnectionProperties, Connection, Queue, types::FieldTable, BasicProperties, Channel};
use lapin::options::*;
use rocket::serde::json;
use crate::models::message::Message;

pub struct RabbitConnection {
    pub connection: Connection,
    pub channel: Channel,
    pub queue: Queue,
}

impl RabbitConnection {
    pub async fn init() -> RabbitConnection {
        let connection = Connection::connect("amqp://localhost:5672",
                                             ConnectionProperties::default())
            .await.expect("Could not connect to RabbitMQ");
        let channel = connection.create_channel().await.expect("Could not create channel");
        let queue = Self::create_queue(&channel).await;
        Self {
            connection,
            channel,
            queue,
        }
    }
    async fn create_queue(channel: &Channel) -> Queue {
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
                             BasicProperties::default(), ).await.expect("Failed to publish");
    }
}
