/**********************************************************************************************
*
*   tests: Full Compilation Testing cases
*
*   Tests that can be run using "cargo test" but are NOT UNIT TESTS!
*   THESE TESTS RELY ON /tests FOLDER
*   which compiles each program file and compares the result with the text file provided
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
use crate::{
    assemble_with_nasm,
    compiler::{compile, OutputType},
    link_to_exc,
    utils::get_program_name,
};
use std::{fs::remove_file, path::Path, process::Command};

macro_rules! test_elf {
    ($tname: ident, $in_path: expr, $res_path: expr) => {
        #[test]
        fn $tname() {
            // Setup names
            let program_name = format!("__elf_{}", get_program_name($in_path));
            let out_path = Path::new(&format!("./build/{program_name}")).to_owned();
            // Generate executable
            compile(
                $in_path.to_string(),
                out_path.to_path_buf(),
                OutputType::Elf,
            );
            link_to_exc(out_path.with_extension(""));
            // Cleanup
            remove_file(out_path.with_extension("o")).unwrap_or_else(|_| ());
            // Return executable path
            let opath = out_path.with_extension("").to_string_lossy().to_string();
            let output = Command::new(&opath)
                .output()
                .expect("Error Executing the program!");
            assert!(output.status.success());
            if !output.status.success() {
                println!("{}", String::from_utf8(output.stderr).unwrap());
                panic!("Failed to run executable!");
            }
            let expectation = std::fs::read_to_string($res_path).unwrap();
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                expectation.to_string()
            );
            remove_file(&opath).unwrap_or_else(|_| ());
        }
    };
}

macro_rules! test_asm {
    ($tname: ident, $in_path: expr, $res_path: expr) => {
        #[test]
        fn $tname() {
            // Setup names
            let program_name = format!("__asm_{}", get_program_name($in_path));
            let out_path = Path::new(&format!("./build/{program_name}")).to_owned();
            // Generate executable
            compile(
                $in_path.to_string(),
                out_path.to_path_buf(),
                OutputType::Asm,
            );
            assemble_with_nasm(out_path.with_extension("o"));
            link_to_exc(out_path.with_extension(""));
            // Cleanup
            remove_file(out_path.with_extension("o")).unwrap_or_else(|_| ());
            remove_file(out_path.with_extension("asm")).unwrap_or_else(|_| ());
            // Return executable path
            let opath = out_path.with_extension("").to_string_lossy().to_string();
            let output = Command::new(&opath)
                .output()
                .expect("Error Executing the program!");
            assert!(output.status.success());
            let expectation = std::fs::read_to_string($res_path).unwrap();
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                expectation.to_string()
            );
            remove_file(&opath).unwrap_or_else(|_| ());
        }
    };
}

mod asm {
    use super::*;

    test_asm!(
        binary_expr,
        "./tests/binary_expr.nmt",
        "./tests/binary_expr.txt"
    );
    test_asm!(
        cont_break,
        "./tests/cont_break.nmt",
        "./tests/cont_break.txt"
    );
    test_asm!(
        compare_expr,
        "./tests/compare_expr.nmt",
        "./tests/compare_expr.txt"
    );
    test_asm!(
        string_expr,
        "./tests/string_expr.nmt",
        "./tests/string_expr.txt"
    );
    test_asm!(structs, "./tests/structs.nmt", "./tests/structs.txt");
    test_asm!(loops, "./tests/loops.nmt", "./tests/loops.txt");
    test_asm!(
        conditions,
        "./tests/conditions.nmt",
        "./tests/conditions.txt"
    );
    test_asm!(functions, "./tests/functions.nmt", "./tests/functions.txt");
    test_asm!(assign, "./tests/assign.nmt", "./tests/assign.txt");
    test_asm!(arrays, "./tests/arrays.nmt", "./tests/arrays.txt");
}

mod elf {
    use super::*;

    test_elf!(
        binary_expr,
        "./tests/binary_expr.nmt",
        "./tests/binary_expr.txt"
    );
    test_elf!(
        cont_break,
        "./tests/cont_break.nmt",
        "./tests/cont_break.txt"
    );
    test_elf!(
        compare_expr,
        "./tests/compare_expr.nmt",
        "./tests/compare_expr.txt"
    );
    test_elf!(
        string_expr,
        "./tests/string_expr.nmt",
        "./tests/string_expr.txt"
    );
    test_elf!(structs, "./tests/structs.nmt", "./tests/structs.txt");
    test_elf!(loops, "./tests/loops.nmt", "./tests/loops.txt");
    test_elf!(
        conditions,
        "./tests/conditions.nmt",
        "./tests/conditions.txt"
    );
    test_elf!(functions, "./tests/functions.nmt", "./tests/functions.txt");
    test_elf!(assign, "./tests/assign.nmt", "./tests/assign.txt");
    test_elf!(arrays, "./tests/arrays.nmt", "./tests/arrays.txt");
}
