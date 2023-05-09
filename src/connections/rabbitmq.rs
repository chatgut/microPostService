use lapin::{ConnectionProperties, Connection, Queue, types::FieldTable, BasicProperties, Channel};
use lapin::options::*;

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

   pub async fn publish_message(rabbit: &Channel) {
        rabbit.basic_publish("",
                             "messages",
                             BasicPublishOptions::default(),
                             "hellorabbit".as_ref(),
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
