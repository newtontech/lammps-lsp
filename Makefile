.PHONY: install format lint typecheck test check cleanup-merged traceability-report traceability-check

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

cleanup-merged:
	bash scripts/cleanup_merged_worktrees.sh
