mod common;

#[cfg(test)]
mod test {
    use rocket::http::{Header, Status};
    use crate::common::helpers::create_test_rocket;

    #[test]
    fn get_on_chat_with_empty_user_id_returns_401() {
        let server = create_test_rocket(123);
        let response = server
            .get("/chat?to=2")
            .header(Header::new("userID", " "))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized)
    }

    #[test]
    fn get_on_chat_without_user_id_returns_401() {
        let server = create_test_rocket(123);
        let response = server
            .get("/chat?to=2")
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized)
    }

    // Happy path tested in new message
}
