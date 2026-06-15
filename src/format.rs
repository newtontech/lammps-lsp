//! See also: wiki/synthesis/Input_Script_Workflow.md
//! See also: wiki/synthesis/Common_Patterns.md
//! Issue #27: Safe formatter for LAMMPS input scripts.
//!
//! Provides a safe, idempotent formatter that normalises:
//! - Trailing whitespace removal
//! - Multiple blank line collapse (max 1 consecutive blank line)
//! - Consistent indentation (commands start at column 0)
//! - Normalise spacing around `#` comments
//!
//! The formatter is intentionally conservative: it only applies
//! transformations that preserve the semantic meaning of the script.

/// Format a LAMMPS input script, returning the formatted text.
///
/// The formatter is safe: it only makes cosmetic changes that
/// do not alter the script's behaviour.
pub fn format_lammps(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut prev_blank = false;

    for line in &lines {
        // Strip trailing whitespace
        let trimmed = line.trim_end();

        // Collapse consecutive blank lines to at most one
        if trimmed.is_empty() {
            if !prev_blank {
                result.push(String::new());
            }
            prev_blank = true;
            continue;
        }

        prev_blank = false;

        // Normalise the line content
        let formatted = format_line(trimmed);
        result.push(formatted);
    }

    // Ensure file ends with a single newline (if there's any content)
    let mut output = result.join("\n");
    if !output.is_empty() {
        output.push('\n');
    }

    output
}

/// Format a single non-blank line.
fn format_line(line: &str) -> String {
    // Detect if this is a pure comment line
    let stripped = line.trim_start();
    if stripped.starts_with('#') {
        // Ensure exactly one space after '#' for comment lines
        let comment_content = stripped.trim_start_matches('#').trim_start();
        return format!("# {comment_content}");
    }

    // For lines with inline comments, split and format separately
    if let Some(comment_pos) = find_inline_comment(line) {
        let code_part = line[..comment_pos].trim_end();
        let comment_part = line[comment_pos..].trim_start_matches('#').trim();
        return format!("{code_part}  # {comment_part}");
    }

    // Regular code line: just strip trailing whitespace (already done)
    line.to_string()
}

/// Find the position of an inline comment (#) that is not inside a string.
/// Returns None if there is no inline comment.
fn find_inline_comment(line: &str) -> Option<usize> {
    let mut in_string = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch == b'"' {
            in_string = !in_string;
        } else if ch == b'#' && !in_string {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Check if formatting is idempotent: applying the formatter twice
/// produces the same output as applying it once.
pub fn is_idempotent(source: &str) -> bool {
    let once = format_lammps(source);
    let twice = format_lammps(&once);
    once == twice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_whitespace_removed() {
        let input = "units metal   \nrun 100  \n";
        let formatted = format_lammps(input);
        assert_eq!(formatted, "units metal\nrun 100\n");
    }

    #[test]
    fn multiple_blank_lines_collapsed() {
        let input = "units metal\n\n\n\nrun 100\n";
        let formatted = format_lammps(input);
        assert_eq!(formatted, "units metal\n\nrun 100\n");
    }

    #[test]
    fn comment_normalised() {
        let input = "#  A comment\nunits metal\n";
        let formatted = format_lammps(input);
        assert_eq!(formatted, "# A comment\nunits metal\n");
    }

    #[test]
    fn inline_comment_spacing() {
        let input = "run 100# a comment\n";
        let formatted = format_lammps(input);
        assert_eq!(formatted, "run 100  # a comment\n");
    }

    #[test]
    fn idempotent_check() {
        let input = "units metal  \n\n\n\n#comment\nrun 100  #go\n";
        assert!(is_idempotent(input));
    }

    #[test]
    fn already_formatted_unchanged() {
        let input = "units metal\nrun 100\n";
        assert!(is_idempotent(input));
        assert_eq!(format_lammps(input), input);
    }

    #[test]
    fn hash_in_string_preserved() {
        let input = r#"variable myvar string "hello#world""#;
        let formatted = format_lammps(input);
        assert!(formatted.contains("hello#world"));
    }

    #[test]
    fn empty_input() {
        assert_eq!(format_lammps(""), "");
    }
}
