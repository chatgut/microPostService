use crate::models::message::Message;
use lapin::options::*;
use lapin::{types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use rocket::serde::json;

pub struct RabbitConnection {
    pub connection: Connection,
    pub channel: Channel,
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
        
        // Declare the fanout exchange
        let exchange = channel
            .exchange_declare(
                "messages",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Could not declare fanout exchange");

        Self {
            connection,
            channel,
        }
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
                "messages", // publish to the exchange instead of the queue
                "", // routing key is ignored for fanout exchanges
                BasicPublishOptions::default(),
                json::serde_json::to_string(&message).unwrap().as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("Failed to publish");
    }
    }
}
