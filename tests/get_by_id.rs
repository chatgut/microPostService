use crate::common::helpers::{create_test_rocket, insert_test_message};
use rocket::http::{Header, Status};
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;

mod common;

#[rocket::async_test]
async fn get_by_id_with_invalid_id_returns_204() {
    let server = create_test_rocket(123).await;

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
    let node = docker.run(Mongo);

    let mongo_port = node.get_host_port_ipv4(27017);
    let server = create_test_rocket(mongo_port).await;

    let insert_response = insert_test_message(&server, 2.to_string(), 1.to_string()).await;
    let id = insert_response
        .headers()
        .get("location")
        .next()
        .expect("Response didnt return location header");

    let get_by_id = server
        .get(id)
        .header(Header::new("userID", "1"))
        .dispatch()
        .await;

    assert_eq!(insert_response.status(), Status::Created);
    assert_eq!(get_by_id.status(), Status::Ok);
}
