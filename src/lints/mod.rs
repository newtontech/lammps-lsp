//! Lint infrastructure for LAMMPS input scripts.
//!
//! This module provides lint rules with diagnostic codes following
//! the `LAMMPS-E###` (errors) and `LAMMPS-W###` (warnings) pattern.

use std::collections::HashMap;

use crate::diagnostics::{Diagnostic, Issue};
use crate::spans::Span;
use crate::{
    ast::{Ast, Ident},
    identifinder::IdentMap,
};

pub mod ampersand_comment;
pub mod circular_deps;
pub mod codes;
pub mod log_error;
pub mod missing_data;
pub mod missing_include;
pub mod rerun_detection;
pub mod sexp;
pub mod sim_box;
pub mod string_expansion;
pub mod style_order;
pub mod units_pair_style;

/// If a fix is redefined before it is run, the first definition is useless.
#[derive(Debug, PartialEq)]
struct MultiplyDefinedBeforeRun<'a> {
    defs: Vec<&'a Ident>,
}

#[derive(Debug)]
/// An identifier that has been defined multiple times within a single run.
pub(crate) struct RedfinedIdent<'a>(pub &'a Ident);

pub(crate) fn redefined_identifiers<'a>(
    ast: &'a Ast,
    idents: &'a IdentMap,
) -> impl Iterator<Item = RedfinedIdent<'a>> {
    fix_redef_before_run(ast, idents)
        .into_iter()
        // Don't show the warning on the first ident.
        // TODO: show an Information for it though
        .flat_map(|x| x.defs.into_iter().skip(1).map(RedfinedIdent))
}

/// Lint against fixes being redefined or deleted before they are run
/// Finds all fixes that meet this criteria.
fn fix_redef_before_run<'a>(
    ast: &'a Ast,
    indents: &'a IdentMap,
) -> Vec<MultiplyDefinedBeforeRun<'a>> {
    // All the commands that have multiple definitions.
    let multiply_defined = indents.values().filter(|v| v.defs().defs.len() > 1);

    let run_blocks: Vec<_> = ast.find_run_blocks().collect();

    // Count the number of times a fix is defined before the next run command
    let mut redefined = vec![];

    for v in multiply_defined {
        let defs = v.defs();

        let mut defs_by_blocks: HashMap<Span, Vec<&Ident>> = HashMap::new();

        for block in &run_blocks {
            for def in defs.iter() {
                if block.contains_span(&def.range()) {
                    defs_by_blocks.entry(*block).or_default().push(def);
                }
            }
        }

        for (_span, defs) in defs_by_blocks {
            if defs.len() > 1 {
                redefined.push(MultiplyDefinedBeforeRun { defs })
            }
        }
    }

    redefined
}

/// Run all lints on the AST and source text, collecting diagnostics.
///
/// This is the main entry point for the linting pipeline.
/// `base_dir` is used for file-existence checks (include, read_data).
/// Pass `None` to skip file-existence checks.
pub fn run_all_lints(ast: &Ast, source: &str) -> Vec<Diagnostic> {
    run_all_lints_with_dir(ast, source, None)
}

/// Run all lints with an optional base directory for file-existence checks.
pub fn run_all_lints_with_dir(
    ast: &Ast,
    source: &str,
    base_dir: Option<&std::path::Path>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Issue #14: Ampersand in comments
    for issue in ampersand_comment::AmpersandInComment::find_all(source) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #13: Circular variable dependencies
    for issue in circular_deps::CircularVariableDependency::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #24: Simulation box check
    for issue in sim_box::SimBoxNotDefined::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #25: Style definition order
    for issue in style_order::StyleAfterDataRead::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #23: String variable expansion
    for issue in string_expansion::StringVarExpansion::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #8: Rerun detection
    for issue in rerun_detection::RerunDetection::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #19: Missing include file
    for issue in missing_include::MissingInclude::find_all(ast, base_dir) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #20: Missing data file
    for issue in missing_data::MissingDataFile::find_all(ast, base_dir) {
        diagnostics.push(issue.diagnostic());
    }

    // Issue #21: Suspicious units/pair_style combination
    for issue in units_pair_style::SuspiciousUnitsPairStyle::find_all(ast) {
        diagnostics.push(issue.diagnostic());
    }

    diagnostics
}

/// Run log-parsing lints (Issue #22) on text that may be a LAMMPS log file.
pub fn run_log_lints(log_text: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Issue #22: LAMMPS ERROR lines in log output
    for issue in log_error::LammpsLogError::find_all(log_text) {
        diagnostics.push(issue.diagnostic());
    }

    diagnostics
}

impl Issue for RedfinedIdent<'_> {
    fn diagnostic(&self) -> crate::diagnostics::Diagnostic {
        let ident = self.0;
        crate::diagnostics::Diagnostic {
            code: None,
            name: "redefined before `run` command",
            severity: crate::diagnostics::Severity::Warning,
            span: ident.span,
            message: format![
                "LAMMPS-W201: {} `{}` defined multiple times before run command",
                ident.ident_type, ident.name
            ],
        }
    }
}

/// Export all lint rule metadata as JSON for agent consumption (Issue #5/#12).
pub fn export_rules_json() -> serde_json::Value {
    use codes::LintCode;

    let rules = vec![
        rule_entry(
            LintCode::AmpersandInComment,
            "error",
            "Ampersand continuation in comments is invalid",
        ),
        rule_entry(
            LintCode::UnterminatedString,
            "error",
            "Unterminated string literal",
        ),
        rule_entry(
            LintCode::UnknownCommand,
            "warning",
            "Unknown LAMMPS command",
        ),
        rule_entry(
            LintCode::RedefinedBeforeRun,
            "warning",
            "Identifier redefined before run command",
        ),
        rule_entry(LintCode::UnusedIdentifier, "warning", "Unused identifier"),
        rule_entry(
            LintCode::UndefinedIdentifier,
            "error",
            "Undefined identifier reference",
        ),
        rule_entry(
            LintCode::InvalidArguments,
            "warning",
            "Invalid command arguments",
        ),
        rule_entry(LintCode::InvalidFixArgs, "warning", "Invalid fix arguments"),
        rule_entry(
            LintCode::InvalidComputeArgs,
            "warning",
            "Invalid compute arguments",
        ),
        rule_entry(LintCode::InvalidFixStyle, "error", "Unknown fix style"),
        rule_entry(
            LintCode::InvalidComputeStyle,
            "error",
            "Unknown compute style",
        ),
        rule_entry(LintCode::InvalidPairStyle, "error", "Unknown pair style"),
        rule_entry(
            LintCode::StyleAfterDataRead,
            "warning",
            "Style defined after data read",
        ),
        rule_entry(
            LintCode::SimBoxNotDefined,
            "error",
            "Simulation box not defined",
        ),
        rule_entry(
            LintCode::MissingCreateBox,
            "warning",
            "Missing create_box command",
        ),
        rule_entry(
            LintCode::MissingRegion,
            "warning",
            "Missing region definition",
        ),
        rule_entry(
            LintCode::CircularVariableDependency,
            "error",
            "Circular variable dependency",
        ),
        rule_entry(
            LintCode::StringVarExpansion,
            "warning",
            "String variable expansion issue",
        ),
        rule_entry(
            LintCode::InvalidVarReference,
            "error",
            "Invalid variable reference",
        ),
        rule_entry(LintCode::RerunInDocs, "warning", "Rerun command detected"),
        rule_entry(LintCode::MissingInclude, "error", "Missing include file"),
        rule_entry(LintCode::MissingDataFile, "error", "Missing data file"),
        rule_entry(
            LintCode::SuspiciousUnitsPairStyle,
            "warning",
            "Suspicious units/pair_style combination",
        ),
        rule_entry(
            LintCode::LammpsLogError,
            "error",
            "LAMMPS ERROR line in log output",
        ),
    ];

    serde_json::json!({
        "diagnostic_engine": "1.0",
        "software": "lammps",
        "source": "lammps-lsp",
        "rules": rules,
    })
}

fn rule_entry(code: codes::LintCode, severity: &str, description: &str) -> serde_json::Value {
    serde_json::json!({
        "code": code.to_string(),
        "label": code.label(),
        "severity": severity,
        "description": description,
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{ast::ts_to_ast, identifinder::IdentiFinder, utils};

    use super::*;

    #[test]
    fn test_multiple_fix_defs() {
        let text = "fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             run 10000
                             fix NVT all nvt temp 1 1.5 $(100.0*dt)";

        let tree = utils::testing::parse(text);

        let ast = ts_to_ast(&tree, text).unwrap();

        let mut idents = IdentiFinder::new(&tree, text).unwrap();
        idents
            .find_symbols(&tree, text)
            .expect("Failed to Find Symbols");

        let v = fix_redef_before_run(&ast, idents.symbols());
        assert_eq!(v.len(), 1);
        dbg!(v);
    }

    #[test]
    fn redefined_fix_after_run() {
        let text = "fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             run 10000
                             fix NVT all nvt temp 1 1.5 $(100.0*dt)";

        let tree = utils::testing::parse(text);

        let ast = ts_to_ast(&tree, text).unwrap();

        let mut idents = IdentiFinder::new(&tree, text).unwrap();
        idents
            .find_symbols(&tree, text)
            .expect("Failed to Find Symbols");

        assert_eq!(fix_redef_before_run(&ast, idents.symbols()), vec![]);
    }

    #[test]
    fn redefined_twice_after_first_run() {
        let text = "fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             run 10000
                             fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             fix NVT all nvt temp 1 1.5 $(100.0*dt)
                             run 20000
";

        let tree = utils::testing::parse(text);

        let ast = ts_to_ast(&tree, text).unwrap();

        let mut idents = IdentiFinder::new(&tree, text).unwrap();
        idents
            .find_symbols(&tree, text)
            .expect("Failed to Find Symbols");

        let v = fix_redef_before_run(&ast, idents.symbols());
        assert_eq!(v.len(), 1);
        dbg!(v);
    }

    #[test]
    fn run_all_lints_no_issues() {
        let source = "units metal\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();

        let diags = run_all_lints(&ast, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn run_all_lints_with_ampersand_comment() {
        let source = "# this is a comment &\nunits metal\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();

        let diags = run_all_lints(&ast, source);
        assert!(diags.iter().any(|d| d.name == "ampersand-in-comment"));
    }

    #[test]
    fn export_rules_json_is_valid() {
        let json = export_rules_json();
        assert_eq!(json["diagnostic_engine"], "1.0");
        let rules = json["rules"].as_array().expect("rules should be array");
        assert!(!rules.is_empty());
        // Check every rule has required fields
        for rule in rules {
            assert!(rule.get("code").is_some());
            assert!(rule.get("label").is_some());
            assert!(rule.get("severity").is_some());
            assert!(rule.get("description").is_some());
        }
    }
}
