#!/usr/bin/env python3
"""Verify the locked lammps-lsp cargo-dist release contract and artifacts."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tarfile
import tomllib
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LOCK_PATH = ROOT / "release" / "artifacts.json"


def fail(message: str) -> None:
    raise ValueError(message)


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def verify_contract(lock: dict) -> None:
    package = lock["package"]
    cargo_dist = lock["cargoDist"]
    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    capabilities = load_json(ROOT / "lsp-capabilities.json")
    workflow = (ROOT / ".github" / "workflows" / "release.yml").read_text(
        encoding="utf-8"
    )

    expected_package = {
        "name": package["name"],
        "version": package["version"],
        "repository": package["repository"],
    }
    actual_package = {key: cargo["package"].get(key) for key in expected_package}
    if actual_package != expected_package:
        fail(
            f"Cargo package metadata does not match artifact lock: "
            f"expected {expected_package}, got {actual_package}"
        )

    dist = cargo["workspace"]["metadata"]["dist"]
    expected_targets = [target["triple"] for target in lock["targets"]]
    if dist.get("cargo-dist-version") != cargo_dist["version"]:
        fail("cargo-dist version does not match artifact lock")
    if dist.get("targets") != expected_targets:
        fail("cargo-dist targets do not match artifact lock")
    if dist.get("installers") != cargo_dist["installers"]:
        fail("cargo-dist installers do not match artifact lock")
    if cargo_dist["publish"] != "github-release-only":
        fail("artifact lock must remain GitHub Release only")
    if package.get("tag") != f"v{package['version']}":
        fail("artifact lock tag must be v followed by the package version")

    capability_contract = {
        "repository": capabilities.get("repository"),
        "packageName": capabilities.get("packageName"),
        "packageVersion": capabilities.get("packageVersion"),
    }
    expected_capabilities = {
        "repository": "newtontech/lammps-lsp",
        "packageName": package["name"],
        "packageVersion": package["version"],
    }
    if capability_contract != expected_capabilities:
        fail("capabilities release metadata does not match artifact lock")
    if capabilities.get("releaseProvenance", {}).get("artifactLock") != str(
        LOCK_PATH.relative_to(ROOT)
    ):
        fail("capabilities artifactLock does not point to the locked manifest")
    release_provenance = capabilities.get("releaseProvenance", {})
    if release_provenance.get("status") != "candidate":
        fail("unpublished release provenance must remain candidate")
    if release_provenance.get("channel") != "prerelease":
        fail("release provenance channel must remain prerelease")

    installer_url = f"v{cargo_dist['version']}/cargo-dist-installer.sh"
    if installer_url not in workflow:
        fail("release workflow does not install the locked cargo-dist version")
    if "python3 scripts/verify-release-artifacts.py artifacts" not in workflow:
        fail("release workflow does not verify gathered artifacts")
    if "cargo publish" in workflow:
        fail("release workflow must not publish to crates.io")


def expected_assets(lock: dict) -> set[str]:
    assets = set(lock["globalAssets"])
    for target in lock["targets"]:
        archive = target["archive"]
        assets.add(archive)
        assets.add(f"{archive}.sha256")
    return assets


def verify_checksum(archive: Path, checksum_path: Path) -> None:
    fields = checksum_path.read_text(encoding="utf-8").strip().split()
    if not fields:
        fail(f"empty checksum file: {checksum_path.name}")
    expected = fields[0].lower()
    actual = hashlib.sha256(archive.read_bytes()).hexdigest()
    if actual != expected:
        fail(
            f"checksum mismatch for {archive.name}: expected {expected}, got {actual}"
        )


def archive_members(path: Path) -> set[str]:
    if path.suffix == ".zip":
        with zipfile.ZipFile(path) as archive:
            return {
                Path(name).name
                for name in archive.namelist()
                if not name.endswith("/")
            }
    with tarfile.open(path, mode="r:xz") as archive:
        return {Path(member.name).name for member in archive if member.isfile()}


def verify_archive(path: Path, binaries: list[str]) -> None:
    members = archive_members(path)
    is_windows = path.suffix == ".zip"
    expected_binaries = {
        f"{binary}.exe" if is_windows else binary for binary in binaries
    }
    missing_binaries = expected_binaries - members
    if missing_binaries:
        fail(
            f"{path.name} is missing binaries: "
            f"{', '.join(sorted(missing_binaries))}"
        )
    missing_docs = {"README.md", "CHANGELOG.md", "LICENSE"} - members
    if missing_docs:
        fail(
            f"{path.name} is missing release documents: "
            f"{', '.join(sorted(missing_docs))}"
        )


def verify_artifacts(lock: dict, artifact_dir: Path) -> None:
    if not artifact_dir.is_dir():
        fail(f"artifact directory does not exist: {artifact_dir}")
    expected = expected_assets(lock)
    actual = {path.name for path in artifact_dir.iterdir() if path.is_file()}
    missing = expected - actual
    if missing:
        fail(f"missing release assets: {', '.join(sorted(missing))}")
    unexpected = actual - expected
    if unexpected:
        fail(f"unexpected release assets: {', '.join(sorted(unexpected))}")

    for target in lock["targets"]:
        archive = artifact_dir / target["archive"]
        verify_checksum(archive, artifact_dir / f"{archive.name}.sha256")
        verify_archive(archive, lock["binaries"])
    source = artifact_dir / "source.tar.gz"
    verify_checksum(source, artifact_dir / "source.tar.gz.sha256")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_dir", nargs="?", type=Path)
    parser.add_argument("--contract-only", action="store_true")
    args = parser.parse_args()
    if not args.contract_only and args.artifact_dir is None:
        parser.error("artifact_dir is required unless --contract-only is used")
    return args


def main() -> int:
    args = parse_args()
    try:
        lock = load_json(LOCK_PATH)
        verify_contract(lock)
        if not args.contract_only:
            verify_artifacts(lock, args.artifact_dir.resolve())
    except (
        KeyError,
        OSError,
        ValueError,
        json.JSONDecodeError,
        tomllib.TOMLDecodeError,
    ) as error:
        print(f"release verification failed: {error}", file=sys.stderr)
        return 1

    target_summary = ", ".join(target["triple"] for target in lock["targets"])
    mode = "contract" if args.contract_only else "artifacts"
    print(
        f"Verified {lock['package']['name']}@{lock['package']['version']} "
        f"{mode}: {target_summary}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
