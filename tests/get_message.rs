use crate::common::helpers::{create_test_rocket, insert_test_message_to_user_id_2};
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

mod common;

#[test]
fn get_message_with_invalid_query_returns_404() {
    let server = create_test_rocket(123);

    let response = server
        .get("/message?to234124")
        .header(Header::new("userID", "1"))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound)
}
#[test]
fn get_messages_returns_200_when_getting_inserted_messages() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port);

    let first_message = insert_test_message_to_user_id_2(&server);
    let second_message = insert_test_message_to_user_id_2(&server);
    let third_message = insert_test_message_to_user_id_2(&server);

    let first_message_header = first_message
        .headers()
        .get("location")
        .next()
        .expect("Response didnt return location header");

    let split_header: Vec<&str> = first_message_header.split("/message/").collect();

    let get_message = server
        .get(format!(
            "/message?to=2&fromMessageId={}&numberOfMessages=2",
            split_header.get(1).expect("There was no ID in header")
        ))
        .header(Header::new("userID", "1"))
        .dispatch();

    assert_eq!(first_message.status(), Status::Created);
    assert_eq!(second_message.status(), Status::Created);
    assert_eq!(third_message.status(), Status::Created);

    assert_eq!(get_message.status(), Status::Ok);
}
