use crate::common::helpers::create_test_rocket;
use micro_post_service::endpoints::new_message::rocket_uri_macro_new_message;
use micro_post_service::models::new_message::NewMessage;
use rocket::http::ext::IntoCollection;
use rocket::http::{Header, Status};
use rocket::uri;
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

mod common;

#[test]
fn get_by_id_with_invalid_id_returns_204() {
    let server = create_test_rocket(123);

    let response = server
        .get("/message/234")
        .header(Header::new("userID", " "))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent)
}

#[test]
fn get_by_id_with_valid_id_returns_200() {
    let docker = clients::Cli::docker();
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port);

    let message = NewMessage {
        to: 2.to_string(),
        message: "Test".to_string(),
    };

    let insert_message = server
        .post(uri!(new_message))
        .json(&message)
        .header(Header::new("userID", "1"))
        .dispatch();

    let id = insert_message
        .headers()
        .get("location")
        .next()
        .expect("Response didnt return location header");

    let get_by_id = server.get(id).header(Header::new("userID", "1")).dispatch();

    assert_eq!(insert_message.status(), Status::Created);
    assert_eq!(get_by_id.status(), Status::Ok);
}
