use super::{CommandSyntax, KeywordArg, Nargs, Style};
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug)]
pub(crate) struct Kwarg<'a> {
    name: &'static str,
    args: Vec<&'a str>,
}

#[derive(Debug)]
pub(crate) struct ParseResult<'a> {
    command_name: &'static str,
    positional: Vec<&'a str>,
    style: Option<&'a str>,
    style_args: Vec<&'a str>,
    kwargs: Vec<Kwarg<'a>>,
}

#[derive(Debug, Error)]
#[error("invalid {command_name} command: {kind}")]
pub(crate) struct ParseError {
    kind: ParseErrorKind,
    command_name: &'static str,
    // TODO: span
}

#[derive(Debug, Error)]
pub(crate) enum ParseErrorKind {
    #[error("expected {expected} positional arguments, found {found}")]
    InvalidPositionals { expected: u32, found: u32 },
    #[error("for style {style}, expected {expected} positional arguments, found {found}")]
    InvalidStyleArgs {
        style: &'static str,
        expected: u32,
        found: u32,
    },
    #[error("invalid keyword {found}, valid keywords: {valid}")]
    InvalidKeyword { found: String, valid: String },

    #[error("invalid style {found}, valid styles:{valid} ")]
    InvalidStyle { found: String, valid: String },

    #[error("for keyword {kwarg}, expected{expected}, found {found}, ")]
    KeywordArguments {
        found: u32,
        expected: u32,
        kwarg: &'static str,
    },
}

impl CommandSyntax {
    /// NOTE: As an initial Proof of Concept, just working on strings...
    pub(crate) fn parse<'a>(&self, words: Vec<&'a str>) -> Result<ParseResult<'a>, ParseError> {
        // TODO: have alternate between modes reading Nargs args and reading keywords?

        /// Some sort of statemachine the parsing is currently in
        enum Mode {
            /// Next word is expected to be a keyword
            FindingKeywords,
            // TODO: take the clap approach and use ranges?
            ReadNargs(u32), // Read a known number of args
            ReadTrailing,   // Read args until a keyword is found
        }

        assert!(
            words[0] == self.command_name,
            "Wrong command name: got {}, expected {}",
            words[0],
            self.command_name
        );
        let mut current = 1;

        // Skip ahead and find the style
        // TODO: Could do instead when reading the rest of the args.
        let style = if self.styles.is_some() {
            Some(self.determine_style(&words)?)
        } else {
            None
        };

        // Dumb O(N) because there should only be a handful of keywords
        let is_keyword_arg = move |word: &str| {
            self.kwargs
                .iter()
                .find(|KeywordArg { name, .. }| *name == word)
        };

        let mut positionals = Vec::with_capacity(self.n_positional as usize);

        for i_pos in 0..self.n_positional {
            if let Some(&word) = words.get(current) {
                positionals.push(word);
            } else {
                return Err(self.positional_err(current as u32));
            }
            current += 1;
        }

        let n_style_args = style.as_ref().map_or(Nargs::None, |sty| sty.arg_count);

        let should_break_on_kw = !n_style_args.is_fixed();

        let mut style_args = Vec::with_capacity(n_style_args.min_args() as usize);

        // FIXME: Break if current is greater than the number of total arguments

        for i_pos in n_style_args.into_iter() {
            let Some(&word) = words.get(current) else {
                if n_style_args.is_fixed() {
                    return Err(self.style_positional_err(
                        style.expect("Unreachable unless a style is defined"),
                        i_pos,
                    ));
                } else {
                    break;
                }
            };

            if should_break_on_kw && is_keyword_arg(word).is_some() {
                break;
            }

            style_args.push(word);

            current += 1;
        }

        // remaining args

        let mut args_iter = words.into_iter().skip(current);
        let mut kwargs = Vec::new();

        while let Some(word) = args_iter.next() {
            if let Some(kwarg) = is_keyword_arg(word) {
                // TODO: advance by the appropriate number of args...
                // Might not work unless the number of keywords is known
                let args = self.read_arguments_for_keyword(kwarg, &mut args_iter)?;

                kwargs.push(Kwarg {
                    name: kwarg.name,
                    args,
                });

                // Advance by the number of args
            } else {
                return Err(self.invalid_keyword_err(word));
            }
        }

        Ok(ParseResult {
            command_name: self.command_name,
            positional: positionals,
            style: style.map(|s| s.name),
            style_args,
            kwargs,
        })
    }

    fn n_styles(&self) -> u32 {
        (self.styles.as_ref().map_or(0, |s| s.styles.len())) as u32
    }

    fn determine_style(&self, words: &[&str]) -> Result<Style, ParseError> {
        let styles = self
            .styles
            .as_ref()
            .expect("should only be called for commands with styles");
        let style_pos = styles.style_position;

        let found_style = words[style_pos as usize];

        let Some(style) = styles.styles.iter().find(|sty| sty.name == found_style) else {
            return Err(self.invalid_style_err(found_style));
        };

        Ok(style.clone())
    }

    fn read_arguments<'a>(
        &self,
        nargs: Nargs,
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<&'a str>, u32> {
        let nargs = match nargs {
            Nargs::Int(n) => n,
            Nargs::None => 0,
            x => unimplemented!("{:?}", x),
        };

        let found_args = iter.take(nargs as usize).collect_vec();

        if (found_args.len() as u32) < nargs {
            return Err(found_args.len() as u32);
        }

        Ok(found_args)
    }

    fn read_arguments_for_keyword<'a>(
        &self,
        keyword: &KeywordArg,
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<&'a str>, ParseError> {
        self.read_arguments(keyword.nargs, iter).map_err(|n_found| {
            ParseErrorKind::KeywordArguments {
                found: n_found,
                // FIXME: a bit of a hack around fact that the nargs is not avalible in this function
                expected: keyword.nargs.min_args(),
                kwarg: keyword.name,
            }
            .with_command(self.command_name)
        })
    }
}

impl CommandSyntax {
    fn positional_err(&self, found: u32) -> ParseError {
        ParseErrorKind::InvalidPositionals {
            expected: self.n_positional,
            found,
        }
        .with_command(self.command_name)
    }

    fn invalid_keyword_err(&self, found: &str) -> ParseError {
        ParseErrorKind::InvalidKeyword {
            found: found.to_owned(),
            valid: self.kwargs.iter().map(|k| k.name).join(", "),
        }
        .with_command(self.command_name)
    }

    fn invalid_style_err(&self, found: &str) -> ParseError {
        ParseErrorKind::InvalidStyle {
            found: found.to_owned(),
            valid: self
                .styles
                .as_ref()
                .expect("Must have styles")
                .styles
                .iter()
                .map(|k| k.name)
                .join(", "),
        }
        .with_command(self.command_name)
    }

    fn style_positional_err(&self, style: Style, found: u32) -> ParseError {
        ParseErrorKind::InvalidStyleArgs {
            style: style.name,
            expected: style.arg_count.min_args(),
            found,
        }
        .with_command(self.command_name)
    }
}

impl ParseErrorKind {
    /// Create a full error for this error kind.
    fn with_command(self, command_name: &'static str) -> ParseError {
        ParseError {
            kind: self,
            command_name,
        }
    }
}
