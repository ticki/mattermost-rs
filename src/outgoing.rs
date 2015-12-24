use server::IncommingHook;
use payload::OutgoingPayload;

pub trait OutgoingCallback {
    fn callback(&mut self, server: &IncommingHook, payload: OutgoingPayload);
}
