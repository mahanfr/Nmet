#[macro_export]
macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}",format_args!($($arg)+))
    );
}
