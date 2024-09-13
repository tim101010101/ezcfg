use ansi_term::Colour::{Blue, Cyan, Red, White, Yellow};

#[macro_export]
macro_rules! log_with_indent {
    ($prefix:expr, $( $msg:tt ),+) => {
        let mut output = String::from($prefix);
        let ident = "    ";

        $(
            output.push_str(&format!("\n{}{}", ident, $msg));
        )*

        println!("{}", output);
    };
}

#[inline]
pub fn info_prefix() -> String {
    format!("{}", White.bold().on(Blue).paint("[INFO]"))
}
#[macro_export]
macro_rules! info {
    ($( $arg:tt )+) => {
        $crate::log_with_indent!(
            $crate::logger::info_prefix(),
            $( $arg )+
        );
    };
}

#[inline]
pub fn warn_prefix() -> String {
    format!("{}", White.bold().on(Yellow).paint("[WARN]"))
}
#[macro_export]
macro_rules! warn {
    ($( $arg:tt )+) => {
        $crate::log_with_indent!(
            $crate::logger::warn_prefix(),
            $( $arg )+
        );
    };
}

#[inline]
pub fn error_prefix() -> String {
    format!("{}", White.bold().on(Red).paint("[ERROR]"))
}
#[macro_export]
macro_rules! error {
    ($( $arg:tt )+) => {
        $crate::log_with_indent!(
            $crate::logger::error_prefix(),
            $( $arg )+
        )
    };
}

#[inline]
pub fn debug_prefix() -> String {
    format!("{}", White.bold().on(Cyan).paint("[DEBUG]"))
}
#[macro_export]
macro_rules! debug {
    ($( $arg:tt )+) => {
        $crate::log_with_indent!(
            $crate::logger::debug_prefix(),
            $( $arg )+
        )
    };
}
