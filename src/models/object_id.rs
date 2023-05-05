use rocket::request::FromParam;
use rocket_db_pools::mongodb::bson::oid::ObjectId;

pub struct MessageId(ObjectId);

impl AsRef<ObjectId> for MessageId {
    fn as_ref(&self) -> &ObjectId {
        &self.0
    }
}

impl MessageId {
    fn new(id: ObjectId) -> Self {
        Self { 0: id }
    }
}

impl<'r> FromParam<'r> for MessageId {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let obj_id = ObjectId::parse_str(param);

        match obj_id {
            Ok(obj_id) => Ok(MessageId::new(obj_id)),
            Err(_) => return Err("Invalid message ID"),
        }
    }
}
