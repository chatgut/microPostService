mod common;

use crate::common::helpers::{create_test_rocket, insert_test_message, test_message};
use micro_post_service::endpoints::new_message::rocket_uri_macro_new_message;
use rocket::http::{Header, Status};
use rocket::uri;
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;
use testcontainers::images::rabbitmq::RabbitMq;

#[rocket::async_test]
async fn new_message_return_with_empty_user_id_returns_401() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let message = test_message(2.to_string());

    let response = server
        .post(uri!(new_message))
        .header(Header::new("userID", " "))
        .json(&message)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Unauthorized)
}

#[rocket::async_test]
async fn new_message_return_without_user_id_returns_401() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let message = test_message(2.to_string());

    let response = server
        .post(uri!(new_message))
        .json(&message)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Unauthorized)
}

#[rocket::async_test]
async fn new_message_return_created_and_message_exist_in_database() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let insert_response = insert_test_message(&server, 1.to_string(), 2.to_string()).await;
    let check_db = server
        .get("/posts?to=2")
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    assert_eq!(insert_response.status(), Status::Created);
    assert_eq!(check_db.status(), Status::Ok);
}
