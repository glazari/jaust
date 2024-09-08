use super::attributes::AttStart;

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};

use anyhow::Result;

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    line_number_table: Vec<LineNumberTable>,
}

#[derive(Debug)]
pub struct LineNumberTable {
    start_pc: u16, // The instruction offset from the start of the code array at which the line number begins.
    line_number: u16, // The line number in the original source file.
}

impl LineNumberTableAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<LineNumberTableAttribute> {
        let line_number_table_length = file.read_u2_to_u16()?;
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _j in 0..line_number_table_length {
            let start_pc = file.read_u2_to_u16()?;
            let line_number = file.read_u2_to_u16()?;
            line_number_table.push(LineNumberTable {
                start_pc,
                line_number,
            });
        }
        Ok(LineNumberTableAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            line_number_table,
        })
    }

    pub fn to_string(&self, _cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("LineNumberTable\n");
        for lnt in &self.line_number_table {
            s.push_str(&format!(
                "\t- start_pc {} line_number {}\n",
                lnt.start_pc, lnt.line_number
            ));
        }
        s
    }
}
