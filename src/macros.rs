/**********************************************************************************************
*
*   macros: Macro rules that have been used in this project
*
*   Defention of Macros used in Nmet source code
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
/// Format asm instruction to Nasm/Human Readable string
#[macro_export]
macro_rules! asm {
    ($($arg:tt)+) => (
        format!("    {}",format_args!($($arg)+))
    );
}

/// Log info
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        println!("[Info] {}",format_args!($($arg)+))
    };
}

/// Log success
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)+) => {
        println!("[Success] {}",format_args!($($arg)+))
    };
}

/// Log Warning
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        println!("[Warn] {}",format_args!($($arg)+))
    };
}

/// Log Error
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        eprintln!("[Error] {}",format_args!($($arg)+))
    };
}

/// log CompilerError
#[macro_export]
macro_rules! log_cerror {
    ($loc:expr, $($arg:tt)+) => {
        eprintln!("[{}] {}",$loc,format_args!($($arg)+))
    };
}
