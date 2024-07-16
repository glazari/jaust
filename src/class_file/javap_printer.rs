use super::access_flags::AccessFlag;
use super::fields::AccessFlag as FieldAccessFlag;
use super::ClassFile;

/// Print a summary of the class file. like javap does by default.
pub fn print_tldr(cf: &ClassFile) {
    let mut out = String::new();
    let source = cf.attributes.get_source_file(&cf.constant_pool);
    if let Some(source) = source {
        out.push_str(&format!("Compiled from \"{}\"\n", source));
    }
    add_class_line(cf, &mut out);

    add_fields(cf, &mut out);

    out.push_str("}\n");

    println!("{}", out);
}

fn add_fields(cf: &ClassFile, out: &mut String) {
    let indent = "  ";
    for field in &cf.fields.fields {
        let field_name = cf.constant_pool.get_to_string(field.name_index);
        let field_descriptor = cf.constant_pool.get_to_string(field.descriptor_index);

        let flags = field.access_flags.flag_vector();
        let mut modifiers = Vec::new();

        if flags.contains(&FieldAccessFlag::Public) {
            modifiers.push("public");
        } else if flags.contains(&FieldAccessFlag::Private) {
            modifiers.push("private");
            continue;
        } else if flags.contains(&FieldAccessFlag::Protected) {
            modifiers.push("protected");
        }

        if flags.contains(&FieldAccessFlag::Static) {
            modifiers.push("static");
        }

        if flags.contains(&FieldAccessFlag::Final) {
            modifiers.push("final");
        }

        if flags.contains(&FieldAccessFlag::Volatile) {
            modifiers.push("volatile");
        }

        if flags.contains(&FieldAccessFlag::Transient) {
            modifiers.push("transient");
        }

        if flags.contains(&FieldAccessFlag::Synthetic) {
            modifiers.push("synthetic");
        }

        if flags.contains(&FieldAccessFlag::Enum) {
            modifiers.push("enum");
        }

        out.push_str(indent);
        out.push_str(&modifiers.join(" "));
        out.push_str(" ");

        match field_descriptor.as_str().get(0..1) {
            Some("L") => out.push_str(&field_descriptor[1..].replace("/", ".").replace(";", "")),
            Some("I") => out.push_str("int"),
            Some("J") => out.push_str("long"),
            Some("F") => out.push_str("float"),
            Some("D") => out.push_str("double"),
            Some("S") => out.push_str("short"),
            Some("B") => out.push_str("byte"),
            Some("C") => out.push_str("char"),
            Some("Z") => out.push_str("boolean"),
            _ => out.push_str(&field_descriptor),
        }

        out.push_str(" ");
        out.push_str(&field_name);
        out.push_str(";\n");
    }
}

pub fn add_class_line(cf: &ClassFile, out: &mut String) {
    add_class_modifiers(cf, out);

    let class_name = cf.constant_pool.get_to_string(cf.this_class);
    out.push_str(" ");
    out.push_str(&class_name);

    if cf.super_class != 0 {
        let super_class_name = cf.constant_pool.get_to_string(cf.super_class);
        out.push_str(" extends ");
        out.push_str(&super_class_name);
    }

    if cf.interfaces.interfaces.len() > 0 {
        out.push_str(" implements ");
        for interface in &cf.interfaces.interfaces {
            let interface_name = cf.constant_pool.get_to_string(*interface);
            out.push_str(&interface_name);
            out.push_str(", ");
        }
        out.pop();
        out.pop();
    }

    out.push_str(" {\n");
}

fn add_class_modifiers(cf: &ClassFile, out: &mut String) {
    let flags = cf.access_flags.flag_vector();

    let mut modifiers = Vec::new();

    if flags.contains(&AccessFlag::Public) {
        modifiers.push("public");
    }

    if flags.contains(&AccessFlag::Final) {
        modifiers.push("final");
    }

    if flags.contains(&AccessFlag::Abstract) {
        modifiers.push("abstract");
    }

    if flags.contains(&AccessFlag::Interface) {
        modifiers.push("interface");
    } else if flags.contains(&AccessFlag::Enum) {
        modifiers.push("enum");
    } else {
        modifiers.push("class");
    }

    out.push_str(&modifiers.join(" "));
}
