#[macro_export]
/// Exit the program with the given exit code.
macro_rules! exit {
    ($code:expr) => {{
        std::process::exit($code);
    }};
}
