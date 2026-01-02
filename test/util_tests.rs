use fcc::common::util::{indent, opt_box, replace_c_with_i};

#[test]
fn test_replace_c_with_i_valid_c_file() {
    assert_eq!(replace_c_with_i("main.c"), "main.i");
    assert_eq!(replace_c_with_i("test.c"), "test.i");
    assert_eq!(replace_c_with_i("program.c"), "program.i");
}

#[test]
fn test_replace_c_with_i_path_with_directories() {
    assert_eq!(replace_c_with_i("src/main.c"), "src/main.i");
    assert_eq!(replace_c_with_i("/path/to/file.c"), "/path/to/file.i");
    assert_eq!(replace_c_with_i("./test.c"), "./test.i");
}

#[test]
fn test_replace_c_with_i_no_extension() {
    assert_eq!(replace_c_with_i("main"), "main");
    assert_eq!(replace_c_with_i("test"), "test");
}

#[test]
fn test_replace_c_with_i_different_extension() {
    assert_eq!(replace_c_with_i("main.cpp"), "main.cpp");
    assert_eq!(replace_c_with_i("test.h"), "test.h");
    assert_eq!(replace_c_with_i("file.txt"), "file.txt");
}

#[test]
fn test_replace_c_with_i_multiple_dots() {
    assert_eq!(replace_c_with_i("test.backup.c"), "test.backup.i");
    assert_eq!(replace_c_with_i("file.1.2.c"), "file.1.2.i");
}

#[test]
fn test_replace_c_with_i_empty_string() {
    assert_eq!(replace_c_with_i(""), "");
}

#[test]
fn test_replace_c_with_i_just_dot_c() {
    assert_eq!(replace_c_with_i(".c"), ".i");
}

#[test]
fn test_indent_single_line() {
    assert_eq!(indent("hello", 2), "  hello");
    assert_eq!(indent("world", 4), "    world");
    assert_eq!(indent("test", 0), "test");
}

#[test]
fn test_indent_multiple_lines() {
    let input = "line1\nline2\nline3";
    let expected = "  line1\n  line2\n  line3";
    assert_eq!(indent(input, 2), expected);
}

#[test]
fn test_indent_empty_string() {
    assert_eq!(indent("", 2), "");
}

#[test]
fn test_indent_empty_lines() {
    let input = "line1\n\nline3";
    let expected = "    line1\n    \n    line3";
    assert_eq!(indent(input, 4), expected);
}

#[test]
fn test_indent_single_empty_line() {
    let input = "\n";
    let expected = "  ";
    assert_eq!(indent(input, 2), expected);
}

#[test]
fn test_indent_trailing_newline() {
    let input = "line1\nline2\n";
    let expected = "  line1\n  line2";
    assert_eq!(indent(input, 2), expected);
}

#[test]
fn test_indent_large_indentation() {
    assert_eq!(indent("test", 8), "        test");
}

#[test]
fn test_indent_complex_multiline() {
    let input = "int main() {\n    return 0;\n}";
    let expected = "  int main() {\n      return 0;\n  }";
    assert_eq!(indent(input, 2), expected);
}

// =============================================================================
// OPT_BOX
// =============================================================================

#[test]
fn test_opt_box_some() {
    let result = opt_box(Some(42));
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn test_opt_box_none() {
    let result: Option<Box<i32>> = opt_box(None);
    assert!(result.is_none());
}

#[test]
fn test_opt_box_string() {
    let result = opt_box(Some("hello".to_string()));
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), "hello".to_string());
}
