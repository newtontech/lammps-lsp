use crate::command_syntax::CommandSyntax;

mod boundary;
mod create_atoms;
mod thermo_style;

pub(crate) fn syntax(command_name: &str) -> Option<CommandSyntax> {
    match command_name {
        "boundary" => Some(boundary::syntax()),
        "create_atoms" => Some(create_atoms::syntax()),
        "thermo_style" => Some(thermo_style::syntax()),
        _ => None,
    }
}
