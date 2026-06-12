.PHONY: install format lint typecheck test check cleanup-merged

install:
	bash scripts/install.sh

format:
	bash scripts/format.sh

lint:
	bash scripts/lint.sh

typecheck:
	bash scripts/typecheck.sh

test:
	bash scripts/test.sh

check: lint typecheck test

cleanup-merged:
	bash scripts/cleanup_merged_worktrees.sh
