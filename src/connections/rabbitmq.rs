use crate::models::message::Message;
use lapin::options::*;
use lapin::{types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties, Queue};
use rocket::serde::json;

pub struct RabbitConnection {
    pub connection: Connection,
    pub channel: Channel,
    pub queue: Queue,
}

impl RabbitConnection {
    pub async fn init() -> RabbitConnection {
        let connection = Connection::connect(
            &std::env::var("ROCKET_RABBIT_HOST").unwrap_or_else(|_| "amqp://localhost:5672".into()),
            ConnectionProperties::default(),
        )
        .await
        .expect("Could not connect to RabbitMQ");
        let channel = connection
            .create_channel()
            .await
            .expect("Could not create channel");
        let queue = Self::create_queue(&channel).await;
        Self {
            connection,
            channel,
            queue,
        }
    }
    async fn create_queue(channel: &Channel) -> Queue {
        channel
            .queue_declare(
                "messages",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Could not create queue")
    }
    pub async fn reconnect(&mut self) {
        self.connection =
            Connection::connect("amqp://localhost:5672", ConnectionProperties::default())
                .await
                .expect("Could not connect to RabbitMQ");
    }

    pub async fn publish_message(rabbit: &RabbitConnection, message: &Message) {
        rabbit
            .channel
            .basic_publish(
                "",
                "messages",
                BasicPublishOptions::default(),
                json::serde_json::to_string(&message).unwrap().as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("Failed to publish");
    }
}
