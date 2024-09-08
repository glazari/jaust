use super::attributes::AttStart;
use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use anyhow::Result;

#[derive(Debug)]
pub struct StackMapTableAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub enum StackMapFrame {
    Same(SameFrame),
    SameLocals1StackItem(SameLocals1StackItemFrame),
    SameLocals1StackItemExtended(SameLocals1StackItemFrameExtended),
    Chop(ChopFrame),
    SameExtended(SameFrameExtended),
    Append(AppendFrame),
    Full(FullFrame),
}

#[derive(Debug)]
pub struct SameFrame {
    offset_delta: u8,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    // type 64-127
    offset_delta: u8,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    // type 247
    offset_delta: u16,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct ChopFrame {
    // type 248-250
    k_absent: u8, // 251 - _type
    offset_delta: u16,
}

#[derive(Debug)]
pub struct SameFrameExtended {
    // type 251
    offset_delta: u16,
}

#[derive(Debug)]
pub struct AppendFrame {
    // type 252-254
    offset_delta: u16,
    locals: Vec<VerificationTypeInfo>, // length = _type - 251
}

#[derive(Debug)]
pub struct FullFrame {
    // type 255
    offset_delta: u16,
    locals: Vec<VerificationTypeInfo>,
    stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVaiableInfo,                            // 0
    IntegerVariableInfo,                       // 1
    FloatVariableInfo,                         // 2
    LongVariableInfo,                          // 4
    DoubleVariableInfo,                        // 3
    NullVariableInfo,                          // 5
    UninitializedThisVariableInfo,             // 6
    ObjectVariableInfo { cpool_index: u16 },   // 7
    UninitializedVariableInfo { offset: u16 }, // 8
}

impl StackMapTableAttribute {
    pub fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<StackMapTableAttribute> {
        let mut entries = Vec::new();
        let number_of_entries = file.read_u2_to_u16()?;
        for _i in 0..number_of_entries {
            let frame = StackMapFrame::parse(file)?;
            entries.push(frame);
        }

        Ok(StackMapTableAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            entries,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("StackMapTable\n");
        for entry in &self.entries {
            s.push_str(&format!("\t- {}\n", entry.to_string(cp)));
        }
        s
    }
}

impl StackMapFrame {
    fn parse(file: &mut FileReader) -> Result<StackMapFrame> {
        let frame_type = file.read_u1()?;
        match frame_type {
            0..=63 => Self::parse_same(file, frame_type),
            64..=127 => Self::parse_same_locals_1_stack_item(file, frame_type),
            247 => Self::parse_same_locals_1_stack_item_extended(file),
            248..=250 => Self::parse_chop(file, frame_type),
            251 => Self::parse_same_extended(file),
            252..=254 => Self::parse_append(file, frame_type),
            255 => Self::parse_full(file),
            _ => panic!("Invalid frame type: {}", frame_type),
        }
    }

    fn parse_same(file: &mut FileReader, frame_type: u8) -> Result<StackMapFrame> {
        Ok(StackMapFrame::Same(SameFrame {
            offset_delta: frame_type,
        }))
    }

    fn parse_same_locals_1_stack_item(
        file: &mut FileReader,
        frame_type: u8,
    ) -> Result<StackMapFrame> {
        let offset_delta = frame_type - 64;
        let stack = VerificationTypeInfo::parse(file)?;
        Ok(StackMapFrame::SameLocals1StackItem(
            SameLocals1StackItemFrame {
                offset_delta,
                stack,
            },
        ))
    }

    fn parse_same_locals_1_stack_item_extended(file: &mut FileReader) -> Result<StackMapFrame> {
        let offset_delta = file.read_u2_to_u16()?;
        let stack = VerificationTypeInfo::parse(file)?;
        Ok(StackMapFrame::SameLocals1StackItemExtended(
            SameLocals1StackItemFrameExtended {
                offset_delta,
                stack,
            },
        ))
    }

    fn parse_chop(file: &mut FileReader, frame_type: u8) -> Result<StackMapFrame> {
        let k_absent = 251 - frame_type;
        let offset_delta = file.read_u2_to_u16()?;
        Ok(StackMapFrame::Chop(ChopFrame {
            k_absent,
            offset_delta,
        }))
    }

    fn parse_same_extended(file: &mut FileReader) -> Result<StackMapFrame> {
        let offset_delta = file.read_u2_to_u16()?;
        Ok(StackMapFrame::SameExtended(SameFrameExtended {
            offset_delta,
        }))
    }

    fn parse_append(file: &mut FileReader, frame_type: u8) -> Result<StackMapFrame> {
        let offset_delta = file.read_u2_to_u16()?;
        let locals = (0..frame_type - 251)
            .map(|_| VerificationTypeInfo::parse(file))
            .collect::<Result<Vec<_>>>()?;
        Ok(StackMapFrame::Append(AppendFrame {
            offset_delta,
            locals,
        }))
    }

    fn parse_full(file: &mut FileReader) -> Result<StackMapFrame> {
        let offset_delta = file.read_u2_to_u16()?;
        let locals = (0..file.read_u2_to_u16()?)
            .map(|_| VerificationTypeInfo::parse(file))
            .collect::<Result<Vec<_>>>()?;
        let stack = (0..file.read_u2_to_u16()?)
            .map(|_| VerificationTypeInfo::parse(file))
            .collect::<Result<Vec<_>>>()?;
        Ok(StackMapFrame::Full(FullFrame {
            offset_delta,
            locals,
            stack,
        }))
    }

    fn to_string(&self, cp: &ConstantPool) -> String {
        match self {
            Self::Same(frame) => format!("Same: {}", frame.offset_delta),
            Self::SameLocals1StackItem(frame) => {
                format!(
                    "SameLocals1StackItem: {} {}",
                    frame.offset_delta,
                    frame.stack.to_string(cp)
                )
            }
            Self::SameLocals1StackItemExtended(frame) => {
                format!(
                    "SameLocals1StackItemExtended: {} {}",
                    frame.offset_delta,
                    frame.stack.to_string(cp)
                )
            }
            Self::Chop(frame) => {
                format!(
                    "Chop: {} ({} locals removed)",
                    frame.offset_delta, frame.k_absent
                )
            }
            Self::SameExtended(frame) => {
                format!("SameExtended: {}", frame.offset_delta)
            }
            Self::Append(frame) => {
                let locals = frame
                    .locals
                    .iter()
                    .map(|v| v.to_string(cp))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Append: {} {}", frame.offset_delta, locals)
            }
            Self::Full(frame) => {
                let locals = frame
                    .locals
                    .iter()
                    .map(|v| v.to_string(cp))
                    .collect::<Vec<_>>()
                    .join(", ");
                let stack = frame
                    .stack
                    .iter()
                    .map(|v| v.to_string(cp))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "Full: {} locals: {} stack: {}",
                    frame.offset_delta, locals, stack
                )
            }
        }
    }
}

impl VerificationTypeInfo {
    fn parse(file: &mut FileReader) -> Result<VerificationTypeInfo> {
        let tag = file.read_u1()?;
        match tag {
            0 => Ok(VerificationTypeInfo::TopVaiableInfo),
            1 => Ok(VerificationTypeInfo::IntegerVariableInfo),
            2 => Ok(VerificationTypeInfo::FloatVariableInfo),
            4 => Ok(VerificationTypeInfo::LongVariableInfo),
            3 => Ok(VerificationTypeInfo::DoubleVariableInfo),
            5 => Ok(VerificationTypeInfo::NullVariableInfo),
            6 => Ok(VerificationTypeInfo::UninitializedThisVariableInfo),
            7 => Ok(VerificationTypeInfo::ObjectVariableInfo {
                cpool_index: file.read_u2_to_u16()?,
            }),
            8 => Ok(VerificationTypeInfo::UninitializedVariableInfo {
                offset: file.read_u2_to_u16()?,
            }),
            _ => panic!("Invalid tag: {}", tag),
        }
    }

    fn to_string(&self, cp: &ConstantPool) -> String {
        match self {
            VerificationTypeInfo::TopVaiableInfo => "Top".to_string(),
            VerificationTypeInfo::IntegerVariableInfo => "Integer".to_string(),
            VerificationTypeInfo::FloatVariableInfo => "Float".to_string(),
            VerificationTypeInfo::LongVariableInfo => "Long".to_string(),
            VerificationTypeInfo::DoubleVariableInfo => "Double".to_string(),
            VerificationTypeInfo::NullVariableInfo => "Null".to_string(),
            VerificationTypeInfo::UninitializedThisVariableInfo => "UninitializedThis".to_string(),
            VerificationTypeInfo::ObjectVariableInfo { cpool_index } => {
                format!("Object: {}", cp.get_to_string(*cpool_index))
            }
            VerificationTypeInfo::UninitializedVariableInfo { offset } => {
                format!("Uninitialized: {}", offset)
            }
        }
    }
}
