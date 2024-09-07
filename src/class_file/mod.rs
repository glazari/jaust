/*
 * Java class file format
 * from the Oracle documentation


ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}

 *
 *
 */

mod access_flags;
mod attributes;
mod bytecode;
mod code_attribute;
mod constant_pool;
mod fields;
mod file_reader;
mod interfaces;
mod javap_printer;
mod methods;

use access_flags::AccessFlags;
use attributes::Attributes;
use constant_pool::ConstantPool;
use fields::Fields;
use file_reader::FileReader;
use interfaces::Interfaces;
use methods::Methods;

use crate::print_debug as p;
pub use javap_printer::print_tldr as javap_print;
pub use javap_printer::Options as JavapOptions;

use anyhow::Result;

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Interfaces,
    pub fields: Fields,
    pub methods: Methods,
    pub attributes: Attributes,
}

pub fn read_class_file(filename: &str) -> Result<ClassFile> {
    let mut file = FileReader::new(filename)?;

    let magic = file.read_u4()?;
    assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

    let minor_version = file.read_u2_to_u16()?;
    let major_version = file.read_u2_to_u16()?;
    p!(
        "versions (minor, major): ({:?}, {:?})",
        minor_version,
        major_version
    );

    let constant_pool = ConstantPool::from(&mut file)?;

    p!("constant pool info");
    p!("{}", constant_pool.to_string());

    let access_flags = AccessFlags::new(file.read_u2_to_u16()?);
    p!("access flags: {:?}", access_flags.flag_vector());

    let this_class = file.read_u2_to_u16()?;
    let super_class = file.read_u2_to_u16()?;

    p!(
        "this class: {:?} {}",
        this_class,
        constant_pool.get_to_string(this_class)
    );
    p!(
        "super class: {:?} {}",
        super_class,
        constant_pool.get_to_string(super_class)
    );

    let interfaces = Interfaces::from(&mut file)?;
    p!("{}", interfaces.to_string(&constant_pool));

    let fields = Fields::from(&mut file, &constant_pool)?;

    p!("{}", fields.to_string(&constant_pool));

    let methods = Methods::from(&mut file, &constant_pool)?;

    p!("{}", methods.to_string(&constant_pool));

    let attributes = Attributes::from(&mut file, &constant_pool)?;

    p!("{}", attributes.to_string(&constant_pool));

    Ok(ClassFile {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    })
}

pub fn raw_string(cf: &ClassFile) -> String {
    let mut s = String::new();
    s.push_str(&format!("minor_version: {}\n", cf.minor_version));
    s.push_str(&format!("major_version: {}\n", cf.major_version));
    s.push_str(&cf.constant_pool.to_string());
    s.push_str(&format!("access_flags: {:?}\n", cf.access_flags.flag_vector()));
    s.push_str(&format!("this_class: {}\n", cf.constant_pool.get_to_string(cf.this_class)));
    s.push_str(&format!("super_class: {}\n", cf.constant_pool.get_to_string(cf.super_class)));
    s.push_str(&cf.interfaces.to_string(&cf.constant_pool));
    s.push_str(&cf.fields.to_string(&cf.constant_pool));
    s.push_str(&cf.methods.to_string(&cf.constant_pool));
    s.push_str(&cf.attributes.to_string(&cf.constant_pool));
    s
}
