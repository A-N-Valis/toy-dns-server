use anyhow::{Context, Result};

pub struct Question {
    pub name: Vec<u8>,
    pub type_: u16,
    pub class: u16,
}

impl Question {
    pub fn new(name: Vec<u8>, type_: u16, class: u16) -> Self {
        Self { name, type_, class }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(&self.name);
        result.extend(self.type_.to_be_bytes());
        result.extend(self.class.to_be_bytes());

        result
    }

    pub fn from_bytes(buf: [u8; 512]) -> Result<Self> {
        let without_header: [u8; 500] = buf[12..].try_into()?;

        let name_len = without_header
            .iter()
            .position(|&x| x == 0)
            .context("missing 0")?;
        let name = without_header[..name_len + 1].to_vec();
        let type_ = u16::from_be_bytes(without_header[name_len + 1..name_len + 3].try_into()?);
        let class = u16::from_be_bytes(without_header[name_len + 3..name_len + 5].try_into()?);

        Ok(Self { name, type_, class })
    }
}
