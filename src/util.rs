pub fn replace_c_with_i(file: &str) -> String {
    if let Some(stripped) = file.strip_suffix(".c") {
        format!("{stripped}.i")
    } else {
        file.to_string()
    }
}

pub fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{pad}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
