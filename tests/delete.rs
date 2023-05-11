use crate::common::helpers::{create_test_rocket, insert_test_message};
use rocket::http::{Header, Status};
use std::fmt::format;
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

mod common;

#[rocket::async_test]
async fn delete_message() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port).await;

    let message = insert_test_message(&server, 1.to_string(), 2.to_string()).await;

    let inserted_message = message
        .headers()
        .get("location")
        .next()
        .expect("Response did not return header");

    let get_by_id = server
        .get(inserted_message)
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    let delete = server.delete(inserted_message).dispatch().await;

    let get_message_again = server
        .get(inserted_message)
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    assert_eq!(message.status(), Status::Created);
    assert_eq!(get_by_id.status(), Status::Ok);
    assert_eq!(delete.status(), Status::Ok);
    assert_eq!(get_message_again.status(), Status::NoContent);
}
