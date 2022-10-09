pub(super) struct HTTPHeader<'a>(&'a str, &'a str);

impl<'a> HTTPHeader<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self(key, value)
    }
}

pub(super) struct HTTPClient<'a> {
    client: reqwest::Client,
    base_url: &'a str,
    headers: Option<Vec<HTTPHeader<'a>>>,
    bearer_token: &'a str,
    content_type: Option<&'a str>,
}

impl<'a> HTTPClient<'a> {
    pub fn new(base_url: &'a str, bearer_token: &'a str) -> Self {
        let base_url = if base_url.ends_with("/") {
            let mut chars = base_url.chars();
            chars.next_back();
            chars.as_str()
        } else {
            base_url
        };

        Self {
            client: reqwest::Client::new(),
            base_url,
            headers: None,
            bearer_token,
            content_type: None,
        }
    }

    pub fn set_content_type(mut self, content_type: &'a str) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn set_headers(mut self, headers: Vec<HTTPHeader<'a>>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn get<S: AsRef<str>>(&self, path: S) -> reqwest::RequestBuilder {
        let path = path.as_ref();
        let path = if path.starts_with("/") {
            let mut chars = path.chars();
            chars.next();
            chars.as_str()
        } else {
            path
        };

        let mut builder = self.client.get(format!("{}/{path}", self.base_url));

        builder = builder.header(
            "Content-Type",
            self.content_type.unwrap_or("application/json"),
        );

        builder = builder.bearer_auth(self.bearer_token);

        if self.headers.is_some() {
            for header in self.headers.as_ref().unwrap() {
                builder = builder.header(header.0, header.1);
            }
        }

        builder
    }

    pub fn get_base_url(&self) -> &'a str {
        self.base_url.clone()
    }

    pub fn get_bearer_token_suffix(&self) -> String {
        self.bearer_token
            .clone()
            .chars()
            .take(16)
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_runs_successfully() {
        let response = HTTPClient::new("https://example.com", "token")
            .get("/")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }
}
