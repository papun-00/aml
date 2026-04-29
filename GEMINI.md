# GEMINI.md — alashore-marine

Repository-level operational rules for the Gemini CLI agent. Mirrors `CLAUDE.md` and `AGENTS.md` — same content, different filename for different harnesses.

## On resume

1. Read `STATE.md`.
2. Read the current `BACKLOG.md` item.
3. Read the last 3 entries of `PROGRESS.md`.
4. Read `.claude/SKILLS_REFERENCE.md` and the relevant skill at `/Volumes/hex/current-work/skills/`.
5. Restate understanding before any action.

## Hard rules

1. Author = `cyfen-code <code@cyfen.co>` for all commits.
2. License = BSD-3-Clause by default.
3. Smoke gate before feature work; TDD discipline for implementation.
4. Threat-model gate for trust-boundary changes.
5. State updates after every meaningful step.
6. No secrets in source / logs / errors.
7. Local-first AI by default.
8. Compile-time security where the language permits.
9. Defense in depth — ≥2 independent controls per high-impact threat.

## Productive-refactor rule

Refactors must preserve current behaviour, be additive where possible, ship in small reversible PRs, and carry proof-of-equivalence tests during the transition.

## Skills

Master skill stack: `/Volumes/hex/current-work/skills/`. Catalog: `.claude/SKILLS_REFERENCE.md`. Read the relevant skill before acting on any non-trivial task.
