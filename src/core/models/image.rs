use crate::core::encoder::base64_encode;

#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    pub id: String,
    pub name: String,
    pub bytes: Vec<u8>,
    pub last_updated: String,
}

impl ImageData {
    /// Formats raw bytes as a browser-compatible Data URI scheme
    pub fn to_data_uri(&self) -> String {
        let encoded = base64_encode(&self.bytes);
        format!("data:image/png;base64,{encoded}")
    }
}
