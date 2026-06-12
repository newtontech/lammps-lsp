# LAMMPS upstream source manifest

Concise index of official documentation used by the LLM wiki and LSP hover/diagnostics.
Do not mirror full manuals here — follow links for authoritative content.

| Label | Kind | URL | Local digest |
|-------|------|-----|--------------|
| LAMMPS manual (30 Mar 2026) | official_docs | https://docs.lammps.org/Manual.html | `raw/assets/lammps_docs_md/` |
| Howto guides | official_tutorial | https://docs.lammps.org/Howto.html | `raw/assets/lammps-advanced-tutorials.md` |
| Example inputs | official_examples | https://github.com/lammps/lammps/tree/develop/examples | `raw/assets/examples/` |
| Python interface | official_docs | https://docs.lammps.org/Python.html | `raw/assets/lammps-python-api.md` |
| ML potentials (PACE, SNAP, ML-IAP) | official_docs | https://docs.lammps.org/Pace.html | `raw/assets/lammps-ml-potentials.md` |
| Plugin mechanism | official_docs | https://docs.lammps.org/Plugin.html | `raw/assets/lammps-plugins.md` |
| RHEO package | official_docs | https://docs.lammps.org/Howto_rheo.html | `raw/assets/lammps-rheo-package.md` |
| Recent changelog | official_changelog | https://docs.lammps.org/changelog.html | `raw/assets/lammps-new-features-2024-2025.md` |

## Example inputs in this repo

- `raw/assets/examples/in.melt` — Lennard-Jones melt (`examples/melt/in.melt` upstream)
- `tests/fixtures/valid/minimal.in` — minimal metal/EAM smoke input for LSP validation

## Wiki cross-links

- Workflow: `wiki/synthesis/Input_Script_Workflow.md`
- Agent context: `wiki/synthesis/openqc-agent-context.md`
- Diagnostic engine: `wiki/concepts/diagnostic-engine-v1.md`
