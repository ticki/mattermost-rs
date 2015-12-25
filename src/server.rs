use std::slice::SliceConcatExt;
use std::io::Read;

use outgoing::OutgoingCallback;
use payload::IncomingPayload;

use hyper::server::Server as HServer;
use hyper::client::Client;
use hyper::header::{Connection, ContentType, Headers};
use hyper::mime::{Mime, TopLevel, SubLevel};

pub struct OutgoingServer<C: OutgoingCallback> {
    callback: C,
    server: HServer,
}

pub struct Server<C: OutgoingCallback> {
    outgoing: Option<OutgoingServer<C>>,
    incoming: IncomingHook,
}

pub struct IncomingHook {
    url: Option<&'static str>,
}

impl IncomingHook {
    pub fn new(url: &'static str) -> IncomingHook {
        IncomingHook {
            url: Some(url),
        }
    }
    pub fn send(&self, payload: IncomingPayload) {
        let url = self.url.expect("Client not initialized");

        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

        let client = Client::new();
        let mut res = client.post(url)
                        .header(Connection::close())
                        .headers(headers)
                        .body(
                            payload.to_json().as_bytes()
                        ).send().expect("Request failed");
    }
}
