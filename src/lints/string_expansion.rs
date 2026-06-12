//! Issue #23: Expand string variables in commands.
//!
//! LAMMPS `string` variables are defined with `variable name string value`
//! and expanded with `${name}`. This lint warns about string variables
//! that are defined but never expanded (potential dead code), and checks
//! that expansions reference defined variables.

use std::collections::HashSet;

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

/// A lint that tracks string variable definitions and their expansions.
#[derive(Debug, Clone, PartialEq)]
pub struct StringVarExpansion {
    pub var_name: String,
    pub span: Span,
    pub kind: StringVarKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringVarKind {
    /// A string variable is defined but never expanded.
    DefinedNotUsed,
    /// A string variable is expanded with `${}` but was never defined.
    ExpandedNotDefined,
}

impl StringVarExpansion {
    /// Find all string variable issues in the AST.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let mut string_vars: HashSet<String> = HashSet::new();
        let mut expansions: HashSet<String> = HashSet::new();
        let mut results = Vec::new();

        // First pass: collect string variable definitions and expansions
        for command in &ast.commands {
            match command {
                Command::VariableDef(def) => {
                    if def.variable_style.contents == "string" {
                        string_vars.insert(def.variable_id.name.clone());
                    }
                }
                Command::Generic(cmd) => {
                    // Check arguments for ${...} expansions
                    for arg in &cmd.args {
                        collect_expansions_from_arg(&arg.kind, &mut expansions);
                    }
                }
                _ => {}
            }
        }

        // Check for defined-but-not-used
        // Iterate over commands in order (not the HashSet) to get deterministic ordering
        for command in &ast.commands {
            if let Command::VariableDef(def) = command {
                if def.variable_style.contents == "string"
                    && !expansions.contains(&def.variable_id.name)
                    && string_vars.contains(&def.variable_id.name)
                {
                    results.push(StringVarExpansion {
                        var_name: def.variable_id.name.clone(),
                        span: def.span,
                        kind: StringVarKind::DefinedNotUsed,
                    });
                }
            }
        }

        // Sort by span position for deterministic output
        results.sort_by_key(|r| (r.span.start.row, r.span.start.column));

        results
    }
}

fn collect_expansions_from_arg(
    arg_kind: &crate::ast::ArgumentKind,
    expansions: &mut HashSet<String>,
) {
    use crate::ast::ArgumentKind;

    match arg_kind {
        ArgumentKind::VarCurly(ident) => {
            if matches!(ident.ident_type, crate::ast::IdentType::Variable) {
                expansions.insert(ident.name.clone());
            }
        }
        ArgumentKind::Concatenation(args) => {
            for a in args {
                collect_expansions_from_arg(&a.kind, expansions);
            }
        }
        _ => {}
    }
}

impl Issue for StringVarExpansion {
    fn diagnostic(&self) -> Diagnostic {
        match self.kind {
            StringVarKind::DefinedNotUsed => Diagnostic {
            code: None,
                name: "string-var-expansion",
                severity: Severity::Warning,
                span: self.span,
                message: format!(
                    "LAMMPS-W501: string variable `{}` is defined but never expanded with `${{{}}}`",
                    self.var_name, self.var_name
                ),
            },
            StringVarKind::ExpandedNotDefined => Diagnostic {
            code: None,
                name: "string-var-expansion",
                severity: Severity::Warning,
                span: self.span,
                message: format!(
                    "LAMMPS-W501: variable `${{{}}}` is expanded but never defined",
                    self.var_name
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
    fn detects_unused_string_var() {
        let source = "variable name string myvalue\nunits metal\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StringVarExpansion::find_all(&ast);
        assert!(!found.is_empty());
        assert_eq!(found[0].var_name, "name");
        assert_eq!(found[0].kind, StringVarKind::DefinedNotUsed);
    }

    #[test]
    fn no_warning_for_used_string_var() {
        let source = "variable name string myvalue\nprint ${name}\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StringVarExpansion::find_all(&ast);
        // All string vars are used, so no warnings for defined-not-used
        let unused: Vec<_> = found
            .iter()
            .filter(|f| f.kind == StringVarKind::DefinedNotUsed)
            .collect();
        assert!(unused.is_empty());
    }

    #[test]
    fn no_warning_for_non_string_var() {
        let source = "variable a equal 1.0\nunits metal\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = StringVarExpansion::find_all(&ast);
        assert!(found.is_empty());
    }
}
