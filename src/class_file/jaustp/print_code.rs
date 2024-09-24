use crate::class_file::{
    constant_pool::ConstantPool,
    methods::Method, 
};

pub(super) fn print_code(method: &Method, _cp: &ConstantPool, out: &mut String) {
    let code = method.get_code().unwrap();
    for c in code.code() {
        out.push_str(&format!("\t- {}\n", c.to_string()));
    }
}
