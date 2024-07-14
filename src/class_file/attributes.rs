use anyhow::Result;
use super::file_reader::FileReader;
use super::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl Attributes {
    pub fn from(file: &mut FileReader) -> Result<Attributes> {
        let mut attributes = Vec::new();

        let attributes_count = file.read_u2_to_u16()?;
        for _i in 0..attributes_count {
            let attribute_name_index = file.read_u2_to_u16()?;
            let attribute_length = file.read_u4_to_u32()?;
            let mut info = Vec::new();
            for _j in 0..attribute_length {
                info.push(file.read_u1()?);
            }
            attributes.push(Attribute {
                attribute_name_index,
                attribute_length,
                info,
            });
        }

        Ok(Attributes { attributes })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();

        s.push_str("Attributes");
        for att in &self.attributes {
            s.push_str(&format!("name {}\n", cp.get_to_string(att.attribute_name_index)));
            s.push_str(&format!("length {}\n", att.attribute_length));
        }
        s
    }
}
