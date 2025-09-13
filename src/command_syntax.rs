use crate::{ast::Argument, styles::FixStyle};

/// A representation of the command/style's syntax
struct CommandSyntax<const N_STYLES: usize, const N_KWARG: usize> {
    command_name: &'static str,
    n_positional: u32, // TODO: Switch to Nargs later
    /// Mutually exclusive keywords
    /// The style's arguments follow the rest of the positional arguments
    /// Which keyword gives the style is kept in the styles struct
    styles: Styles<N_STYLES>,
    // TODO: make this an array and generate at compile time
    kwargs: [KeywordArg; N_KWARG],
}

// TODO: Finish this.
// Trim the first three words away: the fix keyword, the fix id, and the style name.
// And then use the command again.
struct FixSyntax {
    fix_style: FixStyle,
}

struct KeywordArg {
    name: &'static str,
    nargs: Nargs,
    // TODO: add types?
}

struct Styles<const N_STYLES: usize> {
    style_position: u32,
    styles: [(&'static str, u32); N_STYLES],
}
impl Default for Styles<0> {
    fn default() -> Self {
        Styles {
            style_position: 0,
            styles: [],
        }
    }
}

// struct PositionalArg {
//
// }

#[derive(Default)]
enum Nargs {
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

impl<const N_STYLES: usize, const N_KWARG: usize> CommandSyntax<N_STYLES, N_KWARG> {
    /// NOTE: As an initial Proof of Concept, just working on strings...
    fn parse(&self, words: Vec<&str>) {
        let style_pos = if N_STYLES != 0 {
            self.styles.style_position
        } else {
            self.n_positional + 10 // Cannot have a style arument if there are more than
                                   // expected!!!
        };

        assert!(words[0] == self.command_name, "Wrong command name");
        let mut current = 1;

        let is_in = |word: &str, styles: &[(&'static str, u32)]| {
            styles.iter().map(|(k, _)| k).any(|&style| style == word)
        };

        let mut style_args = 0;
        let mut style = "";

        for i_pos in 0..self.n_positional {
            if let Some(word) = words.get(current) {
                if current == style_pos as usize {
                    let Some((sty, n_style_args)) =
                        self.styles.styles.iter().find(|(k, v)| k == word)
                    else {
                        panic!("invalid style {} for {}", word, self.command_name);
                    };

                    style_args = *n_style_args;
                    style = sty;
                }
            } else {
                panic!(
                    "invalid self, expected {} arguments for {}, only found {}",
                    self.n_positional, self.command_name, current
                );
            }
            current += 1;
        }

        println!("style: {style}");
        println!("n_style_args: {style_args}");

        for i_pos in 0..style_args {
            let Some(word) = words.get(current) else {
                panic!(
                    "invalid syntax, expected {} arguments for style {}, only found {}",
                    style_args, style, i_pos
                );
            };

            // TODO: Here, do I abort if I find a keyword, or do I carry on...
            // See what is commonly done in lammps code
            // is_keyword_arg(word)

            current += 1;
        }

        todo!("Handle Keyword Args")
    }
}

#[cfg(test)]
mod test {

    use itertools::Itertools;
    const CREATE_ATOMS: CommandSyntax<5, 0> = CommandSyntax {
        command_name: "create_atoms",
        // FIXME: Doesn't quite work... Has a normal positional BEFORE the style
        // TODO: treat styles as mutually exclusive keywords instead???
        // Could have this be a list of types.
        // if the last positional is a 'style' then we treat it like a keyword?
        // ```
        // struct
        // positional: [Any,Style],
        // // Mutually exclusive keywords, thats name must be the last positional arg.
        // styles : [
        //          ("box", 0),
        //         ("region", 1),
        //         ("single", 3),
        //         ("mesh", 1),
        //         ("random", 3),
        // ]
        // ```
        n_positional: 2,
        styles: Styles {
            style_position: 2,
            styles: [
                ("box", 0),
                ("region", 1),
                ("single", 3),
                ("mesh", 1),
                ("random", 3),
            ],
        },

        kwargs: [], // After the style almost want this to be recursive
    };

    use super::*;

    #[test]
    fn create_atoms_box() {
        let example1 = "create_atoms 1 box".split_whitespace().collect_vec();
        CREATE_ATOMS.parse(example1);
    }

    #[test]
    #[should_panic]
    fn create_atoms_box_bad() {
        let example1 = "create_atoms 1 box extra_arg"
            .split_whitespace()
            .collect_vec();
        CREATE_ATOMS.parse(example1);
    }

    #[test]
    fn create_atoms_region() {
        let example1 = "create_atoms 2 region mybox"
            .split_whitespace()
            .collect_vec();
        CREATE_ATOMS.parse(example1);
    }

    #[test]
    #[should_panic]
    fn create_atoms_region_bad() {
        let example1 = "create_atoms 2 region".split_whitespace().collect_vec();
        CREATE_ATOMS.parse(example1);
    }
}
