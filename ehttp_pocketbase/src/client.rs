use ehttp::Request;
use serde::Deserialize;
use serde_json::json;

use crate::requester::Requester;

#[derive(Debug, Clone)]
pub struct Client {
    pub base_url: String,
    pub auth_token: Option<String>,
    users_collection: String
}

impl Default for Client {
    fn default() -> Self {
        Client::new("http://127.0.0.1:8090")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSuccessResponse {
    pub token: String,
}



#[derive(Debug, Clone, Deserialize)]
pub struct CollectionsListReponse {
    pub token: String,
}

impl Client {
    pub fn new(base_url: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            users_collection: "users".to_string()
        }
    }

    pub fn health_check(&self) -> Request {
        let url = format!("{}/api/health", self.base_url);
        Requester::get(self, url)
    }

    pub fn auth_with_password(&self, identifier: &str, secret: &str) -> Request {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, self.users_collection
        );

        let auth_payload = json!({
            "identity": identifier,
            "password": secret
        });

        Requester::post(self, url,auth_payload)
    }


}
