pub const DEBUG: bool = false;

// A wrapper around println that I can turn on and off to debug the code
#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => {{
        if crate::debug_utils::DEBUG {
            println!($($arg)*);
        }
    }};
}
