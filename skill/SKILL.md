---
name: lammps
description: "LAMMPS input preflight for generated molecular dynamics scripts."
---

# LAMMPS LSP Skill

Use this skill when preparing, repairing, or reviewing LAMMPS input scripts before a run. It provides an installable language server and an agent-facing CLI that reports machine-readable diagnostics.

## Scope

- Input patterns: `*.lmp`, `*.lammps`, `*.lmps`, `in.lammps`
- Server command: `lmp-lsp`
- Agent CLI: `lammps-lsp-tool`
- Diagnostic contract: `DiagnosticEnvelope/v1`

## Installing the checker

```bash
pip install lammps-lsp
```

This installs the `lmp-lsp` language server and the `lammps-lsp-tool` agent CLI from the `lammps-lsp` Python wheel.

## Useful inspection commands

```bash
lammps-lsp-tool capabilities
lammps-lsp-tool skill-spec --format json
lammps-lsp-tool skill-export --output ./skill
lammps-lsp-tool check <input-file> --format json
lammps-lsp-tool context <input-file> --line 0 --character 0 --format json
lammps-lsp-tool hover <input-file> --line 0 --character 0 --format json
lammps-lsp-tool complete <input-file> --line 0 --character 0 --format json
lammps-lsp-tool symbols <input-file> --format json
lammps-lsp-tool fix <input-file> --line 0 --character 0 --format json
```

`fix` is advisory and must be treated as a preview. Do not blindly apply a repair without preserving the user's scientific intent.

## Validation gate

Before saying generated inputs are ready, run:

```bash
lammps-lsp-tool check <input-file> --format json --fail-on-blocking
```

Report `commands`, `files_checked`, `tool_available`, `diagnostics`, `blocking_findings`, `readiness`, and `reason`.

## Repair rules

1. Validate first and identify the smallest blocking issue.
2. Fix syntax or schema errors with minimal edits.
3. Preserve scientific settings unless the user explicitly asks to redesign them.
4. Re-run the checker after every edit.
5. Separate syntax, schema, semantic, and runtime-log diagnostics in the final report.
