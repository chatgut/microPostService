mod common;

use crate::common::helpers::{create_test_rocket, insert_test_message};
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

#[test]
fn get_on_chat_with_empty_user_id_returns_401() {
    let server = create_test_rocket(123);
    let response = server
        .get("/posts?to=2")
        .header(Header::new("userID", " "))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized)
}

#[test]
fn get_on_chat_without_user_id_returns_401() {
    let server = create_test_rocket(123);
    let response = server.get("/posts?to=2").dispatch();
    assert_eq!(response.status(), Status::Unauthorized)
}

#[test]
fn chat_returns_200_when_getting_inserted_messages_with_query() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port);

    let first_message = insert_test_message(&server, 2.to_string(), 1.to_string());
    let second_message = insert_test_message(&server, 2.to_string(), 1.to_string());
    let third_message = insert_test_message(&server, 2.to_string(), 1.to_string());

    let first_message_header = first_message
        .headers()
        .get("location")
        .next()
        .expect("Response didnt return location header");

    let split_header: Vec<&str> = first_message_header.split("/posts/").collect();

    let get_message = server
        .get(format!(
            "/posts?to=2&messageId={}&limit=2",
            split_header.get(1).expect("There was no ID in header")
        ))
        .header(Header::new("userID", "1"))
        .dispatch();

    assert_eq!(first_message.status(), Status::Created);
    assert_eq!(second_message.status(), Status::Created);
    assert_eq!(third_message.status(), Status::Created);

    assert_eq!(get_message.status(), Status::Ok);
}

#[test]
fn chat_with_invalid_query_returns_404() {
    let server = create_test_rocket(123);

    let response = server
        .get("/posts?to234124")
        .header(Header::new("userID", "1"))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound)
}
