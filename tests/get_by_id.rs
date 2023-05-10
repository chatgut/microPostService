use crate::common::helpers::{create_test_rocket, insert_test_message_to_user_id_2};
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

mod common;

#[test]
fn get_by_id_with_invalid_id_returns_204() {
    let server = create_test_rocket(123);

    let response = server
        .get("/posts/234")
        .header(Header::new("userID", " "))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound)
}

#[test]
fn get_by_id_with_valid_id_returns_200() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port);

    let insert_response = insert_test_message_to_user_id_2(&server);
    let id = insert_response
        .headers()
        .get("location")
        .next()
        .expect("Response didnt return location header");

    let get_by_id = server.get(id).header(Header::new("userID", "1")).dispatch();

    assert_eq!(insert_response.status(), Status::Created);
    assert_eq!(get_by_id.status(), Status::Ok);
}
