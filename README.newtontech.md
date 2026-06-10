# newtontech/lammps-lsp

This repository is a public fork of `chappertron/lammps-analyser`, preserving
the upstream Rust CLI and LSP implementation for LAMMPS input scripts.

## Public Interface

The upstream server binary is still available as:

```bash
lmp-lsp
```

This fork also builds an alias with the naming used across the newtontech LSP
family:

```bash
lammps-lsp
lammps-analyser in.lmp
```

Roadmap issues track formatter parity, OpenQC integration, golden diagnostics
fixtures, and MatMaster-specific LAMMPS workflow checks.
