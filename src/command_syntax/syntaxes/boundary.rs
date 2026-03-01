use crate::command_syntax::CommandSyntax;

pub(crate) fn syntax() -> CommandSyntax {
    // TODO: support requiring specific value for arguments

    let mut syntax = CommandSyntax::new("boundary");

    syntax
        .add_positional("x")
        .add_positional("y")
        .add_positional("z");

    syntax
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn lmp_examples() {
        let x = "boundary p p f".split_whitespace().collect_vec();
        insta::assert_debug_snapshot!(super::syntax().parse(x).expect("Should parse"));

        let x = "boundary p fs p".split_whitespace().collect_vec();
        insta::assert_debug_snapshot!(super::syntax().parse(x).expect("Should parse"));

        let x = "boundary s f fm".split_whitespace().collect_vec();
        insta::assert_debug_snapshot!(super::syntax().parse(x).expect("Should parse"));
    }
}
