//! Checks the validity of fix arguments

use thiserror::Error;

use crate::{
    ast::{Ast, Command},
    commands::CommandName,
    diagnostics::{self, Diagnostic, Issue},
    lints::codes::LintCode,
    spans::Span,
};

use self::{computes::check_compute, fixes::check_fix, invalid_arguments::InvalidArguments};

pub mod invalid_arguments;

pub mod computes;
pub mod fixes;

pub mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum InvalidCommand {
    #[error("unknown command: `{0}`")]
    UnknownCommand(String, Span),
    #[error("{0}")]
    InvalidArguments(InvalidArguments),
}

impl From<InvalidArguments> for InvalidCommand {
    fn from(v: InvalidArguments) -> Self {
        Self::InvalidArguments(v)
    }
}

impl Issue for InvalidCommand {
    fn diagnostic(&self) -> diagnostics::Diagnostic {
        match self {
            Self::UnknownCommand(name, span) => Diagnostic {
                name: "invalid command",
                message: format!("{}: unknown command `{}`", LintCode::UnknownCommand, name),
                code: Some(LintCode::UnknownCommand.to_string()),
                span: *span,
                severity: diagnostics::Severity::Error,
            },
            Self::InvalidArguments(invalid_args) => Diagnostic {
                name: "invalid command",
                message: self.to_string(),
                code: Some(LintCode::InvalidArguments.to_string()),
                span: invalid_args.range,
                severity: diagnostics::Severity::Error,
            },
        }
    }
}

impl Ast {
    pub fn check_commands(&self) -> impl Iterator<Item = InvalidCommand> + '_ {
        self.commands.iter().filter_map(|command| match &command {
            Command::Fix(fix_def) => {
                // Check if the fixes are ok.
                check_fix(fix_def).err().map(InvalidCommand::from)
            }
            // Not sure about other named commands yet...
            Command::Compute(compute) => check_compute(compute).err().map(InvalidCommand::from),
            Command::Generic(command) => {
                let command_name = CommandName::from(command.name.as_str());

                if let CommandName::InvalidCommand(name) = command_name {
                    Some(InvalidCommand::UnknownCommand(
                        name,
                        Span {
                            start: command.start,
                            end: command.end,
                        },
                    ))
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}
