use serde::Deserialize;

use crate::requester::Requester;


#[derive(Debug, Clone)]
pub struct Client {
    pub base_url: String,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckResponse {
    pub code: i32,
    pub message: String,
}

impl Client{
    pub fn new(base_url: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
        }
    }
    pub fn health_check(&self) {
        let url = format!("{}/api/health", self.base_url);
        Requester::get(self, url);
    }
}