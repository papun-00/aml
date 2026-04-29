# BACKLOG — alashore-marine

Prioritized task queue. Each task has machine-verifiable acceptance criteria + security/test requirements. Pick the highest-priority unblocked item.

## Format

```
### TASK-<NNN> — <short title>
- Priority: P0 | P1 | P2 | P3
- Status: pending | in-progress | blocked | done
- Owner: <agent name | human | unassigned>
- Depends on: <task-ids or "none">
- Acceptance criteria (machine-verifiable):
  - [ ] <test id / metric threshold>
- Security / test requirements:
  - [ ] threat-model entry: yes | no | N/A (justification if N/A)
  - [ ] regression test in `tests/security/regression/`: yes | no | N/A
  - [ ] secrets scan clean: yes | pending
- Notes:
```

---

_(no tasks yet — owner adds the first task here)_
