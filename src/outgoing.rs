trait OutgoingCallback {
    fn callback(&mut self, server: &Server, payload: OutgoingPayload);
}
