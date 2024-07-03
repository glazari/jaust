mod class_file;

fn main() {
    println!("Hello, world!");
    class_file::read_class_file("Example.class").unwrap();
}
