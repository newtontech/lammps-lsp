//! Issue #10: Write AST into S-Expressions for testing.
//!
//! Provides `SExp` trait for serializing AST nodes into S-expression format,
//! useful for snapshot testing and debugging.

use crate::ast::expressions::{BinaryOp, Index, UnaryOp};
use crate::ast::{
    Argument, ArgumentKind, Ast, Command, ComputeDef, Expression, FixDef, VariableDef, Word,
};
use crate::spans::Span;

/// Trait for serializing AST nodes into S-expression strings.
pub trait SExp {
    /// Convert the node to an S-expression string.
    fn to_sexp(&self) -> String;
}

impl SExp for Ast {
    fn to_sexp(&self) -> String {
        if self.commands.is_empty() {
            return "(ast)".to_string();
        }
        let cmds: Vec<String> = self.commands.iter().map(|c| c.to_sexp()).collect();
        format!("(ast {})", cmds.join("\n     "))
    }
}

impl SExp for Command {
    fn to_sexp(&self) -> String {
        match self {
            Command::Generic(cmd) => {
                let args: Vec<String> = cmd.args.iter().map(|a| a.to_sexp()).collect();
                if args.is_empty() {
                    format!("(command {})", cmd.name.to_sexp())
                } else {
                    format!("(command {} {})", cmd.name.to_sexp(), args.join(" "))
                }
            }
            Command::Fix(fix) => fix.to_sexp(),
            Command::Compute(compute) => compute.to_sexp(),
            Command::VariableDef(var) => var.to_sexp(),
            Command::Shell(span) => format!("(shell {})", span.to_sexp()),
            Command::Error(span) => format!("(error {})", span.to_sexp()),
        }
    }
}

impl SExp for Word {
    fn to_sexp(&self) -> String {
        self.contents.to_string()
    }
}

impl SExp for Argument {
    fn to_sexp(&self) -> String {
        self.kind.to_sexp()
    }
}

impl SExp for ArgumentKind {
    fn to_sexp(&self) -> String {
        match self {
            Self::Int(n) => format!("{n}"),
            Self::Float(f) => format!("{f}"),
            Self::Bool(b) => format!("{b}"),
            Self::Word(w) => w.to_string(),
            Self::String(s) => format!("\"{}\"", s),
            Self::RawString(s) => format!("'{}'", s),
            Self::TripleString(s) => format!("\"\"\"{}\"\"\"", s),
            Self::ArgName(a) => format!("(argname {a})"),
            Self::Group => "(group)".to_string(),
            Self::Expression(expr) => expr.to_sexp(),
            Self::VarCurly(ident) => format!("(var-curly {})", ident.name),
            Self::SimpleExpansion(ident) => format!("(simple-expand {})", ident.name),
            Self::VarRound(expr) => format!("(var-round {})", expr.to_sexp()),
            Self::Concatenation(args) => {
                let inner: Vec<String> = args.iter().map(|a| a.to_sexp()).collect();
                format!("(concat {})", inner.join(" "))
            }
            Self::UnderscoreIdent(ident) => format!("(underscore {})", ident.name),
            Self::IndexedIdent(ident, index) => {
                format!("(indexed {} [{}])", ident.name, index_to_sexp(index))
            }
            Self::Error => "(error-arg)".to_string(),
        }
    }
}

/// Helper to convert Index to sexp string (avoids conflicting Display impl).
fn index_to_sexp(index: &Index) -> String {
    match index {
        Index::Int(n) => format!("{n}"),
        Index::Glob => "*".to_string(),
        Index::Expression(expr) => expr.to_sexp(),
    }
}

/// Helper to convert BinaryOp to sexp string.
fn binop_to_sexp(op: &BinaryOp) -> String {
    match op {
        BinaryOp::Add => "+".to_string(),
        BinaryOp::Subtract => "-".to_string(),
        BinaryOp::Multiply => "*".to_string(),
        BinaryOp::Divide => "/".to_string(),
        BinaryOp::Power => "^".to_string(),
        BinaryOp::Modulo => "%".to_string(),
        BinaryOp::Equal => "==".to_string(),
        BinaryOp::NotEqual => "!=".to_string(),
        BinaryOp::LessThan => "<".to_string(),
        BinaryOp::LessThanOrEqual => "<=".to_string(),
        BinaryOp::GreaterThan => ">".to_string(),
        BinaryOp::GreaterThanOrEqual => ">=".to_string(),
        BinaryOp::And => "&&".to_string(),
        BinaryOp::Or => "||".to_string(),
        BinaryOp::Xor => "^|".to_string(),
    }
}

/// Helper to convert UnaryOp to sexp string.
fn unaryop_to_sexp(op: &UnaryOp) -> String {
    match op {
        UnaryOp::Negate => "neg".to_string(),
        UnaryOp::Not => "not".to_string(),
    }
}

impl SExp for Expression {
    fn to_sexp(&self) -> String {
        match self {
            Self::Missing => "(missing)".to_string(),
            Self::Int(n) => format!("{n}"),
            Self::Float(f) => format!("{f}"),
            Self::Bool(b) => format!("{b}"),
            Self::UnderscoreIdent(ident) => format!("(ident {})", ident.name),
            Self::UnaryOp(op, expr) => format!("({} {})", unaryop_to_sexp(op), expr.to_sexp()),
            Self::BinaryOp(lhs, op, rhs) => {
                format!(
                    "({} {} {})",
                    binop_to_sexp(op),
                    lhs.to_sexp(),
                    rhs.to_sexp()
                )
            }
            Self::Function(name, args) => {
                let inner: Vec<String> = args.iter().map(|a| a.to_sexp()).collect();
                format!("(call {} {})", name.contents, inner.join(" "))
            }
            Self::Parens(expr) => format!("(parens {})", expr.to_sexp()),
            Self::ThermoKeyword(kw) => format!("(thermo {})", kw.contents),
            Self::AtomProperty(kw) => format!("(atom-prop {})", kw.contents),
            Self::Constant(c) => format!("(const {})", c.contents),
            Self::Word(w) => format!("(word {})", w.contents),
            Self::Indexing(expr, idx) => {
                format!("(index {} [{}])", expr.to_sexp(), index_to_sexp(idx))
            }
            Self::VarRound(expr) => format!("($({}))", expr.to_sexp()),
            Self::VarCurly(ident) => format!("(${{{}}})", ident.name),
            Self::SimpleExpansion(ident) => format!("(${})", ident.name),
        }
    }
}

impl SExp for FixDef {
    fn to_sexp(&self) -> String {
        let args: Vec<String> = self.args.iter().map(|a| a.to_sexp()).collect();
        let args_str = if args.is_empty() {
            String::new()
        } else {
            format!(" {}", args.join(" "))
        };
        format!(
            "(fix {} {} {}{})",
            self.fix_id.name, self.group_id.contents, self.fix_style, args_str
        )
    }
}

impl SExp for ComputeDef {
    fn to_sexp(&self) -> String {
        let args: Vec<String> = self.args.iter().map(|a| a.to_sexp()).collect();
        let args_str = if args.is_empty() {
            String::new()
        } else {
            format!(" {}", args.join(" "))
        };
        format!(
            "(compute {} {} {}{})",
            self.compute_id.name, self.group_id.contents, self.compute_style, args_str
        )
    }
}

impl SExp for VariableDef {
    fn to_sexp(&self) -> String {
        let args: Vec<String> = self.args.iter().map(|a| a.to_sexp()).collect();
        let args_str = if args.is_empty() {
            String::new()
        } else {
            format!(" {}", args.join(" "))
        };
        format!(
            "(variable {} {}{})",
            self.variable_id.name, self.variable_style.contents, args_str
        )
    }
}

impl SExp for Span {
    fn to_sexp(&self) -> String {
        format!(
            "({}:{}-{}:{})",
            self.start.row, self.start.column, self.end.row, self.end.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn empty_ast_sexp() {
        let ast = Ast::default();
        assert_eq!(ast.to_sexp(), "(ast)");
    }

    #[test]
    fn generic_command_sexp() {
        let source = "units metal\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let sexp = ast.to_sexp();
        assert!(sexp.starts_with("(ast"));
        assert!(sexp.contains("units"));
        assert!(sexp.contains("metal"));
    }

    #[test]
    fn fix_def_sexp() {
        let source = "fix NVE all nve\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let sexp = ast.to_sexp();
        assert!(sexp.contains("fix"));
        assert!(sexp.contains("NVE"));
    }

    #[test]
    fn variable_def_sexp() {
        let source = "variable a equal 1.0\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let sexp = ast.to_sexp();
        assert!(sexp.contains("variable"));
        assert!(sexp.contains("equal"));
    }

    #[test]
    fn span_sexp() {
        let span = Span {
            start: crate::spans::Point { row: 0, column: 5 },
            end: crate::spans::Point { row: 2, column: 10 },
        };
        assert_eq!(span.to_sexp(), "(0:5-2:10)");
    }
}
