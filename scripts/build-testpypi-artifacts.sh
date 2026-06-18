#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "$ROOT_DIR"

rm -rf dist
mkdir -p dist

if [[ -f pyproject.toml ]] && grep -q 'build-backend = "maturin"' pyproject.toml; then
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
  uvx maturin build --release --out dist
  uvx maturin sdist --out dist
else
  if [[ -f package.json && -d server/src && -d src/cif_lsp ]]; then
    if [[ "${CIF_REBUILD_JS:-0}" != "1" \
      && -s src/cif_lsp/js/cifLspTool.cjs \
      && -s src/cif_lsp/js/server.cjs ]]; then
      echo "Using existing bundled CIF JavaScript files."
    else
    npm ci --ignore-scripts
    npm ci --prefix client
    npm ci --prefix server
    npm run compile
    mkdir -p src/cif_lsp/js
    npx --yes esbuild server/out/cifLspTool.js \
      --bundle \
      --platform=node \
      --format=cjs \
      --outfile=src/cif_lsp/js/cifLspTool.cjs
    npx --yes esbuild server/out/server.js \
      --bundle \
      --platform=node \
      --format=cjs \
      --banner:js='#!/usr/bin/env node' \
      --outfile=src/cif_lsp/js/server.cjs
    chmod 755 src/cif_lsp/js/cifLspTool.cjs
    fi
  fi
  uvx --from build python -m build --outdir dist
fi

uvx twine check dist/*

"${PYTHON:-python3}" - <<'PY'
from pathlib import Path
import sys
import tarfile
import zipfile

dist = Path("dist")
wheels = sorted(dist.glob("*.whl"))
sdists = sorted(dist.glob("*.tar.gz"))
if not wheels or not sdists:
    raise SystemExit("dist must contain at least one wheel and one sdist")

for wheel in wheels:
    with zipfile.ZipFile(wheel) as archive:
        names = archive.namelist()
    has_skill = any(name.endswith("skill/skill.yaml") for name in names)
    has_embedded_skill_cli = any(
        name.endswith(".data/scripts/lammps-lsp-tool")
        or name.endswith("/lammps-lsp-tool")
        for name in names
    )
    if not has_skill and not has_embedded_skill_cli:
        raise SystemExit(f"{wheel} does not expose skill metadata")

for sdist in sdists:
    with tarfile.open(sdist) as archive:
        names = archive.getnames()
    if not any(name.endswith("/skill/skill.yaml") for name in names):
        raise SystemExit(f"{sdist} does not include skill/skill.yaml")

print("dist artifact checks passed", file=sys.stderr)
PY
