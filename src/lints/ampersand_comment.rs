//! Issue #14: Warn against `&` in comments.
//!
//! In LAMMPS, `&` is the line-continuation character. When it appears in a comment,
//! it may indicate a misunderstanding — the user might think they are continuing
//! the command, but LAMMPS treats `# ... &` as a continuation.

use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::{Point, Span};

/// A lint that detects `&` at the end of a comment line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmpersandInComment {
    pub span: Span,
}

impl AmpersandInComment {
    /// Scan source text for comment lines ending with `&` (ignoring trailing whitespace).
    pub fn find_all(source: &str) -> Vec<Self> {
        let mut results = Vec::new();

        for (row, line) in source.lines().enumerate() {
            let trimmed = line.trim_end();

            // Only consider lines that are comments (start with #)
            if !trimmed.starts_with('#') {
                continue;
            }

            // Check if the comment ends with &
            if trimmed.ends_with('&') {
                let col_end = trimmed.len();
                let col_start = col_end - 1;
                results.push(AmpersandInComment {
                    span: Span {
                        start: Point {
                            row,
                            column: col_start,
                        },
                        end: Point {
                            row,
                            column: col_end,
                        },
                    },
                });
            }
        }

        results
    }
}

impl Issue for AmpersandInComment {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            code: None,
            name: "ampersand-in-comment",
            severity: Severity::Warning,
            span: self.span,
            message: "LAMMPS-E100: `&` in a comment continues to the next line; \
                consider removing it or moving the continuation outside the comment"
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_ampersand_in_comment() {
        let source = "# this is a comment &\nunits metal\n";
        let found = AmpersandInComment::find_all(source);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].span.start.row, 0);
        assert_eq!(found[0].span.start.column, 20);
    }

    #[test]
    fn no_ampersand_in_comment() {
        let source = "# this is a comment\nunits metal\n";
        let found = AmpersandInComment::find_all(source);
        assert!(found.is_empty());
    }

    #[test]
    fn ignores_ampersand_in_command() {
        let source = "units metal &\n  lj\n";
        let found = AmpersandInComment::find_all(source);
        assert!(found.is_empty());
    }

    #[test]
    fn detects_multiple_ampersand_comments() {
        let source = "# comment 1 &\nunits metal\n# comment 2 &\ndimension 3\n";
        let found = AmpersandInComment::find_all(source);
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn ignores_trailing_whitespace_after_ampersand() {
        // The `&` is not the last non-whitespace character, but the line ends with `&` after trim
        let source = "# comment &   \n";
        let found = AmpersandInComment::find_all(source);
        assert_eq!(found.len(), 1);
    }
}
