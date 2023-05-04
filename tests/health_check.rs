mod common;

use crate::common::helpers::create_test_rocket;
use micro_post_service::endpoints::health_check::rocket_uri_macro_health_check;
use rocket::http::Status;
use rocket::uri;

#[test]
fn health_check_return_ok() {
    let client = create_test_rocket(123);
    let response = client.get(uri!(health_check)).dispatch();
    assert_eq!(response.status(), Status::Ok)
}
