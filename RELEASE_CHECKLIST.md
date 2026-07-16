# Release Checklist

This checklist is the provenance gate for the GitHub-only
`lammps-lsp@0.1.0-pre.4` prerelease. A checked source-tree gate does not mean a
release exists; tag and hosted-artifact gates stay unchecked until they are
verified against GitHub after merge.

## Candidate identity

- [x] `Cargo.toml`, `Cargo.lock`, `lsp-capabilities.json`, and
  `release/artifacts.json` agree on package `lammps-lsp`, version
  `0.1.0-pre.4`, and repository `newtontech/lammps-lsp`.
- [x] All four shipped commands report `0.1.0-pre.4` with `--version`.
- [x] `lammps-lsp-tool capabilities` reports candidate prerelease provenance
  both inside and outside a source checkout.
- [x] The changelog describes only changes since the latest repository tag,
  `0.1.0-pre-release-3`, without claiming the new tag is published.

## Raw, wiki, and docstring traceability

- [x] `raw/assets/manifest.json` is present and accepted by
  `make traceability-check`.
- [x] `raw/assets/lammps-upstream-sources.md` links the official LAMMPS sources
  used by the local raw collection.
- [x] `reports/docstring-wiki-raw-traceability.json` maps runtime docstrings and
  rule IDs to the raw manifest and local wiki pages.
- [x] Wiki links and required sections pass `make wiki-check`.
- [ ] Re-run both traceability gates on the final tag commit and retain their
  CI results with the release provenance.

## Runtime fixtures and package gates

- [x] The valid fixture succeeds with no blocking diagnostics.
- [x] The invalid fixture fails `--fail-on-blocking` with blocking diagnostics.
- [x] The log fixture emits `LAMMPS-E900`.
- [x] `make format`, `make lint`, `make typecheck`, `make test`, and `make check`
  pass on the candidate branch.
- [x] `cargo package --locked --allow-dirty` validates the source package.
- [x] `cargo dist plan` agrees with `release/artifacts.json`.
- [ ] Verify the clean merge commit with `make release-check` before tagging.

## GitHub Release artifacts

- [x] cargo-dist is pinned to `0.22.1` and publishing is limited to GitHub
  Releases.
- [x] The lock includes archives for Apple Silicon macOS, Intel macOS, Linux
  x86_64, and Windows x86_64, plus shell and PowerShell installers.
- [x] The release workflow runs `scripts/verify-release-artifacts.py` before
  upload, requiring exact asset names, checksums, documentation, and all four
  executables in every platform archive.
- [ ] Create annotated tag `v0.1.0-pre.4` only from the verified merge commit.
- [ ] Confirm the GitHub prerelease contains the exact locked artifact set and
  that installer and checksum smoke checks pass.
- [ ] Change `releaseProvenance.status` from `candidate` only in a follow-up that
  is backed by the hosted release evidence.

## OpenQC probe gate

- [ ] After the tag is hosted, refresh OpenQC's managed LAMMPS checkout and run
  the latest/probe workflow against `v0.1.0-pre.4`.
- [ ] Confirm the probe reports package `lammps-lsp`, version `0.1.0-pre.4`, and
  provenance without missing/stale-tag warnings.
- [ ] If the tag is missing or stale, OpenQC must keep the backend in candidate
  state, show the warning, and must not silently promote it to a mature runtime.

## Explicitly out of scope

- Publishing `lammps-lsp` to crates.io.
- Merging this pull request, creating the tag, or publishing the GitHub Release
  from the candidate branch.
