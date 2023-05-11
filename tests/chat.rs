mod common;

use crate::common::helpers::*;
use rocket::http::hyper::body::Buf;
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;
use testcontainers::images::rabbitmq::RabbitMq;

#[rocket::async_test]
async fn get_on_chat_with_empty_user_id_returns_401() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let response = server
        .get("/posts?to=2")
        .header(Header::new("userID", " "))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Unauthorized)
}

#[rocket::async_test]
async fn get_on_chat_without_user_id_returns_401() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let response = server.get("/posts?to=2").dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized)
}

#[rocket::async_test]
async fn chat_returns_200_when_getting_inserted_messages_with_query() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let first_message = insert_test_message(&server, 2.to_string(), 1.to_string()).await;
    let second_message = insert_test_message(&server, 2.to_string(), 1.to_string()).await;
    let third_message = insert_test_message(&server, 2.to_string(), 1.to_string()).await;

    let split_header: Vec<&str> = get_message_location(&first_message)
        .split("/posts/")
        .collect();

    let get_message = server
        .get(format!(
            "/posts?to=2&messageId={}&limit=2",
            split_header.get(1).expect("There was no ID in header")
        ))
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    assert_eq!(first_message.status(), Status::Created);
    assert_eq!(second_message.status(), Status::Created);
    assert_eq!(third_message.status(), Status::Created);

    assert_eq!(get_message.status(), Status::Ok);
}

#[rocket::async_test]
async fn chat_with_invalid_query_returns_404() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let response = server
        .get("/posts?to234124")
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound)
}
