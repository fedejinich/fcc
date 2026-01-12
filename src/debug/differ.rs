//! Diff computation between snapshots.
//!
//! Provides simple line-by-line diff for V1.

use super::snapshot::{Change, ChangeType, Diff, DiffType};

/// Compute a line-by-line diff between two strings.
pub fn compute_diff(before: &str, after: &str) -> Option<Diff> {
    let before_lines: Vec<&str> = before.lines().collect();
    let after_lines: Vec<&str> = after.lines().collect();

    let mut changes = Vec::new();
    let max_len = before_lines.len().max(after_lines.len());

    for i in 0..max_len {
        let before_line = before_lines.get(i).copied();
        let after_line = after_lines.get(i).copied();

        match (before_line, after_line) {
            (Some(b), Some(a)) if b != a => {
                changes.push(Change {
                    line: i + 1,
                    change_type: ChangeType::Modified,
                    before: b.to_string(),
                    after: a.to_string(),
                });
            }
            (Some(b), None) => {
                changes.push(Change {
                    line: i + 1,
                    change_type: ChangeType::Removed,
                    before: b.to_string(),
                    after: String::new(),
                });
            }
            (None, Some(a)) => {
                changes.push(Change {
                    line: i + 1,
                    change_type: ChangeType::Added,
                    before: String::new(),
                    after: a.to_string(),
                });
            }
            _ => {}
        }
    }

    if changes.is_empty() {
        None
    } else {
        Some(Diff {
            diff_type: DiffType::LineDiff,
            changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_diff() {
        let result = compute_diff("hello\nworld", "hello\nworld");
        assert!(result.is_none());
    }

    #[test]
    fn test_modified_line() {
        let result = compute_diff("Var(\"x\")", "Var(\"x.0\")");
        let diff = result.expect("should have diff");
        assert_eq!(diff.changes.len(), 1);
        assert_eq!(diff.changes[0].change_type, ChangeType::Modified);
        assert_eq!(diff.changes[0].before, "Var(\"x\")");
        assert_eq!(diff.changes[0].after, "Var(\"x.0\")");
    }

    #[test]
    fn test_added_line() {
        let result = compute_diff("line1", "line1\nline2");
        let diff = result.expect("should have diff");
        assert_eq!(diff.changes.len(), 1);
        assert_eq!(diff.changes[0].change_type, ChangeType::Added);
    }

    #[test]
    fn test_removed_line() {
        let result = compute_diff("line1\nline2", "line1");
        let diff = result.expect("should have diff");
        assert_eq!(diff.changes.len(), 1);
        assert_eq!(diff.changes[0].change_type, ChangeType::Removed);
    }
}
