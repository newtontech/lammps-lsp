use crate::command_syntax::{kwarg, CommandSyntax, Nargs, Style};

fn create_atoms_syntax() -> CommandSyntax {
    let mut syntax = CommandSyntax::new("create_atoms");
    syntax.add_positional("type");
    syntax.add_style(Style {
        name: "box",
        arg_count: Nargs::None,
        arg_names: None,
    });
    syntax.add_style(Style {
        name: "region",
        arg_count: 1.into(),
        arg_names: Some(vec!["region-ID"]),
    });

    syntax.add_style(Style {
        name: "single",
        arg_count: 3.into(),
        arg_names: Some(vec!["x", "y", "z"]),
    });

    syntax.add_style(Style {
        name: "mesh",
        arg_count: 1.into(),
        arg_names: Some(vec!["STL-file"]),
    });

    syntax.add_style(Style {
        name: "random",
        arg_count: 3.into(),
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
                    Style::new("box"),
                    Style::new("region").add_arg("region-ID"),
                    Style::new("single").add_arg("x").add_arg("y").add_arg("z"),
                    Style::new("mesh").add_arg("STL-file"),
                    Style::new("random")
                        .add_arg("N")
                        .add_arg("seed")
                        .add_arg("region-ID"),
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

    use crate::command_syntax::{PositionalArg, Styles};

    use super::*;

    #[test]
    fn syntax_creation() {
        pretty_assertions::assert_eq!(create_atoms_syntax(), create_atoms_syntax_manual())
    }

    #[test]
    fn create_atoms_box() {
        let example1 = "create_atoms 1 box".split_whitespace().collect_vec();
        create_atoms_syntax().parse(example1).expect("should parse");
    }

    #[test]
    #[should_panic = "invalid create_atoms command: invalid keyword extra_arg, valid keywords: mol, basis, ratio, subset, remap, var, set, radscale, rotate, overlap, maxtry, units, meshmode"]
    fn create_atoms_box_bad() {
        let example1 = "create_atoms 1 box extra_arg"
            .split_whitespace()
            .collect_vec();
        panic!["{}", create_atoms_syntax().parse(example1).unwrap_err()];
    }

    #[test]
    fn create_atoms_region() {
        let example1 = "create_atoms 2 region mybox"
            .split_whitespace()
            .collect_vec();
        dbg![create_atoms_syntax().parse(example1).expect("should parse")];
    }

    #[test]
    #[should_panic = "invalid create_atoms command: for style region, expected 1 positional arguments, found 0"]
    fn create_atoms_region_bad() {
        let example1 = "create_atoms 2 region".split_whitespace().collect_vec();
        panic!["{}", create_atoms_syntax().parse(example1).unwrap_err()];
    }

    #[test]
    fn create_atoms_region_kwarg() {
        let example1 = "create_atoms 2 region mybox basis 2 3"
            .split_whitespace()
            .collect_vec();
        dbg![create_atoms_syntax().parse(example1).expect("should parse")];
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
            dbg![create_atoms_syntax().parse(example).expect("should parse")];
        }
    }
}
