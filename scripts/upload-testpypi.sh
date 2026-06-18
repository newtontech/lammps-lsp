#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "$ROOT_DIR"

if ! compgen -G "dist/*" >/dev/null; then
  scripts/build-testpypi-artifacts.sh
fi

if [[ "${TWINE_USERNAME:-}" == "" || "${TWINE_PASSWORD:-}" == "" ]]; then
  cat >&2 <<'EOF'
Missing local TestPyPI credentials.

Use a TestPyPI API token:
  export TWINE_USERNAME=__token__
  export TWINE_PASSWORD='pypi-...'

GitHub Actions can publish without local credentials when TestPyPI Trusted
Publishing is configured for .github/workflows/testpypi-release.yml.
EOF
  exit 2
fi

uvx twine upload \
  --repository-url "${TWINE_REPOSITORY_URL:-https://test.pypi.org/legacy/}" \
  "$@" \
  dist/*
