use ehttp::Request;
use serde::Deserialize;

use crate::{auth::AuthManager, requester::Requester};

#[derive(Debug, Clone)]
pub struct Client<T> {
    pub base_url: String,
    pub auth_token: Option<String>,
    pub users_collection: String,
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

    pub fn auth(&self) -> AuthManager<T> {
        AuthManager::<T> { client: &self }
    }

    pub fn health_check(&self) -> Request {
        let url = format!("{}/api/health", self.base_url);
        Requester::get(self, url)
    }

    pub fn records(&self, collection: impl ToString) -> Request {
        let url = format!(
            "{}/api/collections/{}/records",
            self.base_url,
            collection.to_string()
        );

        Requester::get(self, url)
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
