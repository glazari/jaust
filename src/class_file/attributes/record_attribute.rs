use super::attributes::AttStart;
use crate::class_file::attributes::Attributes;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct RecordAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    components: Vec<RecordComponentInfo>,
}

#[derive(Debug)]
pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes: Attributes,
}

impl RecordAttribute {
    pub fn parse(
        file: &mut FileReader,
        att_start: &AttStart,
        cp: &ConstantPool,
    ) -> Result<RecordAttribute> {
        let component_count = file.read_u2_to_u16()?;
        let mut components = Vec::new();
        for _i in 0..component_count {
            let component = RecordComponentInfo::parse(file, cp)?;
            components.push(component);
        }

        Ok(RecordAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            components,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("Record: ");
        for component in &self.components {
            s.push_str(&component.to_string(cp));
            s.push_str(", ");
        }
        s.push_str("\n");
        s
    }
}

impl RecordComponentInfo {
    pub fn parse(file: &mut FileReader, cp: &ConstantPool) -> Result<RecordComponentInfo> {
        let name_index = file.read_u2_to_u16()?;
        let descriptor_index = file.read_u2_to_u16()?;
        let attributes = Attributes::from(file, cp)?;

        Ok(RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str(&cp.get_to_string(self.name_index));
        s.push_str(": ");
        s.push_str(&cp.get_to_string(self.descriptor_index));
        s
    }
}
