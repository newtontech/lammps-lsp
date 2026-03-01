use crate::command_syntax::{CommandSyntax, Style};

fn syntax() -> CommandSyntax {
    let mut syntax = CommandSyntax::new("thermo_style");

    syntax
        .add_style(Style::new("one"))
        .add_style(Style::new("multi"))
        .add_style(Style::new("yaml"))
        .add_style(Style::new("custom").add_var_arg("args"));

    syntax
}

#[cfg(test)]
/// For some reason, newlines needed to be appended to the examples in this module.
mod test {

    use crate::ast::{ts_to_ast, GenericCommand};

    use insta::assert_debug_snapshot;

    use super::syntax;

    fn command_helper(text: &str) -> GenericCommand {
        use crate::utils::parsing::setup_parser;

        let tree = setup_parser().parse(text, None).expect("TS Parser Failed");

        dbg!(&tree.root_node().to_sexp());

        let mut ast = ts_to_ast(&tree, text).expect("failed to parse");

        std::mem::take(&mut ast.commands[0])
            .try_into_generic()
            .expect("failed to parse as GenericCommand")
    }

    #[test]
    fn one() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper("thermo_style one\n"))
            .unwrap());
    }

    #[test]
    fn multi() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper("thermo_style multi\n"))
            .unwrap());
    }

    #[test]
    fn yaml() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper("thermo_style yaml\n"))
            .unwrap());
    }

    #[test]
    fn custom_none() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper("thermo_style custom\n"))
            .unwrap());
    }

    #[test]
    fn custom_one() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper("thermo_style custom step\n"))
            .unwrap());
    }

    #[test]
    fn custom_five() {
        assert_debug_snapshot!(syntax()
            .parse(&command_helper(
                "thermo_style custom step temp press epair ecoul\n"
            ))
            .unwrap());
    }
}
