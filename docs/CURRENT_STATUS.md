# Alashore Marine: Current Project Status

## 1. Project Architecture
The project is a full-stack Rust application using a Cargo workspace:
- **Frontend:** Dioxus 0.6 (Rust/WASM SPA). Located in `/frontend`.
- **Backend:** Axum 0.7 REST API with OWASP security hardening. Located in `/backend`.
- **Shared:** Shared types (models, schemas) in `/shared` guarantee type uniformity across the network boundary.

### Config-Driven Architecture
All product catalog data, company stats, certifications, FAQs, navigation, and company info are defined in a single file: `frontend/src/config.rs`. This eliminates scattered hardcoded values and makes content updates a one-file change.

### Reusable Components
Four reusable components power the UI:
- **ProductCard** (`product_card.rs`) -- renders any `ProductConfig` with category badge, certs, and HS code.
- **StatCounter** (`stat_counter.rs`) -- displays a `StatItem` with label/value/description.
- **CertBadge** (`cert_badge.rs`) -- renders a `CertConfig` with icon.
- **InquiryCta** (`inquiry_cta.rs`) -- call-to-action block linking to the inquiry form.

## 2. UI / Design System Status
- **Styling Strategy:** TailwindCSS utility classes customized to implement the **IBM Carbon Design System**.
- **Carbon Tokens:** `frontend/assets/css/main.css` defines `@layer components` matching Carbon spacing, colors, and typography (Expressive Theme, Dark High Contrast headers).
- **Route Naming:** All Route enum variants use the `XxxPage {}` naming convention (e.g., `HomePage {}`, `ProductsPage {}`), required by the Dioxus 0.6 Router derive macro.

## 3. Security Hardening (OWASP)
The backend implements OWASP-aligned security measures:
- **Security headers middleware:** X-Content-Type-Options, X-Frame-Options, X-XSS-Protection, Referrer-Policy, Permissions-Policy, CSP.
- **Input validation:** Regex-based email validation, HTML entity encoding (sanitization), field length limits (truncation).
- **CORS restriction:** Configured for allowed origins only.
- **Body size limit:** 1 MB max request body.

## 4. SEO & GenAI Strategy (AEO)
- **Semantic HTML:** Native `<dl>`, `<table>`, and accessible elements enforced.
- **Structured Data:** JSON-LD schemas (Organization, Product, FAQPage) injected via the routing engine.
- **LLMs.txt:** `/llms.txt` route for AI web scrapers.

## 5. Testing
All 43 tests pass (`cargo test --workspace`):

| Category | Count | Location |
|---|---|---|
| Backend unit (validation) | 9 | `backend/src/middleware/validate.rs` |
| Backend integration (API) | 10 | `backend/tests/api_tests.rs` |
| Frontend config | 8 | `frontend/tests/config_tests.rs` |
| Shared types | 5 | `frontend/tests/shared_types_tests.rs` |
| SSR landing page | 2 | `frontend/tests/landing_page_tests.rs` |

Note: `backend/src/lib.rs` re-exports modules so integration tests can access handlers and middleware.

## 6. Development Workflow
- `pnpm dev`: Runs Dioxus frontend, Tailwind CSS watch, and Axum backend concurrently.
- `cargo test --workspace`: Runs all tests across all crates.
- `cargo clippy --workspace`: Lint check.
- TDD is required: write tests before implementing features.
