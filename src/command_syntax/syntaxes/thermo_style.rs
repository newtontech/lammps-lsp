use crate::command_syntax::{CommandSyntax, Style};

fn syntax() -> CommandSyntax {
    let mut syntax = CommandSyntax::new("thermo_style");

    syntax
        .add_style(Style::new("one"))
        .add_style(Style::new("multi"))
        .add_style(Style::new("yaml"))
        .add_style(Style::new("custom").add_arg("args"));

    syntax
}
