pub struct AccessFlags(u16);

#[derive(Debug)]
pub enum AccessFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
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
        if self.0 & 0x0010 == 0x0010 {
            flags.push(AccessFlag::Final);
        }
        if self.0 & 0x0020 == 0x0020 {
            flags.push(AccessFlag::Super);
        }
        if self.0 & 0x0200 == 0x0200 {
            flags.push(AccessFlag::Interface);
        }
        if self.0 & 0x0400 == 0x0400 {
            flags.push(AccessFlag::Abstract);
        }
        if self.0 & 0x1000 == 0x1000 {
            flags.push(AccessFlag::Synthetic);
        }
        if self.0 & 0x2000 == 0x2000 {
            flags.push(AccessFlag::Annotation);
        }
        if self.0 & 0x4000 == 0x4000 {
            flags.push(AccessFlag::Enum);
        }
        flags
    }
}
