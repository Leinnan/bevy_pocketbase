use crate::client::Client;
use ehttp::Request;
pub struct Requester;

impl Requester {

    /// Adds authorization headers to the request
    fn with_auth(partial_request: ehttp::Request, client: &Client) -> ehttp::Request {
        let mut request = partial_request.clone();
        match client.auth_token.as_ref() {
            Some(token) => {
                request.headers.insert("Authorization", token);
                request
            }
            None => partial_request,
        }
    }

    pub fn get(client: &Client, url: impl ToString) -> Request {
        let request = Requester::with_auth(Request::get(url), client);
        println!("Calling {}", request.url);
        request
    }

    pub fn post(client: &Client, url:impl ToString, body_content: impl ToString) -> Request {
        let mut request = Requester::with_auth(Request::post(url, body_content.to_string().into_bytes()), client);
        request.headers.insert("Content-Type", "application/json");
        request
    }
}
