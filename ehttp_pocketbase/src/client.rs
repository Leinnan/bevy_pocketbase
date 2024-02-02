use ehttp::Request;
use serde::Deserialize;
use serde_json::json;

use crate::requester::Requester;

#[derive(Debug, Clone)]
pub struct Client<T> {
    pub base_url: String,
    pub auth_token: Option<String>,
    users_collection: String,
    pub user: Option<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
    pub id: String,
}

impl<T> Default for Client<T> {
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
pub struct AuthSuccessResponse<T> {
    pub token: String,
    pub record: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionsListReponse {
    pub token: String,
}

impl<T> Client<T> {
    pub fn new(base_url: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            users_collection: "users".to_string(),
            user: None,
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

        Requester::post(self, url, auth_payload)
    }

    pub fn records(&self, collection: impl ToString) -> Request {
        let url = format!(
            "{}/api/collections/{}/records",
            self.base_url,
            collection.to_string()
        );

        Requester::get(self, url)
    }

    pub fn get_avatar(&self, user: &User) -> Option<Request> {
        if user.avatar.is_empty() {
            return None;
        }
        let url = format!(
            "{}/api/files/{}/{}/{}",
            self.base_url, self.users_collection, user.id, user.avatar
        );

        Some(Requester::get(self, url))
    }
}

impl<T> RequesterInfo for Client<T> {
    fn get_base_url(&self) -> &String {
        &self.base_url
    }

    fn get_token(&self) -> &Option<String> {
        &self.auth_token
    }
}

pub trait RequesterInfo {
    fn get_base_url(&self) -> &String;
    fn get_token(&self) -> &Option<String>;
}
