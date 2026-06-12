#!/usr/bin/env bash
# Lightweight LLM wiki link and manifest sanity check.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail=0

require_path() {
  if [[ ! -e "$1" ]]; then
    echo "missing required path: $1" >&2
    fail=1
  fi
}

require_path index.md
require_path log.md
require_path lsp-capabilities.json
require_path docs/LLM-WIKI-PLAN.md

for dir in raw/assets wiki/entities wiki/concepts wiki/synthesis; do
  require_path "$dir"
done

if command -v python3 >/dev/null 2>&1; then
  python3 - <<'PY'
import json
import re
from pathlib import Path

root = Path(".")
caps = json.loads((root / "lsp-capabilities.json").read_text(encoding="utf-8"))
wiki_paths = caps.get("wikiPaths", {})
for key, rel in wiki_paths.items():
    path = root / rel
    if not path.exists():
        raise SystemExit(f"lsp-capabilities wikiPaths.{key} missing: {rel}")

index = (root / "index.md").read_text(encoding="utf-8")
links = re.findall(r"\]\((wiki/[^)#]+)\)", index)
missing = [link for link in links if not (root / link).exists()]
if missing:
    raise SystemExit("index.md broken wiki links: " + ", ".join(sorted(set(missing))))

print(f"llm-wiki check ok ({len(links)} index wiki links, {len(wiki_paths)} capability paths)")
PY
else
  echo "python3 unavailable; skipped JSON/link validation" >&2
fi

exit "$fail"
