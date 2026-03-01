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
mod test {
    use insta::assert_debug_snapshot;

    use super::syntax;

    #[test]
    fn one() {
        assert_debug_snapshot!(syntax().parse(vec!["thermo_style", "one"]).unwrap());
    }

    #[test]
    fn multi() {
        assert_debug_snapshot!(syntax().parse(vec!["thermo_style", "multi"]).unwrap());
    }

    #[test]
    fn yaml() {
        assert_debug_snapshot!(syntax().parse(vec!["thermo_style", "yaml"]).unwrap());
    }

    #[test]
    fn custom_none() {
        assert_debug_snapshot!(syntax().parse(vec!["thermo_style", "custom"]).unwrap());
    }

    #[test]
    fn custom_one() {
        assert_debug_snapshot!(syntax()
            .parse(vec!["thermo_style", "custom", "step"])
            .unwrap());
    }

    #[test]
    fn custom_five() {
        assert_debug_snapshot!(syntax()
            .parse(vec![
                "thermo_style",
                "custom",
                "step",
                "temp",
                "press",
                "epair",
                "ecoul"
            ])
            .unwrap());
    }
}
