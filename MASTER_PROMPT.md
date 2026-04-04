# Alashore Marine Exports — Autonomous Development Master Prompt

**To the AI Agent reading this prompt:** You are taking over the ongoing development of the Alashore Marine Exports platform.

## 1. Project Context
- **Stack**: Dioxus 7 (Rust Frontend), Axum (Rust Backend), Node.js (Tooling wrapper `pnpm dev`).
- **UI/UX Framework**: Tailwinds configured tightly to the **IBM Carbon Design System**. This means:
  - Typography: IBM Plex Sans & IBM Plex Mono only.
  - Geometry: Sharp structures, 0px border radiuses, grid-aligned layouts.
  - Colors: High contrast (Carbon Blue 60/70, Carbon Gray 10/80/90/100).
- **SEO / AEO Focus**: The frontend enforces dense Schema.org JSON-LD elements and AI-optimized extraction paragraphs (Answer Engine Optimization).

## 2. Mandatory Workflow (Strict TDD)
You must follow this sequence for any new feature or fix:
1. **Analysis & Diagnostics**: Run `cargo check`, `cargo clippy`, and `pnpm run build:tailwind` to ensure the current state is error-free. Resolve any existing bugs before proceeding.
2. **Test-Driven Development (TDD) Phase**: Write your Rust unit tests and integration tests *first* in the relevant `mod tests` or `tests/` folders.
3. **Execution**: Implement the actual feature/logic until the tests pass.
4. **Validation**: Verify UI updates adhere to Carbon Design principles.
5. **Documentation**: Document all changes in the `README.md` or a `walkthrough.md`.

## 3. Immediate Mission (Trigger Command)
1. Read `package.json`, `Cargo.toml`, and the existing `frontend/src/` hierarchy to re-orient yourself.
2. Run standard diagnostic checks.
3. Await the specific user requirement, and immediately draft the testing architecture before writing Dioxus/Axum logic.

**Remember:** Maintain the "Fortune-500" aesthetic and prioritize uncompromising biological/corporate accuracy (e.g., maintaining the true shrimp crest without forcing monograms).
