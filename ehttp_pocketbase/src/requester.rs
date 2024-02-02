use crate::client::RequesterInfo;
use ehttp::Request;
pub struct Requester;

impl Requester {
    /// Adds authorization headers to the request
    fn with_auth(partial_request: ehttp::Request, client: &impl RequesterInfo) -> ehttp::Request {
        let mut request = partial_request.clone();
        match &client.get_token() {
            Some(token) => {
                request.headers.insert("Authorization", token);
                request
            }
            None => partial_request,
        }
    }

    pub fn get(client: &impl RequesterInfo, url: impl ToString) -> Request {
        Requester::with_auth(Request::get(url), client)
    }

    pub fn post(
        client: &impl RequesterInfo,
        url: impl ToString,
        body_content: impl ToString,
    ) -> Request {
        let mut request = Requester::with_auth(
            Request::post(url, body_content.to_string().into_bytes()),
            client,
        );
        request.headers.insert("Content-Type", "application/json");
        request
    }
}
