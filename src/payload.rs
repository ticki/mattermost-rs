use serde_json;

#[derive(Serialize)]
pub struct IncomingPayload {
    text: String,
}

pub struct OutgoingPayload;

impl IncomingPayload {
    pub fn new(text: String) -> IncomingPayload {
        IncomingPayload {
            text: text,
        }
    }

    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self);
        match json {
            Ok(data) => data,
            Err(_) => panic!("All IncomingPayloads should be serializable, yet this one failed"),
        }
    }
}
