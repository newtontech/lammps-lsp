//! Issue #8: Add rerun detection for docs files.
//!
//! When working with LAMMPS documentation files (`.md` code blocks),
//! detects `rerun` commands that may indicate the script is meant for
//! re-processing rather than fresh simulation. Also flags `rerun` usage
//! in general as it requires special setup.

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

/// A diagnostic for `rerun` command usage, which requires special handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerunDetection {
    pub span: Span,
    pub context: RerunContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RerunContext {
    /// The rerun command was found in the script.
    RerunCommand,
    /// A dump file referenced by rerun was not defined earlier.
    MissingDumpFile(String),
}

impl RerunDetection {
    /// Detect rerun commands and validate their setup.
    ///
    /// Checks:
    /// 1. `rerun` commands require a prior `dump` command to produce the dump file.
    /// 2. Warns about `rerun` usage as it implies re-processing.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let mut results = Vec::new();
        let mut has_dump = false;

        for command in &ast.commands {
            if let Command::Generic(cmd) = command {
                match cmd.name.contents.as_str() {
                    "dump" => {
                        has_dump = true;
                    }
                    "rerun" => {
                        // Check if a dump file argument is provided
                        let dump_file = cmd
                            .args
                            .first()
                            .and_then(|arg| match &arg.kind {
                                crate::ast::ArgumentKind::String(s)
                                | crate::ast::ArgumentKind::Word(s) => Some(s.clone()),
                                crate::ast::ArgumentKind::RawString(s) => Some(s.clone()),
                                _ => None,
                            })
                            .unwrap_or_default();

                        if !has_dump && !dump_file.is_empty() {
                            results.push(RerunDetection {
                                span: cmd.span(),
                                context: RerunContext::MissingDumpFile(dump_file),
                            });
                        } else {
                            results.push(RerunDetection {
                                span: cmd.span(),
                                context: RerunContext::RerunCommand,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        results
    }

    /// Check if a file appears to be a documentation file based on its extension.
    pub fn is_docs_file(filename: &str) -> bool {
        filename.ends_with(".md")
            || filename.ends_with(".rst")
            || filename.ends_with(".txt")
            || filename.ends_with(".tex")
    }
}

impl Issue for RerunDetection {
    fn diagnostic(&self) -> Diagnostic {
        match &self.context {
            RerunContext::RerunCommand => Diagnostic {
            code: None,
                name: "rerun-in-docs",
                severity: Severity::Info,
                span: self.span,
                message: "LAMMPS-W600: `rerun` command detected; \
                    ensure the referenced dump file exists"
                    .to_string(),
            },
            RerunContext::MissingDumpFile(file) => Diagnostic {
            code: None,
                name: "rerun-in-docs",
                severity: Severity::Warning,
                span: self.span,
                message: format!(
                    "LAMMPS-W600: `rerun` references `{}` but no preceding `dump` command found",
                    file
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn detects_rerun() {
        let source = "units metal\nrerun dump.file\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = RerunDetection::find_all(&ast);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].context, RerunContext::MissingDumpFile("dump.file".to_string()));
    }

    #[test]
    fn rerun_with_dump_no_missing_file_warning() {
        let source = "units metal\ndump 1 all atom 100 dump.file\nrerun dump.file\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = RerunDetection::find_all(&ast);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].context, RerunContext::RerunCommand);
    }

    #[test]
    fn no_rerun_no_warning() {
        let source = "units metal\nrun 100\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = RerunDetection::find_all(&ast);
        assert!(found.is_empty());
    }

    #[test]
    fn docs_file_detection() {
        assert!(RerunDetection::is_docs_file("readme.md"));
        assert!(RerunDetection::is_docs_file("docs.rst"));
        assert!(!RerunDetection::is_docs_file("in.metal"));
    }
}
