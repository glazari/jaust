use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct SignatureAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    signature_index: u16,
}

impl SignatureAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<SignatureAttribute> {
        Ok(SignatureAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            signature_index: file.read_u2_to_u16()?,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let signature = cp.get_to_string(self.signature_index);
        format!("Signature: {}\n", signature)
    }
}
