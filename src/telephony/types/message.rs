use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub enum MessageType {
    Text,
    Image,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Uuid,
    pub message_type: MessageType,
    pub sent_at: DateTime<Utc>,
    pub sender_id: Uuid,
    pub value: String,
    pub read: bool,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Message", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("message_type", &self.message_type)?;
        state.serialize_field("sent_at", &self.sent_at.to_string())?;
        state.serialize_field("sender_id", &self.sender_id)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Message {
    pub fn new(
        sender_id: Uuid,
        value: String,
        message_type: MessageType,
        sent_at: DateTime<Utc>,
    ) -> Result<Message, String> {
        Ok(Message {
            id: Uuid::new_v4(),
            message_type,
            sent_at,
            sender_id,
            value,
            read: false,
        })
    }

    pub fn read(&self) {
        self.read = true;
    }
}
