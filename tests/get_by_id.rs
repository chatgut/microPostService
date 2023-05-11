use crate::common::helpers::*;
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;
use testcontainers::images::rabbitmq::RabbitMq;

mod common;

#[rocket::async_test]
async fn get_by_id_with_invalid_id_returns_204() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let response = server
        .get("/posts/234")
        .header(Header::new("userID", " "))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound)
}

#[rocket::async_test]
async fn get_by_id_with_valid_id_returns_200() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let message = insert_test_message(&server, 2.to_string(), 1.to_string()).await;

    let get_by_id = get_message_by_id(&server, get_message_location(&message), 1.to_string()).await;

    assert_eq!(message.status(), Status::Created);
    assert_eq!(get_by_id.status(), Status::Ok);
}
