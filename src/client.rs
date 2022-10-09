mod http_client;
use crate::types::{NotionResponse, User};
use serde::{Deserialize, Serialize};

const NOTION_API_VERSION: &str = "2022-06-28";

pub struct NotionClient<'a> {
    http_client: http_client::HTTPClient<'a>,
    version: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotionAPIError {
    status_code: u32,
    message: String,
}

impl From<reqwest::Error> for NotionAPIError {
    fn from(error: reqwest::Error) -> Self {
        dbg!(error);
        Self {
            status_code: 400,
            message: "Notion API error".to_string(),
        }
    }
}

impl<'a> NotionClient<'a> {
    pub fn new(token: &'a str) -> Self {
        let http_client = http_client::HTTPClient::new("https://api.notion.com/v1", token)
            .set_content_type("application/json")
            .set_headers(vec![http_client::HTTPHeader::new(
                "Notion-Version",
                NOTION_API_VERSION,
            )]);

        Self {
            http_client,
            version: NOTION_API_VERSION,
        }
    }

    pub fn base_url(&self) -> &str {
        self.http_client.get_base_url()
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn token(&self) -> String {
        self.http_client.get_bearer_token_suffix()
    }

    pub async fn list_users(&self) -> Result<NotionResponse<User>, NotionAPIError> {
        let response = self.http_client.get("/users").send().await?;
        Ok(response.json::<NotionResponse<User>>().await?)
    }
}
