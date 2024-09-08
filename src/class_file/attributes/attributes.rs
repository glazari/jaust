use super::{CodeAttribute, GenericAttribute, LineNumberTableAttribute, SourceFileAttribute};

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct Attributes {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    Code(CodeAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    GenericAttribute(GenericAttribute),
}

#[derive(Debug)]
pub struct AttStart {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

impl Attributes {
    pub fn from(file: &mut FileReader, cp: &ConstantPool) -> Result<Attributes> {
        let mut attributes = Vec::new();

        let attributes_count = file.read_u2_to_u16()?;
        for _i in 0..attributes_count {
            let attribute_name_index = file.read_u2_to_u16()?;
            let attribute_length = file.read_u4_to_u32()?;

            let att_start = AttStart {
                attribute_name_index,
                attribute_length,
            };

            let name = cp.get_to_string(attribute_name_index);
            match name.as_str() {
                "Code" => {
                    let att = CodeAttribute::parse(file, &att_start, cp)?;
                    attributes.push(Attribute::Code(att));
                }
                "SourceFile" => {
                    let att = SourceFileAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::SourceFile(att));
                }
                "LineNumberTable" => {
                    let att = LineNumberTableAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::LineNumberTable(att));
                }
                _ => {
                    let att = GenericAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::GenericAttribute(att));
                }
            }
        }

        Ok(Attributes { attributes })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();

        s.push_str("Attributes\n");
        for att in &self.attributes {
            s.push_str(&att.to_string(cp));
        }
        s
    }

    pub fn get_source_file(&self, cp: &ConstantPool) -> Option<String> {
        for att in &self.attributes {
            match att {
                Attribute::SourceFile(att) => {
                    return Some(att.to_string(cp));
                }
                _ => {}
            }
        }
        None
    }
}

impl Attribute {
    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        match self {
            Attribute::GenericAttribute(att) => {
                s.push_str(&att.to_string(cp));
            }
            Attribute::SourceFile(att) => {
                s.push_str("SourceFile ");
                s.push_str(&att.to_string(cp));
                s.push_str("\n");
            }
            Attribute::Code(att) => {
                s.push_str(&att.to_string(cp));
            }
            Attribute::LineNumberTable(att) => {
                s.push_str(&att.to_string(cp));
            }
        }
        s
    }
}
