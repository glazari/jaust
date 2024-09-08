use super::attributes::AttStart;

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};

use anyhow::Result;

#[derive(Debug)]
pub struct SourceFileAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    sourcefile_index: u16,
}

impl SourceFileAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<SourceFileAttribute> {
        let sourcefile_index = file.read_u2_to_u16()?;
        Ok(SourceFileAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            sourcefile_index,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        cp.get_to_string(self.sourcefile_index)
    }
}
