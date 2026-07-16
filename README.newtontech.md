# newtontech/lammps-lsp

This repository is a public fork of `chappertron/lammps-analyser`, preserving
the upstream Rust CLI and LSP implementation for LAMMPS input scripts.

The authoritative package identity is `lammps-lsp@0.1.0-pre.4` from
`https://github.com/newtontech/lammps-lsp`. This prerelease candidate is
configured for verified cargo-dist GitHub Release artifacts and is not
published to crates.io.

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
lammps-lsp-tool check in.lmp --fail-on-blocking
```

Roadmap issues track formatter parity, OpenQC integration, golden diagnostics
fixtures, and MatMaster-specific LAMMPS workflow checks.
