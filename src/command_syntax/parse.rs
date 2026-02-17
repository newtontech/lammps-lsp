use super::{CommandSyntax, KeywordArg, Nargs, Style};
use itertools::Itertools;

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

impl CommandSyntax {
    /// NOTE: As an initial Proof of Concept, just working on strings...
    // FIXME: Doesn't really parse just validates, and just panics if invalid
    pub(crate) fn parse<'a>(&self, words: Vec<&'a str>) -> ParseResult<'a> {
        // TODO: have alternate between modes reading Nargs args and reading keywords?

        println!("----------");

        /// Some sort of statemachine the parsing is currently in
        enum Mode {
            /// Next word is expected to be a keyword
            FindingKeywords,
            // TODO: take the clap approach and use ranges?
            ReadNargs(u32), // Read a known number of args
            ReadTrailing,   // Read args until a keyword is found
        }

        assert!(words[0] == self.command_name, "Wrong command name");
        println!("{}", self.command_name);
        let mut current = 1;

        // Skip ahead and find the style
        // TODO: Could do instead when reading the rest of the args.
        let style = self.determine_style(&words);

        let is_keyword_arg = move |word: &str| {
            self.kwargs
                .iter()
                .find(|KeywordArg { name, .. }| *name == word)
        };

        let mut words_iter = words.iter();

        let mut positionals = Vec::with_capacity(self.n_positional as usize);

        for i_pos in 0..self.n_positional {
            if let Some(&word) = words.get(current) {
                println!("{word}");
                positionals.push(word);
            } else {
                panic!(
                    "invalid self, expected {} arguments for {}, only found {}",
                    self.n_positional, self.command_name, current
                );
            }
            current += 1;
        }

        let n_style_args = style.as_ref().map_or(0, |sty| sty.arg_count);

        println!("style: {:?}", style.as_ref().map_or("N/A", |sty| sty.name));
        println!("n_style_args: {:?}", n_style_args);

        let mut style_args = Vec::with_capacity(n_style_args as usize);

        for i_pos in 0..n_style_args {
            let Some(&word) = words.get(current) else {
                panic!(
                    "invalid syntax, expected {} arguments for style `{}`, only found {}",
                    n_style_args,
                    style.as_ref().map_or("", |sty| sty.name),
                    i_pos
                );
            };
            println!("{}", word);
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
                println!("{}:", kwarg.name);
                let args = self.read_arguments(kwarg.nargs, &mut args_iter);

                kwargs.push(Kwarg {
                    name: kwarg.name,
                    args,
                });

                // Advance by the number of args
            } else {
                panic!("Invalid keyword `{word}` or unexpected trailing positional argument")
            }
        }

        println!("----------");

        ParseResult {
            command_name: self.command_name,
            positional: positionals,
            style: style.map(|s| s.name),
            style_args,
            kwargs,
        }
    }

    fn n_styles(&self) -> u32 {
        (self.styles.as_ref().map_or(0, |s| s.styles.len())) as u32
    }

    fn determine_style(&self, words: &[&str]) -> Option<Style> {
        let styles = self.styles.as_ref()?;
        let style_pos = styles.style_position;

        let found_style = words[style_pos as usize];

        let Some(style) = styles.styles.iter().find(|sty| sty.name == found_style) else {
            panic!("invalid style {} for {}", found_style, self.command_name);
        };

        Some(style.clone())
    }

    fn read_arguments<'a>(
        &self,
        nargs: Nargs,
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Vec<&'a str> {
        /// TODO: Return something more useful than just panicking on error!!!
        let nargs = match nargs {
            Nargs::Int(n) => n,
            Nargs::None => 0,
            x => unimplemented!("{:?}", x),
        };

        let found_args = iter
            .take(nargs as usize)
            .map(|w| {
                println!("{w}");
                w
            })
            .collect_vec();

        if (found_args.len() as u32) < nargs {
            panic!("expected {} found {} args", nargs, found_args.len());
        }

        found_args
    }
}
