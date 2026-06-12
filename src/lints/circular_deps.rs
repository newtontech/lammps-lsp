//! Issue #13: Highlight circular dependencies in variables.
//!
//! Detects when variables reference each other in a cycle, e.g.:
//! ```lammps
//! variable a equal v_b
//! variable b equal v_a
//! ```

use std::collections::{HashMap, HashSet, VecDeque};

use crate::ast::{Ast, Command, VariableDef};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

/// Represents a circular dependency cycle found among variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircularVariableDependency {
    /// The variables involved in the cycle, in order.
    pub cycle: Vec<String>,
    /// Span of the variable where the cycle was first detected.
    pub span: Span,
}

impl CircularVariableDependency {
    /// Detect circular variable dependencies in the AST.
    ///
    /// Builds a dependency graph from variable definitions and checks for cycles
    /// using iterative DFS.
    pub fn find_all(ast: &Ast) -> Vec<Self> {
        let var_defs = collect_variable_defs(ast);
        let dep_graph = build_dependency_graph(&var_defs);
        let mut visited: HashSet<String> = HashSet::new();
        let mut results = Vec::new();

        for var_name in dep_graph.keys() {
            if visited.contains(var_name) {
                continue;
            }

            if let Some(cycle) = detect_cycle_from(var_name, &dep_graph, &mut visited) {
                // Find the span of the first variable in the cycle
                let span = var_defs
                    .iter()
                    .find(|(name, _)| name == &cycle[0])
                    .map(|(_, def)| def.span)
                    .unwrap_or_default();

                results.push(CircularVariableDependency { cycle, span });
            }
        }

        results
    }
}

/// Collect all variable definitions from the AST, returning (name, definition) pairs.
fn collect_variable_defs(ast: &Ast) -> Vec<(String, &VariableDef)> {
    ast.commands
        .iter()
        .filter_map(|cmd| match cmd {
            Command::VariableDef(def) => {
                // Skip delete commands
                if def.variable_style.contents == "delete" {
                    return None;
                }
                Some((def.variable_id.name.clone(), def))
            }
            _ => None,
        })
        .collect()
}

/// Extract variable references from a variable definition.
///
/// Looks for `v_XXX` references and `$XXX` expansions in the arguments.
fn extract_var_refs(def: &VariableDef) -> Vec<String> {
    let mut refs = Vec::new();

    for arg in &def.args {
        extract_refs_from_arg(&arg.kind, &mut refs);
    }

    refs
}

fn extract_refs_from_arg(arg_kind: &crate::ast::ArgumentKind, refs: &mut Vec<String>) {
    use crate::ast::ArgumentKind;

    match arg_kind {
        ArgumentKind::UnderscoreIdent(ident) => {
            if matches!(ident.ident_type, crate::ast::IdentType::Variable) {
                refs.push(ident.name.clone());
            }
        }
        ArgumentKind::SimpleExpansion(ident) => {
            refs.push(ident.name.clone());
        }
        ArgumentKind::VarCurly(ident) => {
            refs.push(ident.name.clone());
        }
        ArgumentKind::Expression(expr) => {
            extract_refs_from_expr(expr, refs);
        }
        ArgumentKind::VarRound(expr) => {
            extract_refs_from_expr(expr, refs);
        }
        ArgumentKind::Concatenation(args) => {
            for a in args {
                extract_refs_from_arg(&a.kind, refs);
            }
        }
        _ => {}
    }
}

/// Extract variable references from an Expression.
fn extract_refs_from_expr(expr: &crate::ast::Expression, refs: &mut Vec<String>) {
    use crate::ast::Expression;

    match expr {
        Expression::UnderscoreIdent(ident) => {
            if matches!(ident.ident_type, crate::ast::IdentType::Variable) {
                refs.push(ident.name.clone());
            }
        }
        Expression::SimpleExpansion(ident) => {
            refs.push(ident.name.clone());
        }
        Expression::VarCurly(ident) => {
            refs.push(ident.name.clone());
        }
        Expression::VarRound(inner) => {
            extract_refs_from_expr(inner, refs);
        }
        Expression::BinaryOp(lhs, _, rhs) => {
            extract_refs_from_expr(lhs, refs);
            extract_refs_from_expr(rhs, refs);
        }
        Expression::UnaryOp(_, inner) => {
            extract_refs_from_expr(inner, refs);
        }
        Expression::Parens(inner) => {
            extract_refs_from_expr(inner, refs);
        }
        Expression::Function(_, args) => {
            for arg in args {
                extract_refs_from_expr(arg, refs);
            }
        }
        Expression::Indexing(inner, _) => {
            extract_refs_from_expr(inner, refs);
        }
        _ => {}
    }
}

/// Build a directed graph: variable name -> set of variables it references.
fn build_dependency_graph(var_defs: &[(String, &VariableDef)]) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (name, def) in var_defs {
        let deps = extract_var_refs(def);
        graph.insert(name.clone(), deps);
    }

    graph
}

/// Detect a cycle starting from the given variable using iterative DFS.
fn detect_cycle_from(
    start: &str,
    graph: &HashMap<String, Vec<String>>,
    global_visited: &mut HashSet<String>,
) -> Option<Vec<String>> {
    let mut stack: VecDeque<(String, Vec<String>)> = VecDeque::new();
    stack.push_back((start.to_string(), vec![start.to_string()]));

    let mut local_visited: HashSet<String> = HashSet::new();

    while let Some((current, path)) = stack.pop_back() {
        if global_visited.contains(&current) {
            continue;
        }

        // Check if we've already visited this node in the current traversal
        if local_visited.contains(&current) && path.len() > 1 {
            // Found a cycle - extract the cycle portion
            if let Some(cycle_start) = path.iter().position(|p| p == &current) {
                return Some(path[cycle_start..].to_vec());
            }
        }

        local_visited.insert(current.clone());

        if let Some(deps) = graph.get(&current) {
            for dep in deps {
                if dep == start {
                    // Found a cycle back to start
                    let mut cycle_path = path;
                    cycle_path.push(dep.clone());
                    return Some(cycle_path);
                }

                if !local_visited.contains(dep) {
                    let mut new_path = path.clone();
                    new_path.push(dep.clone());
                    stack.push_back((dep.clone(), new_path));
                }
            }
        }
    }

    global_visited.insert(start.to_string());
    None
}

impl Issue for CircularVariableDependency {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            code: None,
            name: "circular-variable-dep",
            severity: Severity::Error,
            span: self.span,
            message: format!(
                "LAMMPS-E500: circular variable dependency detected: {}",
                self.cycle.join(" -> ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn detects_simple_cycle() {
        let source = "variable a equal v_b\nvariable b equal v_a\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = CircularVariableDependency::find_all(&ast);
        assert!(!found.is_empty(), "should detect circular dependency");
        assert!(found[0].cycle.contains(&"a".to_string()));
        assert!(found[0].cycle.contains(&"b".to_string()));
    }

    #[test]
    fn no_cycle_linear_deps() {
        let source = "variable a equal 1.0\nvariable b equal v_a\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = CircularVariableDependency::find_all(&ast);
        assert!(found.is_empty(), "no cycle expected for linear deps");
    }

    #[test]
    fn no_cycle_no_deps() {
        let source = "variable a equal 1.0\nvariable b equal 2.0\n";
        let tree = utils::testing::parse(source);
        let ast = crate::ast::ts_to_ast(&tree, source).unwrap();

        let found = CircularVariableDependency::find_all(&ast);
        assert!(found.is_empty(), "no cycle for independent vars");
    }
}
