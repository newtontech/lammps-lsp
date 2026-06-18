# TestPyPI release workflow

This repository publishes prerelease artifacts to TestPyPI only. Do not use this
workflow for production PyPI releases.

## Local build

```bash
scripts/build-testpypi-artifacts.sh
```

The build script writes `dist/`, runs package-specific build steps, validates the
wheel and sdist with `twine check`, and verifies that the pluggable `skill/`
metadata is present in the distribution artifacts. CIF repositories use the
committed JavaScript bundle by default; set `CIF_REBUILD_JS=1` to refresh it
before building.

## Local TestPyPI upload

Create a TestPyPI API token outside the repository, then run:

```bash
export TWINE_USERNAME=__token__
export TWINE_PASSWORD='pypi-...'
scripts/upload-testpypi.sh
```

The script defaults to `https://test.pypi.org/legacy/`. Override
`TWINE_REPOSITORY_URL` only for a compatible private index.

## GitHub tag release

The workflow `.github/workflows/testpypi-release.yml` builds on every
`testpypi-v*` tag. The tag version must match the package metadata.

For the current prerelease:

```bash
git tag testpypi-v0.1.0rc1
git push origin testpypi-v0.1.0rc1
```

The workflow can publish with either:

- `TEST_PYPI_API_TOKEN` repository secret, using a TestPyPI token; or
- TestPyPI Trusted Publishing configured for this repository, the
  `testpypi-release.yml` workflow, and the `testpypi` GitHub environment.

Manual runs are supported from the GitHub Actions tab. Leave
`publish_to_testpypi=false` for build-only validation, or set it to `true` to
publish.

## Test install

After publishing:

```bash
python -m venv /tmp/lammps-lsp-testpypi
/tmp/lammps-lsp-testpypi/bin/python -m pip install --upgrade pip
/tmp/lammps-lsp-testpypi/bin/pip install \
  --index-url https://test.pypi.org/simple/ \
  --extra-index-url https://pypi.org/simple/ \
  lammps-lsp==0.1.0rc1
```
