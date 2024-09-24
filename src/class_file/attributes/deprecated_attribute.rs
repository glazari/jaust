use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct DeprecatedAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
}

impl DeprecatedAttribute {
    pub fn parse(_file: &mut FileReader, att_start: &AttStart) -> Result<DeprecatedAttribute> {
        Ok(DeprecatedAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
        })
    }

    pub fn to_string(&self, _cp: &ConstantPool) -> String {
        "Deprecated\n".to_string()
    }
}
