use hyper::server::{Server, Request, Response, Handler};
use std::io::Read;
use hyper::net::Fresh;
use std::marker::{Sync, Send};

pub struct OutgoingPayload {
    channel_id: String,
    channel_name: String,
    text: String,
    trigger_word: String,
    user_name: String,
    user_id: String,
    timestamp: String,
}

impl OutgoingPayload {
    fn from_str(s: &str) -> OutgoingPayload {
        let mut kv = s.lines().map(|x| {
            &x[..x.len() - 1]
        }).map(|x| {
            let slice = if let Some(x) = x.split('=').next() {
                x
            } else {
                return ("", "");
            };

            (slice, &x[slice.len()..])
        });

        let mut channel_id = "";
        let mut channel_name = "";
        let mut text = "";
        let mut trigger_word = "";
        let mut user_name = "";
        let mut user_id = "";
        let mut timestamp = "";

        for (k, v) in kv {
            match k {
                "channel_id"   => channel_id = v,
                "channel_name" => channel_name = v,
                "text"         => text = v,
                "trigger_word" => trigger_word = v,
                "user_name"    => user_name = v,
                "user_id"      => user_id = v,
                "timestamp"    => timestamp = v,
                _              => {},

            }
        }

        OutgoingPayload {
            channel_id: channel_id.into(),
            channel_name: channel_name.into(),
            text: text.into(),
            trigger_word: trigger_word.into(),
            user_name: user_name.into(),
            user_id: user_id.into(),
            timestamp: timestamp.into(),
        }

    }
}

struct Handle<F>
       where F: Fn(OutgoingPayload) {
    callback: F,
}

unsafe impl<F: Fn(OutgoingPayload)> Send for Handle<F> {}
unsafe impl<F: Fn(OutgoingPayload)> Sync for Handle<F> {}

impl<F> Handler for Handle<F>
        where F: Fn(OutgoingPayload) {
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a, Fresh>) {
        let mut s = String::new();

        req.read_to_string(&mut s);

        self.callback(OutgoingPayload::from_str(&s));
    }
}

pub struct OutgoingHook<F>
    where F: Fn(OutgoingPayload) {

    callback: Option<F>,

}

impl<F> OutgoingHook<F>
        where F: Fn(OutgoingPayload) {

    pub fn on(&mut self, f: F) -> &mut Self {
        self.callback = Some(f);
        self
    }

    pub fn init(&mut self, host: &str) {
        let server = Server::http("127.0.0.1:1337")
                            .expect("Could not start HTTP server");
        let _ = server.handle(Handle {
            callback: self.callback.expect("No callback set"),
        });

    }

}
