use server::IncomingHook;
use payload::OutgoingPayload;

pub trait OutgoingCallback {
    fn callback(&mut self, server: &IncomingHook, payload: OutgoingPayload);
}
