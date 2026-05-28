use anyhow::{Context, Result};

pub struct Question {
    pub name: Vec<u8>,
    pub type_: u16,
    pub class: u16,
}

impl Question {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(&self.name);
        result.extend(self.type_.to_be_bytes());
        result.extend(self.class.to_be_bytes());

        result
    }

    pub fn from_bytes(buf: &[u8], qdcount: u16) -> Result<Vec<Question>> {
        let mut question_vec = Vec::new();
        let mut current_pos = 12;

        for _ in 0..qdcount {
            let (name, next_pos) = parse_name(buf, current_pos)?;
            let type_ = u16::from_be_bytes(buf[next_pos..next_pos + 2].try_into()?);
            let class = u16::from_be_bytes(buf[next_pos + 2..next_pos + 4].try_into()?);

            question_vec.push(Self { name, type_, class });

            current_pos = next_pos + 4;
        }

        Ok(question_vec)
    }
}

pub fn parse_name(buf: &[u8], start_pos: usize) -> Result<(Vec<u8>, usize)> {
    let mut name_vec = Vec::new();
    let mut current_pos = start_pos;
    let mut next_pos = None;
    let mut has_jumped = false;

    loop {
        let current_byte = buf[current_pos];
        let is_pointer = current_byte & 0xC0 == 0xC0;

        if current_byte == 0 {
            name_vec.push(0);

            if !has_jumped {
                next_pos = Some(current_pos + 1);
            }

            break;
        } else if is_pointer {
            let pointer = u16::from_be_bytes(buf[current_pos..current_pos + 2].try_into()?);
            let offset = pointer & 0x3FFF;

            if !has_jumped {
                next_pos = Some(current_pos + 2);
            }

            current_pos = offset as usize;
            has_jumped = true;
        } else {
            let name_len = current_byte;
            let label_name = &buf[current_pos + 1..current_pos + 1 + name_len as usize];

            name_vec.push(name_len);
            name_vec.extend_from_slice(label_name);

            current_pos = current_pos + 1 + name_len as usize;
        }
    }

    Ok((name_vec, next_pos.context("missing next position")?))
}
