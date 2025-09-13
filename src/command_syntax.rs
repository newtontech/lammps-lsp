use std::panic;

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

        let is_keyword_arg = move |word: &str| {
            self.kwargs
                .iter()
                .find(|KeywordArg { name, nargs }| *name == word)
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

        // remaining args

        let mut args_iter = words[current..].iter();

        while let Some(word) = args_iter.next() {
            if let Some(kwarg) = is_keyword_arg(word) {
                todo!("Handle the kwarg and its positionals")
                // Advance by the number of args
            } else {
                panic!("Invalid keyword/trailing argument")
            }
        }
    }
}

/// This macro is an abosolutely atrocious and un-necessary use and abuse of them
macro_rules! kwarg {
    ($name:literal,$n:literal) => {
        KeywordArg {
            name: $name,
            nargs: Nargs::Int($n),
        }
    };
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    // -   type = atom type (1-Ntypes) of atoms to create (offset for molecule
    //     creation)
    //
    // -   style = *box* or *region* or *single* or *mesh* or *random*
    //
    //         *box* args = none
    //         *region* args = region-ID
    //           region-ID = particles will only be created if contained in the region
    //         *single* args = x y z
    //           x,y,z = coordinates of a single particle (distance units)
    //         *mesh* args = STL-file
    //           STL-file = file with triangle mesh in STL format
    //         *random* args = N seed region-ID
    //           N = number of particles to create
    //           seed = random # seed (positive integer)
    //           region-ID = create atoms within this region, use NULL for entire simulation box
    //
    // -   zero or more keyword/value pairs may be appended
    //
    // -   keyword = *mol* or *basis* or *ratio* or *subset* or *remap* or
    //     *var* or *set* or *radscale* or *meshmode* or *rotate* or *overlap*
    //     or *maxtry* or *units*
    //
    //         *mol* values = template-ID seed
    //           template-ID = ID of molecule template specified in a separate `molecule <molecule>`__ command
    //           seed = random # seed (positive integer)
    //         *basis* values = M itype
    //           M = which basis atom
    //           itype = atom type (1-N) to assign to this basis atom
    //         *ratio* values = frac seed
    //           frac = fraction of lattice sites (0 to 1) to populate randomly
    //           seed = random # seed (positive integer)
    //         *subset* values = Nsubset seed
    //           Nsubset = # of lattice sites to populate randomly
    //           seed = random # seed (positive integer)
    //         *remap* value = *yes* or *no*
    //         *var* value = name = variable name to evaluate for test of atom creation
    //         *set* values = dim name
    //           dim = *x* or *y* or *z*
    //           name = name of variable to set with x, y, or z atom position
    //         *radscale* value = factor
    //           factor = scale factor for setting atom radius
    //         *meshmode* values = mode arg
    //           mode = *bisect* or *qrand*
    //           *bisect* arg = radthresh
    //             radthresh = threshold value for *mesh* to determine when to split triangles (distance units)
    //           *qrand* arg = density
    //             density = minimum number density for atoms place on *mesh* triangles (inverse distance squared units)
    //         *rotate* values = theta Rx Ry Rz
    //           theta = rotation angle for single molecule (degrees)
    //           Rx,Ry,Rz = rotation vector for single molecule
    //         *overlap* value = Doverlap
    //           Doverlap = only insert if at least this distance from all existing atoms
    //         *maxtry* value = Ntry
    //           Ntry = number of attempts to insert a particle before failure
    //         *units* value = *lattice* or *box*
    //           *lattice* = the geometry is defined in lattice units
    //           *box* = the geometry is defined in simulation box units
    const CREATE_ATOMS: CommandSyntax<5, 13> = CommandSyntax {
        command_name: "create_atoms",
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

        kwargs: [
            kwarg!("mol", 2),
            kwarg!("basis", 2),
            kwarg!("ratio", 2),
            kwarg!("subset", 2),
            kwarg!("remap", 1),
            kwarg!("var", 1),
            kwarg!("set", 2),
            kwarg!("radscale", 1),
            kwarg!("rotate", 4),
            kwarg!("overlap", 1),
            kwarg!("maxtry", 1),
            kwarg!("units", 1),
            kwarg!("meshmode", 2), // WARN: Kwargs like this could cause problems, they could have a mode and
                                   // variable args, like a style
        ],
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

    #[test]
    fn create_atoms_lammps_examples() {
        let examples = "create_atoms 1 box
create_atoms 3 region regsphere basis 2 3
create_atoms 3 region regsphere basis 2 3 ratio 0.5 74637
create_atoms 3 single 0 0 5
create_atoms 1 box var v set x xpos set y ypos
create_atoms 2 random 50 12345 NULL overlap 2.0 maxtry 50
create_atoms 1 mesh open_box.stl meshmode qrand 0.1 units box
create_atoms 1 mesh funnel.stl meshmode bisect 4.0 units box radscale 0.9"
            .lines()
            .map(|l| l.split_whitespace().collect_vec());
        for example in examples {
            CREATE_ATOMS.parse(example);
        }
    }
}
