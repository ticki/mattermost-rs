use std::slice::SliceConcatExt;
use std::io::Read;

use outgoing::OutgoingCallback;
use payload::IncomingPayload;

use hyper::server::Server as HServer;
use hyper::client::Client;
use hyper::header::{Connection, ContentType};

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

        let client = Client::new();
        let mut res = client.post(url)
                        .header(Connection::close())
                        .header(ContentType::json())
                        .body(
                            format!("{}", payload.to_json()).as_bytes()
                        ).send().expect("Request failed");
        let mut message = String::new();
        res.read_to_string(&mut message);
        println!("{}", payload.to_json());

    }
}
