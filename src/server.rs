use outgoing::OutgoingCallback;

use hyper::server::Server;
use hyper::client::Client;

struct OutgoingServer<C: OutgoingCallback> {
    callback: C,
    server: Server,
}

struct Server {
    outgoing: Option<OutgoingServer>,
    client: Option<Client>,
}

impl Server {
    pub fn send(&mut self, payload: IncommingPayload) {

    }
}
