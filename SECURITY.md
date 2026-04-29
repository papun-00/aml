# Security policy — alashore-marine

## Reporting a vulnerability

Please report security issues privately:
- Email: `security@cyfen.co`
- Or open a GitHub private security advisory on this repository.

We acknowledge receipt within **72 hours** and aim to patch high-severity issues within **30 days**. Public disclosure is coordinated with the reporter; default embargo is 90 days.

## Supported versions

| Version | Supported |
|---------|-----------|
| `main` (latest) | yes |
| previous tagged release | best-effort |
| older releases | no |

## Security posture

This repository follows the workspace-wide security skill stack at `/Volumes/hex/current-work/skills/`:

- `threat-modeling.md` — STRIDE / LINDDUN / MAESTRO / ATLAS as applicable
- `red-teaming.md` — adversary emulation, AI red-team
- `blue-teaming.md` — detection engineering, IR
- `offensive-security.md` — fuzzing, exploit dev, OWASP testing
- `defensive-security.md` — secure SDLC, hardening, PQC, supply-chain

## Findings tracking

| Folder | Purpose |
|--------|---------|
| `security/findings/open/` | Unresolved findings: severity, repro, fix plan, owner |
| `security/findings/remediated/` | Closed with proof-of-fix commit + regression test ID |
| `security/findings/accepted-risk/` | Documented why not fixed |

Every remediated finding gets a regression test in `tests/security/regression/` to prevent re-introduction.

## Threat model

System-level + per-component models live under `security/threat-model/`. Re-run when architecture changes.

## SBOM

Software bill of materials regenerated on dependency change; current snapshot in `security/sbom/current.spdx.json`.

## Dependency hygiene

- Rust: `cargo deny check`, `cargo audit` in CI
- Node: `pnpm audit` / `npm audit` + `trivy` SBOM scan
- Python: `pip-audit` + `safety check`
- Containers: `trivy image` on every build

## Secret hygiene

- `.gitleaks.toml` configured at repo root.
- Pre-commit `gitleaks protect` (where configured).
- Server-side `gitleaks scan` in CI.
- Never commit `.env`, tokens, API keys, or credentials.

## Hard floor

`PrivacyLevel::Strict` is the floor: local-first AI by default; cloud only on explicit opt-in with data-classification justification.
