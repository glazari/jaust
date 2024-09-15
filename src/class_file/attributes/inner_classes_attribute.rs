use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use crate::class_file::access_flags::AccessFlags;
use anyhow::Result;


#[derive(Debug)]
pub struct InnerClassesAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    classes: Vec<InnerClassInfo>,
}

#[derive(Debug)]
pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

impl InnerClassesAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<InnerClassesAttribute> {
        let inner_class_count = file.read_u2_to_u16()?;
        let mut classes = Vec::new();
        for _ in 0..inner_class_count {
            let class = InnerClassInfo::parse(file)?;
        }
        Ok(InnerClassesAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            classes,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("InnerClasses: ");
        s.push_str(&format!("({})", self.classes.len()));
        for class in &self.classes {
            s.push_str("\n\t - ");
            s.push_str(&class.to_string(cp));
            s.push_str(", ");
        }
        s.push_str("\n");
        s
    }
}


impl InnerClassInfo {
    pub fn parse(file: &mut FileReader) -> Result<InnerClassInfo> {
        Ok(InnerClassInfo {
            inner_class_info_index: file.read_u2_to_u16()?,
            outer_class_info_index: file.read_u2_to_u16()?,
            inner_name_index: file.read_u2_to_u16()?,
            inner_class_access_flags: file.read_u2_to_u16()?,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str(&cp.get_to_string(self.inner_class_info_index));
        s.push_str(", ");
        s.push_str(&cp.get_to_string(self.outer_class_info_index));
        s.push_str(", ");
        s.push_str(&cp.get_to_string(self.inner_name_index));
        s.push_str(", ");
        let flags = AccessFlags::new(self.inner_class_access_flags);
        s.push_str(&format!("flags: {:04x} ({:?})", self.inner_class_access_flags, flags.flag_vector()));
        s
    }
}
