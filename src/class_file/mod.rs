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
mod constant_pool;
mod file_reader;
mod interfaces;
mod fields;
mod methods;
mod attributes;

use access_flags::AccessFlags;
use constant_pool::ConstantPool;
use file_reader::FileReader;
use interfaces::Interfaces;
use fields::Fields;
use methods::Methods;


use anyhow::Result;

struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: AccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Interfaces,
}

pub fn read_class_file(filename: &str) -> Result<()> {
    let mut file = FileReader::new(filename)?;

    let magic = file.read_u4()?;
    println!("magic: {:?}", magic);
    println!("magic: {:x?}", magic);
    // magic number is 0xCAFEBABE
    assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

    let minor_version = file.read_u2()?;
    let major_version = file.read_u2()?;
    println!("versions");
    println!("minor, major: {:?}, {:?}", minor_version, major_version);

    let constant_pool = ConstantPool::from(&mut file)?;

    println!("constant pool info");
    println!("{}", constant_pool.to_string());

    let access_flags = AccessFlags::new(file.read_u2_to_u16()?);
    println!("access flags: {:?}", access_flags.flag_vector());

    let this_class = file.read_u2_to_u16()?;
    let super_class = file.read_u2_to_u16()?;

    println!(
        "this class: {:?} {}",
        this_class,
        constant_pool.get_to_string(this_class)
    );
    println!(
        "super class: {:?} {}",
        super_class,
        constant_pool.get_to_string(super_class)
    );

    let interfaces = Interfaces::from(&mut file)?;
    println!("{}", interfaces.to_string(&constant_pool));

    let fields = Fields::from(&mut file)?;

    println!("{}", fields.to_string(&constant_pool));

    let methods = Methods::from(&mut file)?;

    println!("{}", methods.to_string(&constant_pool));

    Ok(())
}
