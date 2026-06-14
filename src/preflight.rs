//! Universal generated-input preflight capabilities for the LAMMPS fleet backend.
//!
//! Implements the four cross-fleet capabilities tracked in newtontech/lammps-lsp#33:
//! version-aware keywords, cross-artifact graph, code-actions metadata, and
//! fleet-regression fixtures via [`fleet_manifest`].

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::ast::{ArgumentKind, Ast, Command};

pub const ROLE_PRIMARY_INPUT: &str = "primary-input";
pub const ROLE_STRUCTURE: &str = "structure";
pub const ROLE_INCLUDE: &str = "include";
pub const ROLE_POTENTIAL: &str = "potential";
pub const ROLE_TASK_PARAMETERS: &str = "task-parameters";

pub const CODE_MISSING_PRIMARY: &str = "LAMMPS601";
pub const CODE_UNRESOLVED_STRUCTURE: &str = "LAMMPS602";
pub const CODE_UNRESOLVED_INCLUDE: &str = "LAMMPS603";
pub const CODE_UNRESOLVED_POTENTIAL: &str = "LAMMPS604";
pub const CODE_READ_DATA_WITHOUT_PAIR_STYLE: &str = "LAMMPS605";
pub const CODE_KEYWORD_VERSION_MISMATCH: &str = "LAMMPS606";
pub const CODE_LARGE_TIMESTEP: &str = "LAMMPS607";
pub const CODE_VERSION_ASSUMPTION: &str = "LAMMPS608";

const DEFAULT_TIMESTEP_WARNING: f64 = 0.01;

#[derive(Debug, Clone)]
pub struct ArtifactNode {
    pub role: String,
    pub path: PathBuf,
    pub exists: bool,
    pub source: String,
    pub line: usize,
    pub detail: Option<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ArtifactGraph {
    pub input_path: PathBuf,
    pub nodes: Vec<ArtifactNode>,
}

impl ArtifactGraph {
    pub fn to_json(&self) -> Vec<Value> {
        let mut nodes: Vec<Value> = self
            .nodes
            .iter()
            .map(|node| {
                let mut payload = json!({
                    "role": node.role,
                    "path": node.path.to_string_lossy(),
                    "exists": node.exists,
                    "source": node.source,
                    "line": node.line,
                });
                if let Some(detail) = &node.detail {
                    payload["detail"] = detail.clone();
                }
                payload
            })
            .collect();
        nodes.sort_by(|left, right| {
            let left_key = (
                left.get("role").and_then(Value::as_str).unwrap_or(""),
                left.get("path").and_then(Value::as_str).unwrap_or(""),
            );
            let right_key = (
                right.get("role").and_then(Value::as_str).unwrap_or(""),
                right.get("path").and_then(Value::as_str).unwrap_or(""),
            );
            left_key.cmp(&right_key)
        });
        nodes
    }
}

pub fn resolve_version_assumption(intent: Option<&Value>) -> Value {
    let software_version = intent
        .and_then(|value| value.get("software_version"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let runtime_image = intent
        .and_then(|value| value.get("runtime_image"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let exact_runtime_known = software_version != "unknown" || runtime_image != "unknown";
    json!({
        "software": "lammps",
        "software_version": software_version,
        "runtime_image": runtime_image,
        "schema_source": intent
            .and_then(|value| value.get("schema_source"))
            .and_then(Value::as_str)
            .unwrap_or("lammps-lsp builtin"),
        "exact_runtime_known": exact_runtime_known,
        "declared_by": if exact_runtime_known { "intent" } else { "fallback" },
    })
}

pub fn load_intent(case_dir: &Path) -> Option<Value> {
    let intent_path = case_dir.join(".lammps-lsp").join("intent.json");
    let text = std::fs::read_to_string(intent_path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn looks_like_workspace(case_dir: &Path) -> bool {
    if !case_dir.is_dir() {
        return false;
    }
    const INPUT_NAMES: &[&str] = &[
        "in.lammps",
        "in.lmp",
        "input.lammps",
        "input.lmp",
        "lammps.in",
    ];
    INPUT_NAMES
        .iter()
        .any(|name| case_dir.join(name).is_file())
        || case_dir
            .read_dir()
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .any(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| matches!(ext, "lmp" | "lammps" | "lmps" | "in"))
            })
}

pub fn resolve_primary_input(case_dir: &Path) -> Option<PathBuf> {
    for name in ["in.lammps", "in.lmp", "input.lammps", "input.lmp", "lammps.in"] {
        let candidate = case_dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    case_dir
        .read_dir()
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| matches!(ext, "lmp" | "lammps" | "lmps" | "in"))
        })
        .min_by_key(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
}

pub fn build_artifact_graph(input_path: &Path, ast: &Ast, base_dir: &Path) -> ArtifactGraph {
    let mut graph = ArtifactGraph {
        input_path: input_path.to_path_buf(),
        ..Default::default()
    };
    graph.nodes.push(ArtifactNode {
        role: ROLE_PRIMARY_INPUT.to_string(),
        path: input_path.to_path_buf(),
        exists: input_path.exists(),
        source: "case-root".to_string(),
        line: 1,
        detail: None,
    });

    let mut has_pair_style = false;
    let mut has_read_data = false;

    for command in &ast.commands {
        let Command::Generic(cmd) = command else {
            continue;
        };
        let name = cmd.name.contents.as_str();
        let line = cmd.start.row + 1;
        match name {
            "pair_style" => has_pair_style = true,
            "read_data" => {
                has_read_data = true;
                if let Some(path_str) = first_word_arg(&cmd.args) {
                    let target = base_dir.join(path_str);
                    graph.nodes.push(ArtifactNode {
                        role: ROLE_STRUCTURE.to_string(),
                        path: target.clone(),
                        exists: target.exists(),
                        source: "read_data binding".to_string(),
                        line,
                        detail: Some(json!({"referenced_from": input_path.to_string_lossy()})),
                    });
                }
            }
            "include" => {
                if let Some(path_str) = first_word_arg(&cmd.args) {
                    let target = base_dir.join(path_str);
                    graph.nodes.push(ArtifactNode {
                        role: ROLE_INCLUDE.to_string(),
                        path: target.clone(),
                        exists: target.exists(),
                        source: "include binding".to_string(),
                        line,
                        detail: Some(json!({"referenced_from": input_path.to_string_lossy()})),
                    });
                }
            }
            "pair_coeff" => {
                if let Some(path_str) = potential_file_from_pair_coeff(&cmd.args) {
                    let target = base_dir.join(path_str);
                    graph.nodes.push(ArtifactNode {
                        role: ROLE_POTENTIAL.to_string(),
                        path: target.clone(),
                        exists: target.exists(),
                        source: "pair_coeff binding".to_string(),
                        line,
                        detail: Some(json!({"referenced_from": input_path.to_string_lossy()})),
                    });
                }
            }
            "timestep" | "run" | "thermo" => {
                graph.nodes.push(ArtifactNode {
                    role: ROLE_TASK_PARAMETERS.to_string(),
                    path: input_path.to_path_buf(),
                    exists: true,
                    source: format!("{name} directive"),
                    line,
                    detail: Some(json!({"command": name})),
                });
            }
            _ => {}
        }
    }

    if has_read_data && !has_pair_style {
        graph.nodes.push(ArtifactNode {
            role: ROLE_TASK_PARAMETERS.to_string(),
            path: input_path.to_path_buf(),
            exists: true,
            source: "workflow consistency".to_string(),
            line: 1,
            detail: Some(json!({"missing_after_read_data": "pair_style"})),
        });
    }

    graph
}

pub fn preflight_diagnostics(
    input_path: &Path,
    ast: &Ast,
    intent: Option<&Value>,
) -> (Vec<Value>, ArtifactGraph) {
    let base_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let version_assumption = resolve_version_assumption(intent);
    let graph = build_artifact_graph(input_path, ast, base_dir);
    let mut diagnostics = Vec::new();

    if !input_path.exists() {
        diagnostics.push(make_diagnostic(
            CODE_MISSING_PRIMARY,
            "error",
            format!(
                "primary input `{}` is missing from the workspace",
                input_path.display()
            ),
            input_path,
            1,
            "cross-file reference",
            true,
            vec![ROLE_PRIMARY_INPUT.to_string()],
            vec![format!("Create or restore `{}`", input_path.display())],
            json!({"role": ROLE_PRIMARY_INPUT}),
            Some(version_assumption),
        ));
        return (diagnostics, graph);
    }

    for node in &graph.nodes {
        if node.exists || node.role == ROLE_PRIMARY_INPUT || node.role == ROLE_TASK_PARAMETERS {
            continue;
        }
        let (code, category, blocking) = match node.role.as_str() {
            ROLE_STRUCTURE => (CODE_UNRESOLVED_STRUCTURE, "cross-file reference", true),
            ROLE_INCLUDE => (CODE_UNRESOLVED_INCLUDE, "cross-file reference", true),
            ROLE_POTENTIAL => (CODE_UNRESOLVED_POTENTIAL, "cross-file reference", true),
            _ => continue,
        };
        diagnostics.push(make_diagnostic(
            code,
            "error",
            format!(
                "unresolved {} artifact `{}`",
                node.role,
                node.path.to_string_lossy()
            ),
            input_path,
            node.line,
            category,
            blocking,
            vec![node.role.clone()],
            vec![format!(
                "Create `{}` or update the reference path",
                node.path.display()
            )],
            json!({
                "role": node.role,
                "artifact_path": node.path,
                "source": node.source,
            }),
            Some(version_assumption.clone()),
        ));
    }

    if graph.nodes.iter().any(|node| {
        node.detail
            .as_ref()
            .and_then(|detail| detail.get("missing_after_read_data"))
            .is_some()
    }) {
        diagnostics.push(make_diagnostic(
            CODE_READ_DATA_WITHOUT_PAIR_STYLE,
            "warning",
            "read_data is declared but no pair_style was found before submission preflight".to_string(),
            input_path,
            1,
            "semantic consistency",
            false,
            vec![ROLE_STRUCTURE.to_string(), ROLE_TASK_PARAMETERS.to_string()],
            vec!["Add a pair_style command before or after read_data".to_string()],
            json!({"missing_command": "pair_style"}),
            Some(version_assumption.clone()),
        ));
    }

    diagnostics.extend(keyword_version_diagnostics(input_path, ast, &version_assumption));
    diagnostics.extend(large_timestep_diagnostics(input_path, ast, intent));
    diagnostics.extend(version_assumption_diagnostic(&version_assumption, intent));

    (diagnostics, graph)
}

pub fn fleet_manifest(fixtures: &[Value]) -> Value {
    json!({
        "software": "lammps",
        "backend": "lammps-lsp",
        "preflight_envelope": "DiagnosticEnvelope/v1",
        "capabilities": [
            "version-aware-keywords",
            "cross-artifact-graph",
            "code-actions",
            "fleet-regression-fixtures",
        ],
        "artifact_roles": [
            ROLE_PRIMARY_INPUT,
            ROLE_STRUCTURE,
            ROLE_INCLUDE,
            ROLE_POTENTIAL,
            ROLE_TASK_PARAMETERS,
        ],
        "codes": {
            CODE_MISSING_PRIMARY: {
                "severity": "error",
                "category": "cross-file reference",
                "blocking": true,
                "capability": "cross-artifact-graph",
            },
            CODE_UNRESOLVED_STRUCTURE: {
                "severity": "error",
                "category": "cross-file reference",
                "blocking": true,
                "capability": "cross-artifact-graph",
            },
            CODE_UNRESOLVED_INCLUDE: {
                "severity": "error",
                "category": "cross-file reference",
                "blocking": true,
                "capability": "cross-artifact-graph",
            },
            CODE_UNRESOLVED_POTENTIAL: {
                "severity": "error",
                "category": "cross-file reference",
                "blocking": true,
                "capability": "cross-artifact-graph",
            },
            CODE_READ_DATA_WITHOUT_PAIR_STYLE: {
                "severity": "warning",
                "category": "semantic consistency",
                "blocking": false,
                "capability": "cross-artifact-graph",
            },
            CODE_KEYWORD_VERSION_MISMATCH: {
                "severity": "information",
                "category": "schema",
                "blocking": false,
                "capability": "version-aware-keywords",
            },
            CODE_LARGE_TIMESTEP: {
                "severity": "warning",
                "category": "preflight/runtime-risk",
                "blocking": false,
                "capability": "version-aware-keywords",
            },
            CODE_VERSION_ASSUMPTION: {
                "severity": "information",
                "category": "preflight/runtime-risk",
                "blocking": false,
                "capability": "version-aware-keywords",
            },
        },
        "fixtures": fixtures,
    })
}

fn keyword_version_diagnostics(
    input_path: &Path,
    ast: &Ast,
    version_assumption: &Value,
) -> Vec<Value> {
    let declared = version_assumption
        .get("software_version")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let floors: HashMap<&str, &str> = HashMap::from([
        ("pair_style ml/rann", ">=2024"),
        ("pair_style mace", ">=2024"),
        ("kspace_style pppm/disp", ">=2022"),
        ("fix plumed", ">=2022"),
    ]);
    let mut seen = HashSet::new();
    let mut diagnostics = Vec::new();

    for command in &ast.commands {
        let Command::Generic(cmd) = command else {
            continue;
        };
        let line = cmd.start.row + 1;
        let keyword = match cmd.name.contents.as_str() {
            "pair_style" | "kspace_style" => {
                let style = first_word_arg(&cmd.args).unwrap_or("");
                format!("{} {}", cmd.name.contents, style)
            }
            "fix" => {
                let style = fix_style_from_args(&cmd.args).unwrap_or("");
                format!("fix {style}")
            }
            other => other.to_string(),
        };
        let Some(floor) = floors.get(keyword.as_str()) else {
            continue;
        };
        if !seen.insert(keyword.clone()) {
            continue;
        }
        diagnostics.push(make_diagnostic(
            CODE_KEYWORD_VERSION_MISMATCH,
            "information",
            format!(
                "command '{keyword}' requires LAMMPS {floor}; declared runtime version is {declared}"
            ),
            input_path,
            line,
            "schema",
            false,
            vec![ROLE_PRIMARY_INPUT.to_string()],
            vec![
                format!("Confirm the runtime satisfies LAMMPS {floor}"),
                "Declare software_version in the intent contract".to_string(),
            ],
            json!({
                "keyword": keyword,
                "required_floor": floor,
                "declared_version": declared,
                "schema_source": version_assumption.get("schema_source"),
            }),
            Some(version_assumption.clone()),
        ));
    }
    diagnostics
}

fn large_timestep_diagnostics(
    input_path: &Path,
    ast: &Ast,
    intent: Option<&Value>,
) -> Vec<Value> {
    let threshold = intent
        .and_then(|value| value.get("timestep_warning"))
        .and_then(Value::as_f64)
        .unwrap_or(DEFAULT_TIMESTEP_WARNING);
    let mut diagnostics = Vec::new();
    for command in &ast.commands {
        let Command::Generic(cmd) = command else {
            continue;
        };
        if cmd.name.contents != "timestep" {
            continue;
        }
        let Some(raw) = first_word_arg(&cmd.args) else {
            continue;
        };
        let Ok(dt) = raw.parse::<f64>() else {
            continue;
        };
        if dt <= threshold {
            continue;
        }
        diagnostics.push(make_diagnostic(
            CODE_LARGE_TIMESTEP,
            "warning",
            format!(
                "timestep={dt} exceeds the conservative workflow threshold ({threshold})"
            ),
            input_path,
            cmd.start.row + 1,
            "preflight/runtime-risk",
            false,
            vec![ROLE_TASK_PARAMETERS.to_string()],
            vec![
                format!("Lower timestep to at most {threshold}"),
                "Or document the larger step in the intent contract".to_string(),
            ],
            json!({"timestep": dt, "threshold": threshold}),
            None,
        ));
    }
    diagnostics
}

fn version_assumption_diagnostic(
    version_assumption: &Value,
    intent: Option<&Value>,
) -> Vec<Value> {
    if version_assumption
        .get("exact_runtime_known")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Vec::new();
    }
    vec![make_diagnostic(
        CODE_VERSION_ASSUMPTION,
        "information",
        "Exact LAMMPS runtime/image version is unknown; preflight validated against the builtin keyword set".to_string(),
        Path::new(
            version_assumption
                .get("schema_source")
                .and_then(Value::as_str)
                .unwrap_or("lammps-lsp builtin"),
        ),
        1,
        "preflight/runtime-risk",
        false,
        vec![ROLE_PRIMARY_INPUT.to_string()],
        vec!["Declare software_version/runtime_image in the intent contract".to_string()],
        json!({
            "software_version": version_assumption.get("software_version"),
            "runtime_image": version_assumption.get("runtime_image"),
            "intent_present": intent.is_some(),
        }),
        Some(version_assumption.clone()),
    )]
}

#[allow(clippy::too_many_arguments)]
fn make_diagnostic(
    code: &str,
    severity: &str,
    message: String,
    path: &Path,
    line: usize,
    category: &str,
    blocking: bool,
    artifact_roles: Vec<String>,
    fix_hints: Vec<String>,
    source_provenance: Value,
    version_assumption: Option<Value>,
) -> Value {
    let character = 0usize;
    let mut diagnostic = json!({
        "diagnostic_engine": "1.0",
        "code": code,
        "severity": severity,
        "category": category,
        "confidence": 1.0,
        "source": "lammps-preflight",
        "range": {
            "start": {"line": line.saturating_sub(1), "character": character},
            "end": {"line": line.saturating_sub(1), "character": character + 1},
        },
        "software": "lammps",
        "file_type": path.extension().and_then(|ext| ext.to_str()).unwrap_or("input"),
        "path": path.to_string_lossy(),
        "expected": null,
        "actual": null,
        "manual_ref": null,
        "fix_hints": fix_hints,
        "actions": fix_hints.iter().map(|hint| json!({
            "kind": "repair_hint",
            "summary": hint,
            "target": path.to_string_lossy(),
            "safe_to_auto_apply": false,
            "apply_policy": "preview_only",
        })).collect::<Vec<_>>(),
        "blocking": blocking,
        "message": message,
        "source_provenance": source_provenance,
        "artifact_roles": artifact_roles,
        "domain_tags": ["preflight"],
    });
    if let Some(version_assumption) = version_assumption {
        diagnostic["version_assumption"] = version_assumption;
    }
    diagnostic
}

fn first_word_arg(args: &[crate::ast::Argument]) -> Option<&str> {
    args.iter().find_map(|arg| match &arg.kind {
        ArgumentKind::Word(word) => Some(word.as_str()),
        _ => None,
    })
}

fn fix_style_from_args(args: &[crate::ast::Argument]) -> Option<&str> {
    let words: Vec<&str> = args
        .iter()
        .filter_map(|arg| match &arg.kind {
            ArgumentKind::Word(word) => Some(word.as_str()),
            _ => None,
        })
        .collect();
    if words.len() >= 3 {
        Some(words[2])
    } else {
        None
    }
}

fn potential_file_from_pair_coeff(args: &[crate::ast::Argument]) -> Option<&str> {
    for arg in args {
        let ArgumentKind::Word(word) = &arg.kind else {
            continue;
        };
        if word.ends_with(".eam")
            || word.ends_with(".eam.alloy")
            || word.ends_with(".eam.fs")
            || word.ends_with(".meam")
            || word.ends_with(".snap")
            || word.ends_with(".sw")
        {
            return Some(word.as_str());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::ts_to_ast, utils};

    #[test]
    fn unresolved_structure_emits_preflight_code() {
        let source = "read_data nonexistent.data\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let tmp = std::env::temp_dir();
        let input = tmp.join("lammps_preflight_test.in");
        std::fs::write(&input, source).unwrap();
        let (diagnostics, graph) = preflight_diagnostics(&input, &ast, None);
        assert!(graph.nodes.iter().any(|node| node.role == ROLE_STRUCTURE));
        assert!(diagnostics.iter().any(|diag| {
            diag.get("code").and_then(Value::as_str) == Some(CODE_UNRESOLVED_STRUCTURE)
        }));
        std::fs::remove_file(input).ok();
    }

    #[test]
    fn fleet_manifest_lists_capabilities() {
        let manifest = fleet_manifest(&[]);
        let capabilities = manifest
            .get("capabilities")
            .and_then(Value::as_array)
            .unwrap();
        assert!(capabilities.contains(&json!("cross-artifact-graph")));
    }
}
