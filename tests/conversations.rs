use crate::common::helpers::{create_test_rocket, insert_test_message};
use micro_post_service::models::user_id::UserID;
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;
use testcontainers::images::rabbitmq::RabbitMq;

mod common;

#[rocket::async_test]
async fn get_conversations_returns_conversations_from_user_id_header() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    insert_test_message(&server, 1.to_string(), 2.to_string()).await;
    insert_test_message(&server, 1.to_string(), 3.to_string()).await;
    insert_test_message(&server, 5.to_string(), 1.to_string()).await;
    let get_conversations = server
        .get("/posts/conversations")
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    let status = get_conversations.status().clone();
    let users = get_conversations.into_json::<Vec<UserID>>().await.unwrap();
    let user_id_2 = UserID::new(2.to_string());
    let user_id_3 = UserID::new(3.to_string());
    let user_id_5 = UserID::new(5.to_string());

    assert_eq!(status, Status::Ok);
    assert_eq!(users.get(0).unwrap(), &user_id_2);
    assert_eq!(users.get(1).unwrap(), &user_id_3);
    assert_eq!(users.get(2).unwrap(), &user_id_5)
}
