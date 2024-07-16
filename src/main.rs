mod class_file;

mod debug_utils;

use crate::print_debug as p;
use class_file::javap_print;

fn main() {
    p!("Hello, world!");
    let cf = class_file::read_class_file("Example.class").unwrap();
    javap_print(&cf);
}
