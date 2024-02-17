use ehttp::Request;
use serde_json::json;

use crate::{
    client::{Client, User},
    requester::Requester,
};

pub struct AuthManager<'a, T> {
    pub client: &'a Client<T>,
}
impl<'a, T> AuthManager<'a, T> {
    pub fn login(&self, identifier: &str, password: &str) -> Request {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.client.base_url, self.client.users_collection
        );

        let auth_payload = json!({
            "identity": identifier,
            "password": password
        });

        Requester::post(self.client, url, auth_payload)
    }

    pub fn get_avatar(&self, user: &User) -> Option<Request> {
        if user.avatar.is_empty() {
            return None;
        }
        let url = format!(
            "{}/api/files/{}/{}/{}",
            self.client.base_url, self.client.users_collection, user.id, user.avatar
        );

        Some(Requester::get(self.client, url))
    }
}
