use crate::rsa::keys::Key;

#[derive(Debug)]
pub struct KeyData {
    pub(crate) mode: String,
    pub(crate) comment: String,
    pub(crate) key: Key,
    pub(crate) header: String,
    pub(crate) footer: String,
}

impl KeyData {
    pub fn generate_header_footer(&mut self) {
        self.header = format!("-----BEGIN RSA-RS {} KEY-----", self.mode.to_uppercase());
        self.footer = format!("-----END RSA-RS {} KEY-----", self.mode.to_uppercase());
    }

    pub fn generate_header_footer_bits(&mut self, bits: usize) {
        self.header = format!("-----BEGIN RSA-{} {} KEY-----", bits, self.mode.to_uppercase());
        self.footer = format!("-----END RSA-{} {} KEY-----", bits, self.mode.to_uppercase());
    }

    pub fn new_public(key: Key, comment: String) -> Self {
        Self {
            mode: "PUBLIC ".to_string(),
            comment,
            key,
            header: "".to_string(),
            footer: "".to_string(),
        }
    }

    pub fn new_private(key: Key, comment: String) -> Self {
        Self {
            mode: "PRIVATE".to_string(),
            comment,
            key,
            header: "".to_string(),
            footer: "".to_string(),
        }
    }
}