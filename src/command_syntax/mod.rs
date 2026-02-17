use itertools::Itertools;

use crate::{
    ast::Argument,
    styles::{self, FixStyle},
};

pub mod parse;

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

    pub(crate) fn add_positional(&mut self, argname: &'static str) {
        self.positional_labels.push(PositionalArg { name: argname });
        self.n_positional += 1;
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
        KeywordArg {
            name: $name,
            nargs: Nargs::Int($n),
            labels:None
        }
    };
    ($name:literal,$n:literal;$($label:literal),+) => {
        KeywordArg {
            name: $name,
            nargs: Nargs::Int($n),
            labels: Some(vec![$($label),+]),
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

    fn create_atoms_syntax_manual() -> CommandSyntax {
        CommandSyntax {
            command_name: "create_atoms",
            n_positional: 2,
            styles: Some(Styles {
                style_position: 2,
                styles: vec![
                    Style {
                        name: "box",
                        arg_count: 0,
                        arg_names: None,
                    },
                    Style {
                        name: "region",
                        arg_count: 1,
                        arg_names: Some(vec!["region-ID"]),
                    },
                    Style {
                        name: "single",
                        arg_count: 3,
                        arg_names: Some(vec!["x", "y", "z"]),
                    },
                    Style {
                        name: "mesh",
                        arg_count: 1,
                        arg_names: Some(vec!["STL-file"]),
                    },
                    Style {
                        name: "random",
                        arg_count: 3,
                        arg_names: Some(vec!["N", "seed", "region-ID"]),
                    },
                ],
            }),

            kwargs: vec![
                kwarg!("mol", 2;"template-ID","seed"),
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
                                       // this one is ok, because it takes only a fixed number.
            ],
            positional_labels: vec![
                PositionalArg { name: "type" },
                PositionalArg { name: "style" },
            ],
            has_trailing_positionals: false,
        }
    }

    fn create_atoms_syntax() -> CommandSyntax {
        let mut syntax = CommandSyntax::new("create_atoms");
        syntax.add_positional("type");
        syntax.add_style(Style {
            name: "box",
            arg_count: 0,
            arg_names: None,
        });
        syntax.add_style(Style {
            name: "region",
            arg_count: 1,
            arg_names: Some(vec!["region-ID"]),
        });

        syntax.add_style(Style {
            name: "single",
            arg_count: 3,
            arg_names: Some(vec!["x", "y", "z"]),
        });

        syntax.add_style(Style {
            name: "mesh",
            arg_count: 1,
            arg_names: Some(vec!["STL-file"]),
        });

        syntax.add_style(Style {
            name: "random",
            arg_count: 3,
            arg_names: Some(vec!["N", "seed", "region-ID"]),
        });

        syntax.add_kwargs([
            kwarg!("mol", 2; "template-ID","seed"),
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
        ]);

        syntax
    }

    use super::*;

    #[test]
    fn syntax_creation() {
        pretty_assertions::assert_eq!(create_atoms_syntax(), create_atoms_syntax_manual())
    }

    #[test]
    fn create_atoms_box() {
        let example1 = "create_atoms 1 box".split_whitespace().collect_vec();
        create_atoms_syntax().parse(example1);
    }

    #[test]
    #[should_panic = "Invalid keyword `extra_arg` or unexpected trailing positional argument"]
    fn create_atoms_box_bad() {
        let example1 = "create_atoms 1 box extra_arg"
            .split_whitespace()
            .collect_vec();
        dbg![create_atoms_syntax().parse(example1)];
    }

    #[test]
    fn create_atoms_region() {
        let example1 = "create_atoms 2 region mybox"
            .split_whitespace()
            .collect_vec();
        dbg![create_atoms_syntax().parse(example1)];
    }

    #[test]
    #[should_panic = "invalid syntax, expected 1 arguments for style `region`, only found 0"]
    fn create_atoms_region_bad() {
        let example1 = "create_atoms 2 region".split_whitespace().collect_vec();
        dbg![create_atoms_syntax().parse(example1)];
    }

    #[test]
    fn create_atoms_region_kwarg() {
        let example1 = "create_atoms 2 region mybox basis 2 3"
            .split_whitespace()
            .collect_vec();
        dbg![create_atoms_syntax().parse(example1)];
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
            dbg![create_atoms_syntax().parse(example)];
        }
    }
}
