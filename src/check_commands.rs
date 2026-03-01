//! Checks the validity of fix arguments

use thiserror::Error;

use crate::{
    ast::{Ast, Command},
    command_syntax,
    commands::CommandName,
    diagnostics::{self, Diagnostic, Issue},
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
    InvalidArguments(#[from] InvalidArguments),

    // TODO: Consider merging with the above
    #[error(transparent)]
    SyntaxError(#[from] command_syntax::parse::ParseError),
}

impl Issue for InvalidCommand {
    fn diagnostic(&self) -> diagnostics::Diagnostic {
        let span = match self {
            Self::UnknownCommand(_, span) => *span,
            Self::InvalidArguments(invalid_args) => invalid_args.range,
            Self::SyntaxError(err) => err.span,
        };
        Diagnostic {
            name: "invalid command",
            message: self.to_string(),
            span,
            severity: diagnostics::Severity::Error,
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
                    // ignore the actual result, just care about the error.
                    let syntax = command_syntax::syntaxes::syntax(command.name.as_str())?;
                    syntax.parse(command).err().map(InvalidCommand::from)
                }
            }
            _ => None,
        })
    }
}
