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
    pub(crate) fn add_style(&mut self, style: Style) -> &mut Self {
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

        self
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
                Nargs::Optional => todo!("One label for all variables"),
                Nargs::ZeroPlus => todo!("One Label for all variables"),
                Nargs::OnePlus => todo!("One Label for all variables"),
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
    arg_count: Nargs,
    arg_names: Option<Vec<&'static str>>,
}

impl Style {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            arg_count: Nargs::None,
            arg_names: None,
        }
    }

    /// Add a new argument with a given label:
    ///
    /// Adds to the arg count if Int or None
    fn add_arg(mut self, label: &'static str) -> Self {
        let argnames = match &mut self.arg_names {
            Some(argnames) => argnames,
            None => {
                self.arg_names = Some(Vec::new());
                &mut self.arg_names.as_mut().unwrap()
            }
        };
        argnames.push(label);
        match self.arg_count {
            Nargs::Int(i) => {
                self.arg_count = Nargs::Int(i + 1);
            }
            Nargs::None => self.arg_count = Nargs::Int(1),
            _ => (),
        }
        self
    }

    /// Add an argument that can support more than one values.
    fn add_var_arg(mut self, label: &'static str) -> Self {
        // FIXME: add support for styles with both number of positional args and some trailing.
        self.arg_names.get_or_insert_default().push(label);
        // FIXME: Only do so if there is no positional already set?
        self.arg_count = Nargs::OnePlus;
        self
    }
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

impl From<u32> for Nargs {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl Nargs {
    pub(crate) fn is_fixed(&self) -> bool {
        match self {
            Nargs::Int(_) | Nargs::None => true,
            Nargs::Optional | Nargs::ZeroPlus | Nargs::OnePlus => false,
        }
    }
    pub(crate) const fn max_args(&self) -> Option<u32> {
        match self {
            Nargs::Int(i) => Some(*i),
            Nargs::None => Some(0),
            Nargs::Optional => Some(1),
            Nargs::ZeroPlus => None,
            Nargs::OnePlus => None,
        }
    }

    pub(crate) const fn min_args(&self) -> u32 {
        match self {
            Nargs::Int(i) => *i,
            Nargs::None => 0,
            Nargs::Optional => 0,
            Nargs::ZeroPlus => 0,
            Nargs::OnePlus => 1,
        }
    }
}

pub struct NargsIter {
    nargs: Nargs,
    current: u32,
}

impl Iterator for NargsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nargs == Nargs::None {
            return None;
        }

        let old = self.current;
        self.current += 1;

        let Some(n_max) = self.nargs.max_args() else {
            return Some(old); // These are infinite iterators
        };

        if self.current > n_max {
            return None;
        }

        Some(old)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.nargs.min_args() as usize,
            self.nargs.max_args().map(|n| n as usize),
        )
    }
}

impl IntoIterator for Nargs {
    type Item = u32;

    type IntoIter = NargsIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current: 0,
            nargs: self,
        }
    }
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

#[cfg(test)]
mod test {
    use super::Nargs;

    #[test]
    fn nargs_fixed_iter() {
        let mut iter = Nargs::Int(3).into_iter();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn nargs_optional_iter() {
        let mut iter = Nargs::Optional.into_iter();

        // Has at most 1 argument
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn nargs_zero_plus_iter() {
        let mut iter = Nargs::ZeroPlus.into_iter();

        // Has infinite number of args

        for i in 0..1000 {
            assert_eq!(iter.next(), Some(i));
        }
        // And so on
    }

    #[test]
    fn nargs_one_plus_iter() {
        let mut iter = Nargs::ZeroPlus.into_iter();

        // Has infinite number of args

        for i in 0..1000 {
            assert_eq!(iter.next(), Some(i));
        }
        // And so on
    }
}
