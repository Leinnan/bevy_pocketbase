use ehttp::Request;

use crate::client::Client;


pub struct Requester;

impl Requester {
    fn with_auth(partial_request: ehttp::Request,client: &Client) -> ehttp::Request {
        let mut request = partial_request.clone();
        match client.auth_token.as_ref() {
            Some(token) => {
                request.headers.insert("Authorization", token);
                request
            }
            None => partial_request
        }
    }
    pub fn get(client: &Client, url : impl ToString) {
        let mut request = Request::get(url);

        request = Requester::with_auth(request,client);
        ehttp::fetch(request, move |response| {
            println!("{:?}", response);
        })
    }
}