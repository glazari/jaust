use super::attributes::AttStart;

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};

use anyhow::Result;

#[derive(Debug)]
pub struct ExceptionsAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    exception_index_table: Vec<u16>,
}

impl ExceptionsAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<ExceptionsAttribute> {
        let number_of_exceptions = file.read_u2_to_u16()?;
        let mut exception_index_table = Vec::new();
        for _i in 0..number_of_exceptions {
            let exception_index = file.read_u2_to_u16()?;
            exception_index_table.push(exception_index);
        }
        Ok(ExceptionsAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            exception_index_table,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("Exceptions\n");
        for exception_index in &self.exception_index_table {
            s.push_str(&format!("\t- {}\n", cp.get_to_string(*exception_index)));
        }
        s
    }
}
