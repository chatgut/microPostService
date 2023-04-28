#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_check)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
