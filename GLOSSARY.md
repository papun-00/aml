# GLOSSARY — alashore-marine

Domain terms used in this repository. Add a term whenever an agent or human uses jargon that is not industry-standard. Keeps cross-session interpretation stable so agents do not drift on naming after compaction.

| Term | Definition | First appearance |
|------|------------|------------------|
| _example_ | _short, unambiguous definition_ | _file:line or PR reference_ |

---

## Workspace-wide terms (inherited from master skills)

| Term | Definition |
|------|------------|
| Skill stack | The 13 skill files at `/Volumes/hex/current-work/skills/` |
| Smoke gate | Mandatory `scripts/smoke.sh` pass before any feature work (`testing-strategy.md` §3) |
| Threat-model gate | Threat-model entry must exist for any work touching trust boundaries (`threat-modeling.md`) |
| Iron Law (TDD) | Failing test before implementation; red → green → refactor (`tdd-discipline.md`) |
| Productive refactor | Refactor that preserves current behavior, lands in small reversible steps, with proof-of-equivalence tests |
| Compaction | Auto-summarisation of conversation context; defended against by externalising state to `STATE.md` |
| Resume | Post-compaction recovery via `/resume` slash command |
| MAESTRO + ATLAS | AI-specific threat-model frameworks (see `threat-modeling.md`) |
| LLM Top 10 | OWASP LLM Top 10 — must be considered for any AI-touching feature |
