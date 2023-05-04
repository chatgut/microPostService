mod common;

use crate::common::helpers::create_test_rocket;
use micro_post_service::endpoints::new_message::rocket_uri_macro_new_message;
use micro_post_service::models::new_message::NewMessage;
use rocket::http::{Header, Status};
use rocket::uri;
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

#[test]
fn new_message_return_with_empty_user_id_returns_401() {
    let server = create_test_rocket(123);

    let message = NewMessage {
        to: 2.to_string(),
        message: "Test".to_string(),
    };

    let response = server
        .post(uri!(new_message))
        .header(Header::new("userID", " "))
        .json(&message)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized)
}

#[test]
fn new_message_return_without_user_id_returns_401() {
    let server = create_test_rocket(123);
    let message = NewMessage {
        to: 2.to_string(),
        message: "Test".to_string(),
    };

    let response = server.post(uri!(new_message)).json(&message).dispatch();
    assert_eq!(response.status(), Status::Unauthorized)
}

#[test]
fn new_message_return_created_and_message_exist_in_database() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port);

    let message = NewMessage {
        to: 2.to_string(),
        message: "Test".to_string(),
    };

    let response = server
        .post(uri!(new_message))
        .json(&message)
        .header(Header::new("userID", "1"))
        .dispatch();
    let check_db = server
        .get("/chat?to=2")
        .header(Header::new("userID", "1"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    assert_eq!(check_db.status(), Status::Ok);
}
