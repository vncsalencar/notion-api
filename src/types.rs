use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    object: String,
    id: String,
    #[serde(rename = "type")]
    user_type: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
    person: PersonData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonData {
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    object: String,
    id: String,
    #[serde(rename = "type")]
    user_type: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
    bot: BotData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotData {
    workspace_name: String,
    owner: BotOwner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotOwner {
    #[serde(rename = "type")]
    owner_type: String,
    workspace: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum User {
    Person(Person),
    Bot(Bot),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionResponse<T> {
    pub object: String,
    pub results: Vec<T>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
    #[serde(rename = "type")]
    pub results_type: String,
}
