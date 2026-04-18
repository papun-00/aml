# Alashore Marine Exports

Full-stack Rust web application for India's premier frozen seafood exporter.

## Stack

- **Frontend**: Dioxus 0.6 (Rust/WASM SPA)
- **Styling**: Tailwind CSS + IBM Carbon Design System (IBM Plex fonts, monochrome/blue spectrum)
- **Backend**: Axum 0.7 REST API with OWASP security hardening
- **Shared**: Common types crate for frontend/backend type safety
- **Tooling**: pnpm for concurrent dev commands

## Quick Start

Requires `pnpm`, `cargo`, and `dioxus-cli`.

```bash
pnpm install        # Tailwind, concurrently
pnpm dev            # Starts frontend + backend + Tailwind watcher
```

**Before deploying:** Run `pnpm ci:local` to test locally (saves GitHub Actions costs)

**Troubleshooting?** See [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for common issues and solutions.

## Architecture Overview

```
alashore-marine/
  frontend/        # Dioxus 0.6 SPA
    src/
      config.rs    # Single source of truth for all data
      components/  # Reusable: ProductCard, StatCounter, CertBadge, InquiryCta
      pages/       # Route page components
  backend/         # Axum 0.7 API
    src/
      handlers/    # Route handlers (inquiry, newsletter, health, etc.)
      middleware/  # Security headers, input validation/sanitization
      lib.rs       # Re-exports for integration test access
    tests/         # Integration tests (api_tests.rs)
  shared/          # Shared types (InquiryPayload, NewsletterPayload, etc.)
```

## Config-Driven Design

All content lives in `frontend/src/config.rs`:
- Product catalog (6 products with HS codes, certs, categories)
- Company stats, certifications, FAQs, navigation items, company info
- To add a product: add a `ProductConfig` entry to `all_products()` in config.rs

## Security (OWASP)

- Security headers middleware (CSP, X-Frame-Options, X-Content-Type-Options, etc.)
- Input validation: regex email check, HTML entity encoding, field length limits
- CORS restriction to allowed origins
- 1 MB request body size limit

## Testing

```bash
cargo test --workspace    # 43 tests across all crates
cargo clippy --workspace  # Lint
```

Test breakdown: 9 backend unit (validation), 10 backend integration (API endpoints), 8 frontend config, 5 shared types, 2 SSR landing page.

## SEO & AEO Strategy

- Schema.org JSON-LD injections (Organization, Product, FAQPage)
- Semantic HTML for LLM extraction
- `/llms.txt` route for AI crawlers

## TDD Protocol

1. Write tests for new behavior first.
2. Implement Rust logic.
3. Verify UI matches Carbon Design tokens.
4. Run `pnpm ci:quick` before committing (auto-runs via pre-commit hook).
5. Run `pnpm ci:local` before deploying to production.

For AI agents, see `CLAUDE.md` for development guidance.

## Cost-Optimized CI/CD

**GitHub Actions runs manually only** to save billing costs. All checks run locally:

```bash
pnpm ci:local   # Full CI pipeline (runs everything GitHub Actions would)
pnpm ci:quick   # Fast checks (format, lint, test)
pnpm precommit  # Pre-commit checks (automatic on git commit)
```

**Deploy to production:** Trigger workflow manually at https://github.com/cyfen-code/alashore-marine-pvt-ltd/actions

See [docs/COST_OPTIMIZATION.md](docs/COST_OPTIMIZATION.md) for details.
