//! Issue #24: Check simulation box is defined.
//!
//! Many LAMMPS commands require a simulation box to be set up first.
//! This lint checks that `create_box` or `read_data` or `read_restart`
//! appears before commands that require a simulation box, such as
//! `create_atoms`, `run`, `minimize`, etc.

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

/// Commands that create a simulation box.
const BOX_DEFINING_COMMANDS: &[&str] = &["create_box", "read_data", "read_restart"];

/// Commands that require a simulation box to exist.
const BOX_REQUIRING_COMMANDS: &[&str] = &[
    "create_atoms",
    "run",
    "minimize",
    "run_style",
    "fix",
    "compute",
    "velocity",
    "pair_coeff",
    "bond_coeff",
    "angle_coeff",
    "dihedral_coeff",
    "improper_coeff",
    "kspace_style",
    "thermo",
    "thermo_style",
    "dump",
    "reset_timestep",
];

/// A diagnostic indicating a command that requires a simulation box was used before one was defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimBoxNotDefined {
    pub command_name: String,
    pub span: Span,
}

impl SimBoxNotDefined {
    /// Scan the AST for commands that require a simulation box but appear
    /// before any box-defining command.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let mut results = Vec::new();
        let mut box_defined = false;

        for command in &ast.commands {
            match command {
                Command::Generic(cmd) => {
                    let name = cmd.name.contents.as_str();

                    if BOX_DEFINING_COMMANDS.contains(&name) {
                        box_defined = true;
                    } else if !box_defined && BOX_REQUIRING_COMMANDS.contains(&name) {
                        results.push(SimBoxNotDefined {
                            command_name: name.to_string(),
                            span: cmd.span(),
                        });
                    }
                }
                Command::Fix(_) | Command::Compute(_) if !box_defined => {
                    results.push(SimBoxNotDefined {
                        command_name: command.span().start.row.to_string(),
                        span: command.span(),
                    });
                }
                _ => {}
            }
        }

        results
    }
}

impl Issue for SimBoxNotDefined {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            code: None,
            name: "sim-box-not-defined",
            severity: Severity::Error,
            span: self.span,
            message: format!(
                "LAMMPS-E400: command `{}` used before simulation box is defined \
                 (add `create_box` or `read_data` first)",
                self.command_name
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn detects_missing_box() {
        let source = "units metal\nrun 100\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = SimBoxNotDefined::find_all(&ast);
        assert!(!found.is_empty(), "should detect missing simulation box");
    }

    #[test]
    fn no_error_with_box() {
        let source = "units metal\ncreate_box 1 box\nrun 100\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = SimBoxNotDefined::find_all(&ast);
        assert!(found.is_empty(), "should not error when box is defined");
    }

    #[test]
    fn no_error_with_read_data() {
        let source = "units metal\nread_data data.lmp\nrun 100\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = SimBoxNotDefined::find_all(&ast);
        assert!(found.is_empty(), "read_data defines a box");
    }

    #[test]
    fn empty_script_no_error() {
        let source = "";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = SimBoxNotDefined::find_all(&ast);
        assert!(found.is_empty());
    }
}
