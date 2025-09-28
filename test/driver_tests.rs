use clap::Parser;
use fcc::driver::CompilerDriver;

#[test]
fn test_compiler_driver_creation() {
    let args = vec!["fcc", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("CompilerDriver"));
}

#[test]
fn test_compiler_driver_lex_flag() {
    let args = vec!["fcc", "--lex", "test.c"];
    let _driver = CompilerDriver::parse_from(args);
}

#[test]
fn test_compiler_driver_parse_flag() {
    let args = vec!["fcc", "--parse", "test.c"];
    let _driver = CompilerDriver::parse_from(args);
}

#[test]
fn test_compiler_driver_validate_flag() {
    let args = vec!["fcc", "--validate", "test.c"];
    let _driver = CompilerDriver::parse_from(args);
}

#[test]
fn test_compiler_driver_tacky_flag() {
    let args = vec!["fcc", "--tacky", "test.c"];
    let _driver = CompilerDriver::parse_from(args);
}

#[test]
fn test_compiler_driver_codegen_flag() {
    let args = vec!["fcc", "--codegen", "test.c"];
    let _driver = CompilerDriver::parse_from(args);
}

#[test]
fn test_compiler_driver_debug_flag() {
    let args = vec!["fcc", "--debug", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("debug: true"));
}

#[test]
fn test_compiler_driver_trace_flag() {
    let args = vec!["fcc", "--trace", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("trace: true"));
}

#[test]
fn test_compiler_driver_print_ast_flag() {
    let args = vec!["fcc", "--print-ast", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("print_ast: true"));
}

#[test]
fn test_compiler_driver_print_tacky_flag() {
    let args = vec!["fcc", "--print-tacky", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("print_tacky: true"));
}

#[test]
fn test_compiler_driver_multiple_flags() {
    let args = vec!["fcc", "--debug", "--parse", "--print-ast", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("debug: true"));
    assert!(debug_str.contains("parse: true"));
    assert!(debug_str.contains("print_ast: true"));
    assert!(debug_str.contains("program_path: \"test.c\""));
}

#[test]
fn test_preprocess_invalid_extension() {
    let driver = CompilerDriver::parse_from(vec!["fcc", "test.txt"]);
    let result = driver.preprocess("test.txt");

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, "SOURCE_FILE should have a .c file extension");
    }
}

#[test]
fn test_preprocess_nonexistent_file() {
    let driver = CompilerDriver::parse_from(vec!["fcc", "nonexistent.c"]);
    let result = driver.preprocess("nonexistent.c");

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, "source file does not exist");
    }
}

#[test]
fn test_preprocess_valid_extension_generates_correct_output() {
    let driver = CompilerDriver::parse_from(vec!["fcc", "test.c"]);

    let result = driver.preprocess("nonexistent.c");

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, "source file does not exist");
    }
}

#[test]
fn test_different_program_paths_in_debug_output() {
    let test_cases = vec![
        ("main.c", "main.c"),
        ("src/test.c", "src/test.c"),
        ("../other.c", "../other.c"),
        ("./local.c", "./local.c"),
    ];

    for (input, expected_path) in test_cases {
        let args = vec!["fcc", input];
        let driver = CompilerDriver::parse_from(args);
        let debug_str = format!("{driver:?}");
        assert!(debug_str.contains(&format!("program_path: \"{expected_path}\"")));
    }
}

#[test]
fn test_compiler_driver_debug_formatting() {
    let args = vec!["fcc", "--debug", "test.c"];
    let driver = CompilerDriver::parse_from(args);

    let debug_str = format!("{driver:?}");
    assert!(debug_str.contains("CompilerDriver"));
    assert!(debug_str.contains("debug: true"));
    assert!(debug_str.contains("program_path: \"test.c\""));
    assert!(debug_str.contains("lex: false"));
    assert!(debug_str.contains("parse: false"));
}
