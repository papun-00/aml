# Copilot instructions — alashore-marine

Repository-level rules for GitHub Copilot Chat / agents. Same content as `CLAUDE.md` / `AGENTS.md` / `GEMINI.md` / `.cursorrules` / `.windsurfrules` — kept in sync.

## On every prompt

- Read `STATE.md` and the current `BACKLOG.md` item before suggesting code.
- Follow the master skill stack at `/Volumes/hex/current-work/skills/` — see `.claude/SKILLS_REFERENCE.md`.
- Read the matching skill (`testing-strategy.md`, `tdd-discipline.md`, `threat-modeling.md`, etc.) before non-trivial work.

## Hard rules

1. Author = `cyfen-code <code@cyfen.co>`.
2. License = BSD-3-Clause.
3. TDD: failing test first.
4. Smoke gate before feature work (`scripts/smoke.sh`).
5. Threat-model gate for trust-boundary changes.
6. No secrets in source / logs / errors.
7. Local-first AI default.
8. Compile-time security where the language permits.
9. Defense in depth — ≥2 independent controls per high-impact threat.
10. Refactors preserve current product behaviour; additive where possible; small reversible PRs.

## Compaction discipline

- Before compaction: dump task / questions / next 3 steps to `STATE.md`.
- After resume: re-read `STATE.md` + current `BACKLOG.md` + last 3 `PROGRESS.md` entries before any action.
