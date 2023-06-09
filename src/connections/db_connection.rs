use rocket_db_pools::mongodb::Collection;
use rocket_db_pools::{mongodb::Client, Database};

use crate::models::message::Message;

#[derive(Database)]
#[database("postservice")]
pub struct MessagesDatabase(Client);

impl MessagesDatabase {
    pub fn collection(&self) -> Collection<Message> {
        self.database("postservice").collection("messages")
    }
}
