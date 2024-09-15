use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct GenericAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl GenericAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<GenericAttribute> {
        let mut info = Vec::new();
        for _j in 0..att_start.attribute_length {
            info.push(file.read_u1()?);
        }
        Ok(GenericAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            info,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let name = cp.get_to_string(self.attribute_name_index);
        format!(
            "GenericAttribute: ({}) length: {}\n",
            name, self.attribute_length
        )
    }
}
