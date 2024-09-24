use super::attributes::Attributes;
use super::constant_pool::ConstantPool;
use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub struct Fields {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes,
}

#[derive(Debug)]
pub struct AccessFlags(u16);

#[derive(Debug, PartialEq)]
pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum,
}

impl AccessFlags {
    pub fn new(flags: u16) -> AccessFlags {
        AccessFlags(flags)
    }

    pub fn flag_vector(&self) -> Vec<AccessFlag> {
        let mut flags = Vec::new();
        if self.0 & 0x0001 == 0x0001 {
            flags.push(AccessFlag::Public);
        }
        if self.0 & 0x0002 == 0x0002 {
            flags.push(AccessFlag::Private);
        }
        if self.0 & 0x0004 == 0x0004 {
            flags.push(AccessFlag::Protected);
        }
        if self.0 & 0x0008 == 0x0008 {
            flags.push(AccessFlag::Static);
        }
        if self.0 & 0x0010 == 0x0010 {
            flags.push(AccessFlag::Final);
        }
        if self.0 & 0x0040 == 0x0040 {
            flags.push(AccessFlag::Volatile);
        }
        if self.0 & 0x0080 == 0x0080 {
            flags.push(AccessFlag::Transient);
        }
        if self.0 & 0x1000 == 0x1000 {
            flags.push(AccessFlag::Synthetic);
        }
        if self.0 & 0x4000 == 0x4000 {
            flags.push(AccessFlag::Enum);
        }
        flags
    }
}

impl Fields {
    pub fn from(file: &mut FileReader, cp: &ConstantPool) -> Result<Fields> {
        let mut fields = Vec::new();

        let fields_count = file.read_u2_to_u16()?;
        for _i in 0..fields_count {
            let access_flags = AccessFlags::new(file.read_u2_to_u16()?);
            let name_index = file.read_u2_to_u16()?;
            let descriptor_index = file.read_u2_to_u16()?;

            let attributes = Attributes::from(file, cp)?;

            fields.push(Field {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            });
        }

        Ok(Fields { fields })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();

        for field in &self.fields {
            let name = cp.get_to_string(field.name_index);

            s.push_str(name.as_str());
            s.push_str(": ");

            let description = cp.get_to_string(field.descriptor_index);
            s.push_str(description.as_str());

            s.push_str(" [");
            s.push_str(format!("{:?}", field.attributes).as_str());
            s.push_str("] ");

            let flags = field.access_flags.flag_vector();
            s.push_str(format!("{:?}", flags).as_str());

            s.push_str("\n");
        }

        s
    }
}
