use serde::Serialize;

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Serialize)]
pub struct ClientKey {
    key : [u8;8],   
}

impl ClientKey {
    pub fn new(key : [u8;8]) -> Self {
        Self {
            key : key,
        }
    }

    pub fn get_raw_bytes(&self) -> &[u8] {
        &self.key
    }
}