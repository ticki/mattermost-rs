use std::slice::SliceConcatExt;

use outgoing::OutgoingCallback;
use payload::IncommingPayload;

use hyper::server::Server as HServer;
use hyper::client::Client;
use hyper::header::Connection;

pub struct OutgoingServer<C: OutgoingCallback> {
    callback: C,
    server: HServer,
}

pub struct Server<C: OutgoingCallback> {
    outgoing: Option<OutgoingServer<C>>,
    incomming: IncommingHook,
}

pub struct IncommingHook {
    url: Option<&'static str>,
}

impl IncommingHook {
    pub fn send(&mut self, payload: IncommingPayload) {
        let url = self.url.expect("Client not initialized");

        let client = Client::new();
        let res = client.get(url)
                        .header(Connection::close())
                        .body(
                            format!("payload=\"{}\"", payload.to_json()).as_bytes()
                        ).send().expect("Request failed");

    }
}
