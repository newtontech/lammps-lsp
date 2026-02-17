use itertools::Itertools;

use crate::{
    ast::Argument,
    styles::{self, FixStyle},
};

pub mod parse;

pub mod syntaxes;

/// A representation of the command/style's syntax
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CommandSyntax {
    command_name: &'static str,
    n_positional: u32, // TODO: Switch to Nargs later
    positional_labels: Vec<PositionalArg>,
    /// Mutually exclusive keywords
    /// The style's arguments follow the rest of the positional arguments
    /// Which keyword gives the style is kept in the styles struct
    styles: Option<Styles>,
    // TODO: make this an array and generate at compile time
    kwargs: Vec<KeywordArg>,
    has_trailing_positionals: bool,
}

pub(crate) struct CommandSyntaxBuilder {
    command_name: &'static str,
    positional_args: Vec<PositionalArg>,
    styles: Vec<Style>,
    style_pos: Option<u32>,
    kwargs: Vec<KeywordArg>,
    /// Could this command have more positonal arguments?
    has_trailing_positionals: bool,
}

impl CommandSyntax {
    pub(crate) fn new(command_name: &'static str) -> Self {
        Self {
            command_name,
            positional_labels: Vec::default(),
            styles: None,
            kwargs: Vec::new(),
            has_trailing_positionals: false,
            n_positional: 0,
        }
    }

    pub(crate) fn add_positional(&mut self, argname: &'static str) -> &mut Self {
        self.positional_labels.push(PositionalArg { name: argname });
        self.n_positional += 1;
        self
    }

    pub(crate) fn add_keyword(&mut self, kwarg: KeywordArg) {
        self.kwargs.push(kwarg)
    }

    pub(crate) fn add_kwargs(&mut self, kwargs: impl IntoIterator<Item = KeywordArg>) {
        for kwarg in kwargs {
            self.kwargs.push(kwarg);
        }
    }

    /// Add a style to the command.
    ///
    /// If a `<style>` positional has not yet been added, this will be appended to the list of
    /// positonal args.
    pub(crate) fn add_style(&mut self, style: Style) {
        let styles = match &mut self.styles {
            Some(styles) => styles,

            None => {
                self.add_positional("style");
                self.styles = Some(Styles {
                    style_position: self.n_positional,
                    styles: Vec::new(),
                });
                self.styles.as_mut().unwrap()
            }
        };

        styles.styles.push(style);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct KeywordArg {
    name: &'static str,
    nargs: Nargs,
    labels: Option<Vec<&'static str>>,
    // TODO: add types?
}

impl KeywordArg {
    pub(crate) fn new(name: &'static str, nargs: Nargs, labels: Option<Vec<&'static str>>) -> Self {
        if let Some(labels) = &labels {
            match nargs {
                Nargs::Int(n) => assert_eq!(
                    labels.len() as u32,
                    n,
                    "missmatched number of positional args and labels"
                ),
                Nargs::Optional => todo!(),
                Nargs::ZeroPlus => todo!(),
                Nargs::OnePlus => todo!(),
                Nargs::None => assert_eq!(labels.len(), 0),
            }
        }

        Self {
            name,
            nargs,
            labels,
        }
    }
}

// TODO: See how sub commands work? Maybe this will be similar
#[derive(Debug, PartialEq, Eq)]
struct Styles {
    /// Location of the 'style' argument among the positional args
    style_position: u32,
    // TODO: Revert back to an array
    styles: Vec<Style>,
}

impl Styles {
    fn is_valid_style(&self, style: &str) -> bool {
        // NOTE: Assuming only a few styles
        self.styles.iter().map(|s| s.name).contains(&style)
    }

    fn get_style(&self, style: &str) -> Option<&Style> {
        self.styles.iter().find(|s| s.name == style)
    }
}

impl Default for Styles {
    fn default() -> Self {
        Styles {
            style_position: 0,
            styles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Style {
    name: &'static str,
    arg_count: u32,
    arg_names: Option<Vec<&'static str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PositionalArg {
    name: &'static str,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Nargs {
    /// Exactly this many arguments.
    Int(u32),
    /// Zero or 1 Arguments (? in regex)
    Optional,
    /// 1 or more arguments (* in regex)
    ZeroPlus,
    /// 1 or more arguments (+ in regex)
    OnePlus,

    /// First argument is a style, with further positional arguments dependant on the style
    // Styles(StyleNargs<N_STYLES>),

    #[default]
    None,
}

/// This macro is an abosolutely atrocious and un-necessary use and abuse of them
// TODO: is there a way to just add the hints and figure the counts out from them?
macro_rules! kwarg {
    ($name:literal,$n:literal) => {
        crate::command_syntax::KeywordArg {
            name: $name,
            nargs: crate::command_syntax::Nargs::Int($n),
            labels:None
        }
    };
    ($name:literal,$n:literal;$($label:literal),+) => {
        crate::command_syntax::KeywordArg {
            name: $name,
            nargs: crate::command_syntax::Nargs::Int($n),
            labels: Some(vec![$($label),+]),
        }
    };
}

pub(crate) use kwarg;
