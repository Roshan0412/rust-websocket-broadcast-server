use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
}
