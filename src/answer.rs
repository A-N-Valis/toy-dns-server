use crate::question::encode_name;

pub struct Answer {
    name: Vec<u8>,
    type_: u16,
    class: u16,
    ttl: u32,
    length: u16,
    data: Vec<u8>,
}

impl Answer {
    pub fn new(name: String, type_: u16, class: u16, ttl: u32, length: u16, data: Vec<u8>) -> Self {
        Self {
            name: encode_name(name),
            type_,
            class,
            ttl,
            length,
            data,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(&self.name);
        result.extend(self.type_.to_be_bytes());
        result.extend(self.class.to_be_bytes());
        result.extend(self.ttl.to_be_bytes());
        result.extend(self.length.to_be_bytes());
        result.extend(&self.data);

        result
    }
}
