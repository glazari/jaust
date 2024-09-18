use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;
use crate::print_debug as p;

#[derive(Debug)]
pub struct MethodParametersAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct MethodParameter {
    name_index: u16,
    access_flags: AccessFlags,
}

#[derive(Debug)]
pub struct AccessFlags(u16);

#[derive(Debug)]
pub enum AccessFlag {
    FINAL,     // Indicates that the formal parameter was declared final.
    SYNTHETIC, // Indicates that the formal parameter was not explicitly or implicitly declared in source code.
    MANDATED,  // Indicates that the formal parameter was implicitly declared in source code.
}

impl MethodParametersAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<MethodParametersAttribute> {
        let mut parameters = Vec::new();
        let parameters_count = file.read_u1()?;
        for _j in 0..parameters_count {
            parameters.push(MethodParameter::parse(file)?);
        }
        Ok(MethodParametersAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            parameters,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        p!("\nMethodParametersAttribute");
        s.push_str("MethodParameters:");
        for param in &self.parameters {
            s.push_str("\n\t-");
            s.push_str(&param.to_string(cp));
        }
        s
    }
}

impl MethodParameter {
    pub fn parse(file: &mut FileReader) -> Result<MethodParameter> {
        Ok(MethodParameter {
            name_index: file.read_u2_to_u16()?,
            access_flags: AccessFlags(file.read_u2_to_u16()?),
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        let name = if self.name_index == 0 {
            "<no name>".to_string()
        } else {
            cp.get_to_string(self.name_index)
        };
        p!("name: {}", name);
        let access_flags = self.access_flags.flag_vector();
        p!("access_flags: {:?}", access_flags);
        s.push_str(&format!("{} ({:?}, {:?})", name, self.access_flags, access_flags));
        s
    }
}

impl AccessFlags {
    pub fn flag_vector(&self) -> Vec<AccessFlag> {
        let mut flags = Vec::new();
        add_flag(&mut flags, self.0, 0x0010, AccessFlag::FINAL);
        add_flag(&mut flags, self.0, 0x1000, AccessFlag::SYNTHETIC);
        add_flag(&mut flags, self.0, 0x8000, AccessFlag::MANDATED);
        flags

    }
}

fn add_flag(flags: &mut Vec<AccessFlag>, bit_flags: u16, bit_mask: u16, access_flag: AccessFlag) {
    if bit_flags & bit_mask == bit_mask  {
        flags.push(access_flag);
    } 
}
