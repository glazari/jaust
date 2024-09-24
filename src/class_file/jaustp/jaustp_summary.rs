use crate::class_file::{
    fields::AccessFlag as FieldAccessFlag,
    methods::AccessFlag as MethodAccessFlag,
    access_flags::AccessFlag,
    ClassFile,
};

use crate::class_file::jaustp::{
    print_code::print_code,
    parse_method_descriptor::parse_method_descriptor,
    parse_method_descriptor::parse_type_descriptor,
};



pub struct Options {
    pub private: bool,
    pub code: bool,
}

/// Print a summary of the class file. like javap does by default.
pub fn jaustp_summary_print(cf: &ClassFile, opts: &Options) {
    let out = jaustp_summary(cf, opts);
    print!("{}", out);
}

pub fn jaustp_summary(cf: &ClassFile, opts: &Options) -> String {
    let mut out = String::new();
    let source = cf.attributes.get_source_file(&cf.constant_pool);
    if let Some(source) = source {
        out.push_str(&format!("Compiled from \"{}\"\n", source));
    }
    add_class_line(cf, &mut out);

    add_fields(cf, &mut out, opts);

    add_methods(cf, &mut out, opts);

    out.push_str("}\n");

    out
}


fn add_fields(cf: &ClassFile, out: &mut String, opts: &Options) {
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
            if !opts.private {
                continue;
            }
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

        // Synthetic fields are not marked synthetic in javap
        // Enum fields are not marked enum in javap
        

        out.push_str(indent);
        out.push_str(&modifiers.join(" "));
        if modifiers.len() > 0 {
            out.push_str(" ");
        }

        let mut descriptor = field_descriptor.chars().peekable();
        let _type = parse_type_descriptor(&mut descriptor).unwrap_or_else(|e| e.to_string());
        out.push_str(&_type);

        out.push_str(" ");
        out.push_str(&field_name);
        out.push_str(";\n");
    }
}

fn add_methods(cf: &ClassFile, out: &mut String, opts: &Options) {
    let indent = "  ";
    for method in &cf.methods.methods {
        let flags = method.access_flags.flag_vector();

        if flags.contains(&MethodAccessFlag::Private) && !opts.private {
            continue;
        }

        let modifiers = flags
            .iter()
            .map(|f| f.to_str().to_string())
            .filter(|s| s != "synthetic") // synthetic methods are not marked synthetic in javap
            .collect::<Vec<String>>()
            .join(" ");
        out.push_str(indent);
        out.push_str(&modifiers);

        if modifiers.len() > 0 {
            out.push_str(" ");
        }

        let method_name = cf.constant_pool.get_to_string(method.name_index);
        if method_name == "<clinit>" {
            // static initializer
            out.push_str("{};\n");
            continue;
        }

        let descriptor = cf.constant_pool.get_to_string(method.descriptor_index);
        let signature = parse_method_descriptor(&descriptor, method_name);

        if signature.name == "<init>" {
            let class_name = cf.constant_pool.get_to_string(cf.this_class);
            let class_name = class_name.replace("/", ".");
            out.push_str(&class_name);
        } else {
            out.push_str(&signature.return_type);
            out.push_str(" ");
            out.push_str(&signature.name);
        }

        out.push_str("(");
        out.push_str(&signature.args.join(", "));
        out.push_str(")");

        let checked_exceptions = method.attributes.get_checked_exceptions(&cf.constant_pool);
        if checked_exceptions.len() > 0 {
            let exceptions = checked_exceptions.iter()
                .map(|e| e.replace('/', "."))
                .collect::<Vec<String>>() 
                .join(" ");
            out.push_str(" throws ");
            out.push_str(&exceptions);
        }

        out.push_str(";\n");
        if opts.code {
            print_code(&method, &cf.constant_pool, out);
            out.push_str("\n");
        }
    }
}

pub fn add_class_line(cf: &ClassFile, out: &mut String) {
    add_class_modifiers(cf, out);

    let class_name = cf.constant_pool.get_to_string(cf.this_class);
    let class_name = class_name.replace("/", ".");
    out.push_str(" ");
    out.push_str(&class_name);

    if cf.super_class != 0 {
        let super_class_name = cf.constant_pool.get_to_string(cf.super_class);
        if super_class_name != "java/lang/Object" {
            let super_class_name = super_class_name.replace("/", ".");
            out.push_str(" extends ");
            out.push_str(&super_class_name);
        }
    }

    if cf.interfaces.interfaces.len() > 0 {
        out.push_str(" implements ");
        for interface in &cf.interfaces.interfaces {
            let interface_name = cf.constant_pool.get_to_string(*interface);
            let interface_name = interface_name.replace("/", ".");
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

    let kind = if flags.contains(&AccessFlag::Interface) {
        "interface"
    } else if flags.contains(&AccessFlag::Enum) {
        "enum"
    } else {
        "class"
    };

    if flags.contains(&AccessFlag::Public) {
        modifiers.push("public");
    }

    if flags.contains(&AccessFlag::Final) {
        modifiers.push("final");
    }

    if flags.contains(&AccessFlag::Abstract) && kind != "interface" /* interfaces are always abstract */ {
        modifiers.push("abstract");
    }

    modifiers.push(kind);
    out.push_str(&modifiers.join(" "));
}
