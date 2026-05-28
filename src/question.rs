pub struct Question {
    name: Vec<u8>,
    type_: u16,
    class: u16,
}

impl Question {
    pub fn new(name: String, type_: u16, class: u16) -> Self {
        Self {
            name: encode_name(name),
            type_,
            class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        result.extend(&self.name);
        result.extend(self.type_.to_be_bytes());
        result.extend(self.class.to_be_bytes());

        result
    }
}

fn encode_name(name: String) -> Vec<u8> {
    let name_vec = name.split(".").collect::<Vec<&str>>();
    let mut encoded_name = Vec::new();

    for name in name_vec {
        encoded_name.push(name.len() as u8);
        encoded_name.extend(name.as_bytes());
    }

    encoded_name.push(0);

    encoded_name
}
