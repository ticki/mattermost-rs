use payload::IncomingPayload;

use hyper::client::Client;
use hyper::header::{Connection, ContentType, Headers};
use hyper::mime::{Mime, TopLevel, SubLevel};

use serde_json;

#[derive(Serialize)]
pub struct IncomingPayload {
    text: String,
}


impl IncomingPayload {
    pub fn new(text: String) -> IncomingPayload {
        IncomingPayload {
            text: text,
        }
    }

    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self);
        json.expect("Fatal error in serialization")
    }
}

pub struct IncomingHook {
    url: &'static str,
}

impl IncomingHook {
    pub fn new(url: &'static str) -> IncomingHook {
        IncomingHook {
            url: url,
        }
    }

    pub fn send(&self, payload: IncomingPayload) {
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

        let client = Client::new();
        client.post(self.url)
            .header(Connection::close())
            .headers(headers)
            .body(payload.to_json().as_bytes())
            .send().expect("Request failed");
    }
}
