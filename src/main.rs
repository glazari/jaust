mod class_file;

fn main() {
    class_file::read_class_file("Example.class").unwrap();
}
