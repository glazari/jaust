pub struct AccessFlags(u16);

#[derive(Debug, PartialEq)]
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
        add_flag(&mut flags, self.0, 0x0001, AccessFlag::Public);
        add_flag(&mut flags, self.0, 0x0010, AccessFlag::Final);
        add_flag(&mut flags, self.0, 0x0020, AccessFlag::Super);
        add_flag(&mut flags, self.0, 0x0200, AccessFlag::Interface);
        add_flag(&mut flags, self.0, 0x0400, AccessFlag::Abstract);
        add_flag(&mut flags, self.0, 0x1000, AccessFlag::Synthetic);
        add_flag(&mut flags, self.0, 0x2000, AccessFlag::Annotation);
        add_flag(&mut flags, self.0, 0x4000, AccessFlag::Enum);
        flags
    }
}

fn add_flag(flags: &mut Vec<AccessFlag>, bit_flags: u16, bit_mask: u16, access_flag: AccessFlag) {
    if bit_flags & bit_mask == bit_mask {
        flags.push(access_flag);
    }
}
