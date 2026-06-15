#!/usr/bin/env python3
"""Generate the OpenQC v1 docstring/wiki/raw traceability report."""

from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCHEMA_VERSION = "openqc.lsp.traceability.v1"
SERVER_ID = "lammps-lsp"
LANGUAGE_ID = "lammps"
REPOSITORY = "newtontech/lammps-lsp"
DEFAULT_BRANCH = "master"
RULE_PREFIX = "LAMMPS"
SOURCE_ROOTS = ["src"]
REPORT_PATH = Path("reports/docstring-wiki-raw-traceability.json")
RAW_MANIFEST_PATH = Path("raw/assets/manifest.json")

WIKI_PATH_RE = re.compile(r"wiki/[A-Za-z0-9_./-]+\.md")
RAW_PATH_RE = re.compile(r"raw/assets/[A-Za-z0-9_./-]+")
URL_RE = re.compile(r"https?://[^\s)\]>'\"`]+")
SHORT_RULE_RE = re.compile(r"\b" + re.escape(RULE_PREFIX) + r"-([EWI])(\d{3})\b")
OPENQC_RULE_RE = re.compile(r"\b[A-Z0-9]+-[A-Z0-9_]+-[A-Z0-9_]+-\d{3}\b")
SEVERITY_KIND = {"E": "ERROR", "W": "WARNING", "I": "INFO"}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def rel(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


def clean_path(value: str) -> str:
    return value.rstrip(".,;:)!?'\"")


def stable_generated_at() -> str:
    # Deterministic output keeps --check mode stable across repeated runs.
    return datetime.fromtimestamp(0, timezone.utc).isoformat()


def infer_symbol(path: Path, text: str) -> str:
    for pattern in (
        r"^class\s+(\w+)",
        r"^def\s+(\w+)",
        r"^pub\s+(?:async\s+)?fn\s+(\w+)",
        r"^pub\s+struct\s+(\w+)",
    ):
        match = re.search(pattern, text, re.MULTILINE)
        if match:
            return match.group(1)
    return path.stem


def source_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for source_root in SOURCE_ROOTS:
        base = root / source_root
        if not base.exists():
            continue
        for candidate in sorted(base.rglob("*")):
            if not candidate.is_file():
                continue
            if candidate.suffix not in {".py", ".rs", ".ts", ".js"}:
                continue
            if any(
                part in {"__pycache__", "node_modules", "target"}
                for part in candidate.parts
            ):
                continue
            files.append(candidate)
    return files


def collect_docstrings(root: Path) -> tuple[list[dict[str, str]], int]:
    entries: list[dict[str, str]] = []
    broken = 0
    seen: set[tuple[str, str, str]] = set()
    for path in source_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        symbol = infer_symbol(path, text)
        for match in WIKI_PATH_RE.findall(text):
            wiki_path = clean_path(match)
            if not (root / wiki_path).is_file():
                broken += 1
                continue
            key = (rel(path, root), wiki_path, symbol)
            if key in seen:
                continue
            seen.add(key)
            entries.append({"path": key[0], "wikiPath": key[1], "symbol": key[2]})
    return entries, broken


def source_url_near(text: str, raw_path: str) -> str | None:
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if raw_path not in line:
            continue
        window = "\n".join(lines[index : min(index + 4, len(lines))])
        urls = [clean_path(url) for url in URL_RE.findall(window)]
        if urls:
            return urls[0]
    return None


def collect_wiki_sources(root: Path) -> tuple[list[dict[str, str]], int]:
    wiki_dir = root / "wiki"
    entries: list[dict[str, str]] = []
    missing_raw = 0
    seen: set[tuple[str, str]] = set()
    if not wiki_dir.exists():
        return entries, missing_raw
    for path in sorted(wiki_dir.rglob("*.md")):
        text = path.read_text(encoding="utf-8", errors="replace")
        raw_refs = sorted({clean_path(item) for item in RAW_PATH_RE.findall(text)})
        for raw_path in raw_refs:
            raw_file = root / raw_path
            if raw_file.is_dir() or raw_path.endswith("/"):
                continue
            if not raw_file.is_file():
                missing_raw += 1
                continue
            key = (rel(path, root), raw_path)
            if key in seen:
                continue
            seen.add(key)
            entries.append(
                {
                    "wikiPath": key[0],
                    "rawPath": key[1],
                    "sourceUrl": source_url_near(text, raw_path)
                    or f"https://github.com/{REPOSITORY}/blob/{DEFAULT_BRANCH}/{raw_path}",
                }
            )
    return entries, missing_raw


def normalize_rule(code: str) -> str:
    short = SHORT_RULE_RE.fullmatch(code)
    if short:
        return f"{RULE_PREFIX}-INPUT-{SEVERITY_KIND[short.group(1)]}-{short.group(2)}"
    return code


def collect_rule_ids(root: Path) -> list[dict[str, str]]:
    entries: list[dict[str, str]] = []
    seen: set[tuple[str, str]] = set()
    for path in source_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        codes = set(OPENQC_RULE_RE.findall(text))
        codes.update(
            normalize_rule(match.group(0)) for match in SHORT_RULE_RE.finditer(text)
        )
        for code in sorted(codes):
            if not OPENQC_RULE_RE.fullmatch(code):
                continue
            key = (code, rel(path, root))
            if key in seen:
                continue
            seen.add(key)
            entries.append({"code": code, "sourcePath": key[1]})
    return entries


def ensure_raw_manifest(
    root: Path, wiki_sources: list[dict[str, str]], check: bool
) -> bool:
    manifest_path = root / RAW_MANIFEST_PATH
    if manifest_path.exists():
        return True
    if check:
        return False
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    unique = {entry["rawPath"]: entry["sourceUrl"] for entry in wiki_sources}
    payload = {
        "manifest_version": "1.0.0",
        "schema_version": "provenance-manifest-v1",
        "repository": REPOSITORY,
        "entries": [
            {
                "path": raw_path.removeprefix("raw/assets/"),
                "source_type": "raw_asset",
                "source_url": source_url,
                "stable_id": raw_path.replace("/", "-").replace(".", "-"),
            }
            for raw_path, source_url in sorted(unique.items())
        ],
    }
    manifest_path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return True


def build_report(root: Path, check: bool = False) -> dict[str, Any]:
    docstrings, broken_wiki = collect_docstrings(root)
    wiki_sources, missing_raw = collect_wiki_sources(root)
    manifest_ok = ensure_raw_manifest(root, wiki_sources, check=check)
    source_urls = [
        {"rawPath": item["rawPath"], "url": item["sourceUrl"]}
        for item in sorted(
            wiki_sources, key=lambda entry: (entry["rawPath"], entry["sourceUrl"])
        )
    ]
    report = {
        "schemaVersion": SCHEMA_VERSION,
        "serverId": SERVER_ID,
        "repository": REPOSITORY,
        "languageId": LANGUAGE_ID,
        "generatedAt": stable_generated_at(),
        "summary": {
            "docstringsTotal": len(docstrings),
            "docstringsLinked": len(docstrings),
            "brokenWikiLinks": broken_wiki,
            "wikiSourcesWithoutRaw": missing_raw,
            "rawManifestFailures": 0 if manifest_ok else 1,
        },
        "docstrings": docstrings,
        "wikiSources": wiki_sources,
        "ruleIds": collect_rule_ids(root),
        "sourceUrls": source_urls,
        "rawManifest": {"path": RAW_MANIFEST_PATH.as_posix(), "ok": manifest_ok},
    }
    return report


def write_report(report: dict[str, Any], root: Path) -> None:
    path = root / REPORT_PATH
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check", action="store_true", help="fail if the report would change"
    )
    args = parser.parse_args()
    root = repo_root()
    report = build_report(root, check=args.check)
    path = root / REPORT_PATH
    if args.check:
        if not path.exists():
            print(f"missing {REPORT_PATH}", file=sys.stderr)
            return 1
        current = json.loads(path.read_text(encoding="utf-8"))
        if current != report:
            print(
                f"{REPORT_PATH} is stale; run scripts/generate-traceability-report.py",
                file=sys.stderr,
            )
            return 1
        return 0
    write_report(report, root)
    print(f"wrote {REPORT_PATH}")
    print(json.dumps(report["summary"], sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
