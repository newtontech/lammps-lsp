# OpenQC Agent Context

OpenQC consumes `lammps-lsp-tool` and `lsp-capabilities.json` to assemble diagnostics, hover, completion, symbols, examples, next-token guidance, and repair-plan hints for `lammps` documents.

## Wiki-grounded surfaces

| LSP operation | Wiki evidence | Raw evidence |
|---------------|---------------|--------------|
| `check` / diagnostics | [Diagnostic Engine v1](../concepts/diagnostic-engine-v1.md) | `src/lints/codes.rs`, `diagnostics/diagnostic-engine-v1.schema.json` |
| `hover` | [Entity pages](../entities/) (commands, pair/fix/compute styles) | `raw/assets/lammps_docs_md/`, `docs_extract/index_map.txt` |
| `complete` | [Input Script Workflow](Input_Script_Workflow.md), [Common Patterns](Common_Patterns.md) | `raw/assets/examples/in.melt` |
| `context` | [OpenQC agent context](openqc-agent-context.md) (this page) | `lsp-capabilities.json` → `llmWiki` |
| `symbols` | [Variable_Command](../entities/Variable_Command.md), [Group_Command](../entities/Group_Command.md) | tree-sitter LAMMPS grammar |
| `fix` | Diagnostic fix hints from lint rules | `tests/fixtures/rules/*.json` |

## Upstream manuals

Official LAMMPS documentation is indexed in `raw/assets/lammps-upstream-sources.md`. Hover text is derived from the 990-page manual digest under `raw/assets/lammps_docs_md/`.

## Example workflow

Start from [Input_Script_Workflow](Input_Script_Workflow.md) and the upstream melt tutorial at `raw/assets/examples/in.melt` (source: [lammps/examples/melt/in.melt](https://github.com/lammps/lammps/blob/develop/examples/melt/in.melt)).
