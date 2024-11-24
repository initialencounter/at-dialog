use std::fmt;

use serde_derive::{Deserialize, Serialize};
/// AtMessage
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtMessage {
    pub font: i64,
    pub group_id: i64,
    pub message: Vec<Message>,
    pub message_format: String,
    pub message_id: i64,
    pub message_seq: i64,
    pub message_type: String,
    pub post_type: String,
    pub raw_message: String,
    pub real_id: i64,
    pub self_id: i64,
    pub sender: Sender,
    pub sub_type: String,
    pub time: i64,
    pub user_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub data: Option<MessageData>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageData {
    pub text: Option<String>,
    pub qq: Option<String>,
    pub emoji_id: Option<String>,
    pub emoji_package_id: Option<i64>,
    pub file: Option<String>,
    pub file_id: Option<String>,
    pub file_unique: Option<String>,
    pub key: Option<String>,
    pub path: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    At,
    Text,
    Image,
}
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::At => write!(f, "at"),
            MessageType::Text => write!(f, "text"),
            MessageType::Image => write!(f, "image"),
        }
    }
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "at" => MessageType::At,
            "text" => MessageType::Text,
            "image" => MessageType::Image,
            _ => MessageType::Text,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sender {
    pub card: String,
    pub nickname: String,
    pub role: String,
    pub user_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginInfo {
    pub status: String,
    pub retcode: i64,
    pub data: LoginData,
    pub echo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginData {
    pub nickname: String,
    pub user_id: i64,
}

/// SendMessage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMessage {
    pub group_id: Option<String>,
    pub message: Vec<SendMessageMessage>,
    pub message_type: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMessageMessage {
    pub data: Option<SendMessageData>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMessageData {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupInfo {
    pub group_id: i64,
    pub no_cache: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OneBotRequest {
    pub action: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupInfoResponse {
    pub data: GetGroupInfoData,
    pub echo: Option<serde_json::Value>,
    pub message: String,
    pub retcode: i64,
    pub status: String,
    pub wording: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupInfoData {
    pub group_id: i64,
    pub group_name: String,
    pub max_member_count: i64,
    pub member_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoginInfoResponse {
    pub data: GetLoginInfoData,
    pub echo: Option<serde_json::Value>,
    pub message: String,
    pub retcode: i64,
    pub status: String,
    pub wording: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoginInfoData {
    pub nickname: String,
    pub user_id: i64,
}
