use rocket::http::Status;

#[get("/health_check")]
pub fn health_check() -> Status {
    Status::Ok
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::rocket;

    #[test]
    fn health_check_return_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_check)).dispatch();
        assert_eq!(response.status(), Status::Ok)
    }
}
