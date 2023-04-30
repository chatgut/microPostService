use rocket_db_pools::{mongodb::Client, Database};

#[derive(Database)]
#[database("postservice")]
pub struct MessagesDatabase(Client);
