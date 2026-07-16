.PHONY: install format lint typecheck test check release-check cleanup-merged traceability-report traceability-check

install:
	bash scripts/install.sh

format:
	bash scripts/format.sh

lint:
	bash scripts/lint.sh

typecheck:
	bash scripts/typecheck.sh

test:
	cargo test --all-features

traceability-report:
	bash scripts/generate-traceability-report.sh

traceability-check:
	bash scripts/generate-traceability-report.sh --check

wiki-check:
	bash scripts/check-llm-wiki.sh

check: lint typecheck test traceability-check wiki-check

release-check: check
	python3 scripts/verify-release-artifacts.py --contract-only
	cargo package --locked --allow-dirty
	cargo dist plan

cleanup-merged:
	bash scripts/cleanup_merged_worktrees.sh
