//! Issue #25: Ensure pair/angle/bond styles are defined before data read.
//!
//! In LAMMPS, interaction styles (pair_style, bond_style, angle_style, etc.)
//! should be defined before `read_data` or `create_atoms` commands.
//! This lint warns when styles are defined after data is read.

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

/// Commands that load atom data.
const DATA_READING_COMMANDS: &[&str] = &["read_data", "read_restart", "create_atoms"];

/// Style-setting commands that should come before data read.
const STYLE_COMMANDS: &[&str] = &[
    "pair_style",
    "pair_coeff",
    "bond_style",
    "bond_coeff",
    "angle_style",
    "angle_coeff",
    "dihedral_style",
    "dihedral_coeff",
    "improper_style",
    "improper_coeff",
    "kspace_style",
];

/// A diagnostic for a style defined after data is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleAfterDataRead {
    pub style_command: String,
    pub span: Span,
}

impl StyleAfterDataRead {
    /// Check for style definitions that appear after data-reading commands.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let mut results = Vec::new();
        let mut data_read = false;

        for command in &ast.commands {
            if let Command::Generic(cmd) = command {
                let name = cmd.name.contents.as_str();

                if DATA_READING_COMMANDS.contains(&name) {
                    data_read = true;
                } else if data_read && STYLE_COMMANDS.contains(&name) {
                    results.push(StyleAfterDataRead {
                        style_command: name.to_string(),
                        span: cmd.span(),
                    });
                }
            }
        }

        results
    }
}

impl Issue for StyleAfterDataRead {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            code: None,
            name: "style-after-data-read",
            severity: Severity::Warning,
            span: self.span,
            message: format!(
                "LAMMPS-W303: `{}` defined after data is read; \
                 consider defining styles before `read_data`",
                self.style_command
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn detects_style_after_data_read() {
        let source = "units metal\nread_data data.lmp\npair_style eam/alloy\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StyleAfterDataRead::find_all(&ast);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].style_command, "pair_style");
    }

    #[test]
    fn no_warning_when_style_before_data() {
        let source = "units metal\npair_style eam/alloy\nread_data data.lmp\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StyleAfterDataRead::find_all(&ast);
        assert!(found.is_empty());
    }

    #[test]
    fn detects_multiple_styles_after_data() {
        let source = "units metal\nread_data data.lmp\npair_style lj/cut\nbond_style harmonic\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StyleAfterDataRead::find_all(&ast);
        assert_eq!(found.len(), 2);
    }
}
