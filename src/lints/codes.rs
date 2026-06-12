//! Diagnostic codes for LAMMPS lint rules.
//!
//! Codes follow the pattern `LAMMPS-E###` (errors) and `LAMMPS-W###` (warnings).
//!
//! # Code Ranges
//!
//! | Range | Category |
//! |-------|----------|
//! | E100-E199, W100-W199 | Syntax and parsing |
//! | E200-E299, W200-W299 | Command validation |
//! | E300-E399, W300-W399 | Style validation |
//! | E400-E499, W400-W499 | Simulation box and data |
//! | E500-E599, W500-W599 | Variable and expansion |
//! | E600-E699, W600-W699 | Identifier and reference |

use std::fmt;

/// A lint diagnostic code with a category prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintCode {
    // Syntax / Parsing (E100-E199, W100-W199)
    /// E100: Ampersand continuation in comments is invalid.
    AmpersandInComment,
    /// E101: Unterminated string.
    UnterminatedString,

    // Command validation (E200-E299, W200-W299)
    /// W200: Unknown command.
    UnknownCommand,
    /// W201: Redefined identifier before run.
    RedefinedBeforeRun,
    /// W202: Unused identifier.
    UnusedIdentifier,
    /// E203: Undefined identifier.
    UndefinedIdentifier,
    /// W204: Invalid arguments.
    InvalidArguments,
    /// W205: Fix argument validation.
    InvalidFixArgs,
    /// W206: Compute argument validation.
    InvalidComputeArgs,

    // Style validation (E300-E399, W300-W399)
    /// E300: Invalid fix style.
    InvalidFixStyle,
    /// E301: Invalid compute style.
    InvalidComputeStyle,
    /// E302: Invalid pair style.
    InvalidPairStyle,
    /// W303: Style defined after data read.
    StyleAfterDataRead,

    // Simulation box / data (E400-E499, W400-W499)
    /// E400: Simulation box not defined.
    SimBoxNotDefined,
    /// W401: Missing create_box.
    MissingCreateBox,
    /// W402: Missing region definition.
    MissingRegion,

    // Variable / expansion (E500-E599, W500-W599)
    /// E500: Circular variable dependency.
    CircularVariableDependency,
    /// W501: String variable expansion.
    StringVarExpansion,
    /// E502: Variable reference in expression invalid.
    InvalidVarReference,

    // Identifier / reference (E600-E699, W600-W699)
    /// W600: Rerun detection in docs file.
    RerunInDocs,
}

impl LintCode {
    /// Returns the string code (e.g., "LAMMPS-E100").
    pub fn code(&self) -> &'static str {
        match self {
            Self::AmpersandInComment => "LAMMPS-E100",
            Self::UnterminatedString => "LAMMPS-E101",

            Self::UnknownCommand => "LAMMPS-W200",
            Self::RedefinedBeforeRun => "LAMMPS-W201",
            Self::UnusedIdentifier => "LAMMPS-W202",
            Self::UndefinedIdentifier => "LAMMPS-E203",
            Self::InvalidArguments => "LAMMPS-W204",
            Self::InvalidFixArgs => "LAMMPS-W205",
            Self::InvalidComputeArgs => "LAMMPS-W206",

            Self::InvalidFixStyle => "LAMMPS-E300",
            Self::InvalidComputeStyle => "LAMMPS-E301",
            Self::InvalidPairStyle => "LAMMPS-E302",
            Self::StyleAfterDataRead => "LAMMPS-W303",

            Self::SimBoxNotDefined => "LAMMPS-E400",
            Self::MissingCreateBox => "LAMMPS-W401",
            Self::MissingRegion => "LAMMPS-W402",

            Self::CircularVariableDependency => "LAMMPS-E500",
            Self::StringVarExpansion => "LAMMPS-W501",
            Self::InvalidVarReference => "LAMMPS-E502",

            Self::RerunInDocs => "LAMMPS-W600",
        }
    }

    /// Returns a short human-readable label for the code.
    pub fn label(&self) -> &'static str {
        match self {
            Self::AmpersandInComment => "ampersand-in-comment",
            Self::UnterminatedString => "unterminated-string",

            Self::UnknownCommand => "unknown-command",
            Self::RedefinedBeforeRun => "redefined-before-run",
            Self::UnusedIdentifier => "unused-identifier",
            Self::UndefinedIdentifier => "undefined-identifier",
            Self::InvalidArguments => "invalid-arguments",
            Self::InvalidFixArgs => "invalid-fix-args",
            Self::InvalidComputeArgs => "invalid-compute-args",

            Self::InvalidFixStyle => "invalid-fix-style",
            Self::InvalidComputeStyle => "invalid-compute-style",
            Self::InvalidPairStyle => "invalid-pair-style",
            Self::StyleAfterDataRead => "style-after-data-read",

            Self::SimBoxNotDefined => "sim-box-not-defined",
            Self::MissingCreateBox => "missing-create-box",
            Self::MissingRegion => "missing-region",

            Self::CircularVariableDependency => "circular-variable-dep",
            Self::StringVarExpansion => "string-var-expansion",
            Self::InvalidVarReference => "invalid-var-reference",

            Self::RerunInDocs => "rerun-in-docs",
        }
    }
}

impl fmt::Display for LintCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_codes_are_unique() {
        let all: Vec<LintCode> = vec![
            LintCode::AmpersandInComment,
            LintCode::UnterminatedString,
            LintCode::UnknownCommand,
            LintCode::RedefinedBeforeRun,
            LintCode::UnusedIdentifier,
            LintCode::UndefinedIdentifier,
            LintCode::InvalidArguments,
            LintCode::InvalidFixArgs,
            LintCode::InvalidComputeArgs,
            LintCode::InvalidFixStyle,
            LintCode::InvalidComputeStyle,
            LintCode::InvalidPairStyle,
            LintCode::StyleAfterDataRead,
            LintCode::SimBoxNotDefined,
            LintCode::MissingCreateBox,
            LintCode::MissingRegion,
            LintCode::CircularVariableDependency,
            LintCode::StringVarExpansion,
            LintCode::InvalidVarReference,
            LintCode::RerunInDocs,
        ];

        let codes: Vec<&str> = all.iter().map(|c| c.code()).collect();
        let mut sorted = codes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), codes.len(), "duplicate lint codes detected");
    }

    #[test]
    fn code_format() {
        assert!(LintCode::AmpersandInComment.code().starts_with("LAMMPS-"));
        assert!(LintCode::SimBoxNotDefined.code().starts_with("LAMMPS-"));
    }
}
