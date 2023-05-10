pub mod common;

use crate::common::helpers::create_test_rocket;
use micro_post_service::endpoints::health_check::rocket_uri_macro_health_check;
use rocket::http::Status;
use rocket::uri;

#[rocket::async_test]
async fn health_check_return_ok() {
    let client = create_test_rocket(123).await;
    let response = client.get(uri!(health_check)).dispatch().await;
    assert_eq!(response.status(), Status::Ok)
}
