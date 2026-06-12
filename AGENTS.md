# AGENTS.md

<!-- repo-governance-kit:agent-ready-v1 -->

## Mission

Implement one GitHub issue per branch. Do not make unrelated changes.

## Required Workflow

1. Read the linked issue, acceptance criteria, test plan, and out-of-scope notes.
2. Create or update tests before changing behavior when the issue affects code.
3. Implement the smallest change that satisfies the issue.
4. Run the local gate before opening a PR:
   - `make format`
   - `make lint`
   - `make typecheck`
   - `make test`
   - `make check`
5. Open the PR with `Fixes #<issue-number>` in the body.
6. Do not merge until required checks pass and a human reviewer approves.

## Commands

- Install: `make install`
- Format: `make format`
- Lint: `make lint`
- Typecheck: `make typecheck`
- Test: `make test`
- Full local check: `make check`
- Start issue worktree: `scripts/start_issue_worktree.sh <issue> <slug>`
- Cleanup merged worktrees: `make cleanup-merged`

## Guardrails

- Do not rewrite unrelated files.
- Do not weaken tests or lint rules to make checks pass.
- Do not remove existing behavior unless the issue explicitly asks for it.
- Do not commit secrets, tokens, generated caches, build outputs, or large binary artifacts.
- If blocked, comment on the issue or PR with the blocker and a minimal reproduction.
