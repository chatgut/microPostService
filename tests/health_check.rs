pub mod common;

use crate::common::helpers::create_test_rocket;
use micro_post_service::endpoints::health_check::rocket_uri_macro_health_check;
use rocket::http::Status;
use rocket::uri;
use testcontainers::clients;
use testcontainers::images::mongo::Mongo;
use testcontainers::images::rabbitmq::RabbitMq;

#[rocket::async_test]
async fn health_check_return_ok() {
    let docker = clients::Cli::docker();
    let mongo = docker.run(Mongo);
    let rabbit = docker.run(RabbitMq);

    let mongo_port = mongo.get_host_port_ipv4(27017);
    let rabbit_port = rabbit.get_host_port_ipv4(5672);
    let server = create_test_rocket(mongo_port, rabbit_port).await;

    let response = server.get(uri!(health_check)).dispatch().await;
    assert_eq!(response.status(), Status::Ok)
}
