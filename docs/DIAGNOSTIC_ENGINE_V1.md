# Diagnostic Engine v1

This repository implements the shared newtontech Scientific LSP diagnostics
contract inspired by python-lsp-server's provider model: the editor-facing LSP
can keep native providers, while the agent-facing layer exposes deterministic
JSON for check/repair/recheck loops.

## Severity Policy

- `error`: high-confidence syntax, schema, type/value, or reference issue that
  should block automated submission because the upstream runtime is likely to
  reject the input.
- `warning`: high-risk or suspicious input that may be intentional and should
  be shown to agents without automatically blocking repair loops.
- `information` / `hint`: style, documentation, or optional optimization facts.

## Categories

- `syntax`
- `schema`
- `type/value`
- `cross-file reference`
- `semantic consistency`
- `preflight/runtime-risk`
- `style/deprecation`

## Rich Diagnostic Shape

Every agent-facing diagnostic must include:

```json
{
  "code": "STABLE_CODE",
  "severity": "error",
  "category": "schema",
  "confidence": 1.0,
  "source": "lammps-lsp",
  "range": {
    "start": {"line": 0, "character": 0},
    "end": {"line": 0, "character": 1}
  },
  "software": "lammps",
  "file_type": "input",
  "path": "input",
  "expected": null,
  "actual": null,
  "manual_ref": null,
  "fix_hints": [],
  "blocking": true
}
```

## Agent CLI

Use the shared entry point:

```bash
lammps-lsp-tool check path/to/input --format json
lammps-lsp-tool context path/to/input --format json
lammps-lsp-tool complete path/to/input --format json
lammps-lsp-tool hover path/to/input --format json
lammps-lsp-tool symbols path/to/input --format json
lammps-lsp-tool fix path/to/input --format json
```

`check` is wired to the existing repository diagnostics. The other operations
reserve stable JSON shapes so downstream agents can call every LSP family with
the same command surface while each repo fills in richer providers over time.
