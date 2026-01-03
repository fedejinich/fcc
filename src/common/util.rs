use std::sync::atomic::{AtomicUsize, Ordering};

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

pub fn temporary_name(name: &str, counter: &AtomicUsize) -> String {
    let id = counter.fetch_add(1, Ordering::Relaxed);
    format!("{name}.{id}")
}

pub fn opt_box<T>(opt: Option<T>) -> Option<Box<T>> {
    if let Some(t) = opt {
        return Some(Box::new(t));
    }
    None
}
