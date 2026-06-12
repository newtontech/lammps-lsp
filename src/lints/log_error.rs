//! Issue #22: lammps.log.error (LAMMPS-E900)
//!
//! Detect LAMMPS ERROR lines in log output. LAMMPS outputs errors
//! with the format:
//!   `ERROR: <message>`
//!   `ERROR on proc <n>: <message>`
//!
//! This lint operates on log file content rather than input scripts,
//! and is used by the log-parsing pipeline.

use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::lints::codes::LintCode;
use crate::spans::{Point, Span};

/// A LAMMPS error line found in log output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LammpsLogError {
    pub message: String,
    pub span: Span,
}

impl LammpsLogError {
    /// Scan text (typically log file content) for LAMMPS ERROR lines.
    ///
    /// Returns one diagnostic per ERROR line found.
    pub fn find_all(text: &str) -> Vec<Self> {
        let mut issues = Vec::new();

        for (i, line) in text.lines().enumerate() {
            let trimmed = line.trim();

            // Match "ERROR: ..." or "ERROR on proc ...: ..."
            if trimmed.starts_with("ERROR") {
                let col = line.find("ERROR").unwrap_or(0);
                let end_col = col + trimmed.len().min(line.len() - col);
                let message = if let Some(rest) = trimmed.strip_prefix("ERROR: ") {
                    rest.to_string()
                } else if let Some(rest) = trimmed.strip_prefix("ERROR on proc") {
                    format!("proc {rest}")
                } else {
                    trimmed.to_string()
                };

                issues.push(Self {
                    message,
                    span: Span {
                        start: Point {
                            row: i,
                            column: col,
                        },
                        end: Point {
                            row: i,
                            column: end_col,
                        },
                    },
                });
            }
        }

        issues
    }
}

impl Issue for LammpsLogError {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            name: LintCode::LammpsLogError.label(),
            severity: Severity::Error,
            span: self.span,
            message: format!(
                "{}: LAMMPS error detected: {}",
                LintCode::LammpsLogError,
                self.message
            ),
            code: Some(LintCode::LammpsLogError.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_simple_error() {
        let log = "LAMMPS (29 Oct 2020)\nERROR: Invalid command\n";
        let issues = LammpsLogError::find_all(log);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Invalid command"));
    }

    #[test]
    fn detects_proc_error() {
        let log = "ERROR on proc 0: Lost atoms\n";
        let issues = LammpsLogError::find_all(log);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Lost atoms"));
    }

    #[test]
    fn no_errors_in_clean_log() {
        let log = "LAMMPS (29 Oct 2020)\nStep Temp\n0 300.0\n100 310.0\n";
        let issues = LammpsLogError::find_all(log);
        assert!(issues.is_empty());
    }

    #[test]
    fn multiple_errors() {
        let log = "ERROR: First error\nSome output\nERROR: Second error\n";
        let issues = LammpsLogError::find_all(log);
        assert_eq!(issues.len(), 2);
    }
}
