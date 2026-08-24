#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    pub id: String,
    pub name: String,
    pub bytes: Vec<u8>,
    pub last_updated: String,
}
