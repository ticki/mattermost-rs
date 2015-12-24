use rustc_serialize::json;

#[derive(RustcEncodable)]
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
        let json = json::encode(&self);
        match json {
            Ok(data) => data,
            Err(_) => "{}".to_string(),
        }
    }
}
