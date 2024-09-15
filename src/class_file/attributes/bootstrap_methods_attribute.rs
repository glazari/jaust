use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct BootstrapMethodsAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
pub struct BootstrapMethod {
    method_ref: u16,
    arguments: Vec<u16>, // each u16 is a constant pool index
}

impl BootstrapMethodsAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<BootstrapMethodsAttribute> {
        let num_bootstrap_methods = file.read_u2_to_u16()?;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod::parse(file)?);
        }
        Ok(BootstrapMethodsAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            bootstrap_methods,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("BootstrapMethods:");
        for bm in &self.bootstrap_methods {
            s.push_str("\n\t- ");
            s.push_str(&bm.to_string(cp));
        }
        s.push_str("\n");
        s   
    }
}

impl BootstrapMethod {
    pub fn parse(file: &mut FileReader) -> Result<BootstrapMethod> {
        let method_ref = file.read_u2_to_u16()?;
        let argument_count = file.read_u2_to_u16()?;
        let mut arguments = Vec::with_capacity(argument_count as usize);
        for _ in 0..argument_count {
            arguments.push(file.read_u2_to_u16()?);
        }
        Ok(BootstrapMethod { method_ref, arguments })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let method_name = cp.get_to_string(self.method_ref);
        let mut s = format!("{}(", method_name);
        for arg in &self.arguments {
            s.push_str("\n\t\t- ");
            s.push_str(&cp.get_to_string(*arg));
            s.push_str(", ");
        }
        s.push_str(")");
        s
    }
}
