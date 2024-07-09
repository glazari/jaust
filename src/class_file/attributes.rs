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
}
