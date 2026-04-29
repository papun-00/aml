# DECISIONS — alashore-marine

Append-only architectural / design / process decision log. ADR style: one entry per decision, never delete or edit prior entries (supersede instead).

## Format

```
## DEC-<YYYYMMDD>-<NN> — <short title>
- Decided: <YYYY-MM-DD>
- Status: proposed | accepted | superseded by DEC-<id>
- Context: <forces / problem>
- Decision: <what was decided>
- Consequences: <pros / cons / follow-ups>
- Tags: <e.g. security, architecture, license>
```

---

## DEC-20260429-01 — adopt workspace skill stack as authoritative

- Decided: 2026-04-29
- Status: accepted
- Context: this repo lacked the autonomous-iteration scaffolding defined in `/Volumes/hex/current-work/skills/scafolding.md`. Without it, agents cannot resume cleanly after compaction and security/test discipline is uneven.
- Decision: adopt the master skill stack at `/Volumes/hex/current-work/skills/` as the single source of truth for this repo. Reference, do not copy.
- Consequences:
  - Agents must read `.claude/SKILLS_REFERENCE.md` and the relevant skill before acting.
  - Cross-harness portability: `AGENTS.md`, `GEMINI.md`, `.cursorrules`, `.windsurfrules`, `.github/copilot-instructions.md` mirror `CLAUDE.md`.
  - Default license: BSD-3-Clause (`LICENSE` added if absent; existing licenses preserved unless explicitly relicensed).
- Tags: process, license, skills
