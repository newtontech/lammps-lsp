//! Issue #21: lammps.units.pair_style_suspicious (LAMMPS-W800)
//!
//! Warn on suspicious combinations of `units` and `pair_style`.
//! Some pair styles are designed for specific unit systems:
//! - `lj` and `lj/cut` are for `lj` units
//! - `eam`, `eam/alloy`, `eam/fs` are typically for `metal` units
//! - `reaxff` is for `real` or `metal` units

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::lints::codes::LintCode;
use crate::spans::Span;

/// Pair styles that are typically only valid with specific unit systems.
struct StyleUnitExpectation {
    style: &'static str,
    expected_units: &'static [&'static str],
    reason: &'static str,
}

const EXPECTATIONS: &[StyleUnitExpectation] = &[
    StyleUnitExpectation {
        style: "lj/cut",
        expected_units: &["lj"],
        reason: "lj/cut is designed for reduced LJ units",
    },
    StyleUnitExpectation {
        style: "lj/charmm/coul/long",
        expected_units: &["real"],
        reason: "lj/charmm/coul/long is designed for real units (kcal/mol)",
    },
];

/// A diagnostic for a suspicious units/pair_style combination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspiciousUnitsPairStyle {
    pub units: String,
    pub pair_style: String,
    pub reason: String,
    pub span: Span,
}

impl SuspiciousUnitsPairStyle {
    /// Find all suspicious units/pair_style combinations.
    ///
    /// Scans the script for `units` and `pair_style` commands and checks
    /// if the combination is known to be problematic.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let mut units_value: Option<String> = None;
        let mut pair_style_value: Option<String> = None;
        let mut pair_style_span: Option<Span> = None;

        for command in &ast.commands {
            let Command::Generic(cmd) = &command else {
                continue;
            };

            if cmd.name.contents == "units" {
                if let Some(arg) = cmd.args.first() {
                    if let crate::ast::ArgumentKind::Word(word) = &arg.kind {
                        units_value = Some(word.clone());
                    }
                }
            } else if cmd.name.contents == "pair_style" {
                if let Some(arg) = cmd.args.first() {
                    if let crate::ast::ArgumentKind::Word(word) = &arg.kind {
                        pair_style_value = Some(word.clone());
                        pair_style_span = Some(cmd.args[0].span);
                    }
                }
            }
        }

        let (Some(units), Some(pair_style), Some(span)) =
            (units_value, pair_style_value, pair_style_span)
        else {
            return Vec::new();
        };

        let units_lower = units.to_lowercase();

        for exp in EXPECTATIONS {
            if pair_style == exp.style && !exp.expected_units.contains(&units_lower.as_str()) {
                return vec![Self {
                    units,
                    pair_style,
                    reason: exp.reason.to_string(),
                    span,
                }];
            }
        }

        Vec::new()
    }
}

impl Issue for SuspiciousUnitsPairStyle {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            name: LintCode::SuspiciousUnitsPairStyle.label(),
            severity: Severity::Warning,
            span: self.span,
            message: format!(
                "{}: pair_style `{}` may be incompatible with units `{}` ({})",
                LintCode::SuspiciousUnitsPairStyle,
                self.pair_style,
                self.units,
                self.reason
            ),
            code: Some(LintCode::SuspiciousUnitsPairStyle.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::ts_to_ast, utils};

    #[test]
    fn suspicious_lj_cut_with_metal() {
        let source = "units metal\npair_style lj/cut 2.5\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = SuspiciousUnitsPairStyle::find_all(&ast);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].pair_style, "lj/cut");
        assert_eq!(issues[0].units, "metal");
    }

    #[test]
    fn valid_lj_cut_with_lj() {
        let source = "units lj\npair_style lj/cut 2.5\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = SuspiciousUnitsPairStyle::find_all(&ast);
        assert!(issues.is_empty());
    }

    #[test]
    fn no_units_or_pair_style() {
        let source = "dimension 3\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = SuspiciousUnitsPairStyle::find_all(&ast);
        assert!(issues.is_empty());
    }

    #[test]
    fn eam_with_metal_units_ok() {
        let source = "units metal\npair_style eam/alloy\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = SuspiciousUnitsPairStyle::find_all(&ast);
        assert!(issues.is_empty(), "eam/alloy with metal should be fine");
    }
}
