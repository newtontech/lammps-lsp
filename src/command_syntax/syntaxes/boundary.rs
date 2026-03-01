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
    use crate::ast::{from_node::FromNode, ts_to_ast, GenericCommand};

    fn command_helper(text: &str) -> GenericCommand {
        use crate::utils::parsing::setup_parser;

        let mut ast = ts_to_ast(
            &setup_parser().parse(text, None).expect("TS Parser Failed"),
            text,
        )
        .expect("failed to parse");

        std::mem::take(&mut ast.commands[0])
            .try_into_generic()
            .expect("failed to parse as GenericCommand")
    }

    #[test]
    fn lmp_example_1() {
        let x = command_helper("boundary p p f\n");
        insta::assert_debug_snapshot!(super::syntax().parse(&x).expect("Should parse"));
    }

    #[test]
    fn lmp_example_2() {
        let x = command_helper("boundary p fs p\n");
        insta::assert_debug_snapshot!(super::syntax().parse(&x).expect("Should parse"));
    }

    #[test]
    fn lmp_example_3() {
        let x = command_helper("boundary s f fm\n");
        insta::assert_debug_snapshot!(super::syntax().parse(&x).expect("Should parse"));
    }
}
