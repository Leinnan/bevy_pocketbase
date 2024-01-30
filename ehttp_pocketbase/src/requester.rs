use crate::client::Client;
use ehttp::Request;
use std::sync::mpsc::channel;

pub struct Requester;

impl Requester {
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
    pub fn get(client: &Client, url: impl ToString) -> Result<ehttp::Response, String> {
        let mut request = Request::get(url);
        let (sender, receiver) = channel();
        request = Requester::with_auth(request, client);
        println!("Calling {}", request.url);
        ehttp::fetch(request, move |response| {
            sender.send(response.clone()).unwrap();
        });
        receiver.recv().unwrap()
    }
}
