# CLAUDE.md — Alashore Marine Exports

## Autonomous Protocol

When triggered with **"Continue building Alashore Marine. Follow CLAUDE.md autonomous protocol."**, execute this loop:

```
1. ASSESS    → cargo check && cargo clippy && cargo test --workspace
2. PICK      → Read the Task Backlog below. Pick the highest-priority unblocked task.
3. TDD       → Write failing tests FIRST for the chosen task.
4. IMPLEMENT → Write code until tests pass. Follow Design Principles strictly.
5. VERIFY    → cargo test --workspace && cargo clippy (zero warnings, zero failures).
6. DOCUMENT  → Update CLAUDE.md (mark task done, update test count, add new how-to if needed).
               Update docs/CURRENT_STATUS.md if architecture changed.
7. LOOP      → Go to step 2. Pick next unblocked task. Continue until user interrupts or all tasks done.
```

**Rules:**
- Never skip TDD. Tests first, always.
- Never break existing tests. If a change causes a regression, fix it before moving on.
- Never hardcode data in pages. Config-driven only (`frontend/src/config.rs`).
- All new components must be reusable with Props.
- All new endpoints must have integration tests.
- OWASP security on all user inputs.
- **Code coverage must be >95% for both backend and frontend.** Measure and maintain this bar on every change.
- Update this file's test count and backlog after each task.

---

## Task Backlog (Priority Order)

Pick the **first unblocked** task. Mark `[x]` when done.

### Phase 1 — Backend Foundation
- [x] **Database persistence for products** — Products table in SQLite, seeded on startup, handlers query DB. JSON columns for specs/certs/markets. 3 new integration tests.
- [x] **Email sending on inquiry** — Trait-based EmailService (SmtpEmailService prod, MockEmailService test). `lettre` SMTP. Fire-and-forget via tokio::spawn. 4 new tests (3 unit + 1 integration).
- [x] **Rate limiting** — Global rate limiter via `governor` crate. Configurable RPM from env. Returns 429 + Retry-After header. 3 unit tests + 1 integration test.
- [x] **Request logging & audit trail** — `audit_log` table with auto-increment ID. Inquiry handler logs on success. GET /api/v1/admin/audit returns last 100 entries. 3 integration tests.

### Phase 2 — Frontend Enhancement
- [x] **Frontend API integration** — `utils/api.rs` client module with `submit_inquiry()` and `subscribe_newsletter()`. Inquiry form wired to real backend with async submit, error display, and 429 handling.
- [x] **Product images** — `image_url` field added to ProductConfig. ProductCard renders `<img>` with lazy loading. Paths point to `/assets/images/*.webp`.
- [x] **Loading states & error handling** — Reusable `Loading` and `ErrorDisplay` components. CSS spinner animation. Inquiry form shows real API errors.
- [x] **Dark/light theme toggle** — Carbon Design SVG icons (Light/Asleep). Persists in localStorage. Soft blue palette (#B8D4E8).
- [ ] **Accessibility audit** — Ensure WCAG 2.1 AA compliance. Keyboard navigation, ARIA labels, focus management, skip links, screen reader testing.

### Phase 3 — Production Readiness
- [x] **CI/CD pipeline** — GitHub Actions with 5 jobs: security audit (cargo-audit), test & lint (clippy + fmt + tests), build (optimized release), deploy (GitHub Pages), verify (post-deployment). Includes OWASP security headers, CSP, SPA routing support. See `docs/DEPLOYMENT.md` for details.
- [ ] **PostgreSQL support** — Add feature flag for PostgreSQL alongside SQLite. Conditional compilation in db.rs.
- [ ] **Docker production build** — Verify multi-stage Dockerfile builds and runs correctly. Add health check endpoint to docker-compose.
- [ ] **Monitoring & observability** — Add Prometheus metrics endpoint. Request duration, error rate, active connections.
- [ ] **SEO static generation** — Pre-render critical pages to static HTML for instant LCP. Serve from nginx.

### Phase 4 — Enterprise Features
- [ ] **Admin dashboard** — Protected admin route with JWT auth. View inquiries, newsletter subscribers, audit logs.
- [ ] **Multi-language support** — i18n config for English, Hindi, Japanese. Config-driven translations.
- [ ] **PDF quote generator** — Generate PDF quotes from inquiry data. Attach to email response.
- [ ] **Webhook notifications** — POST to Slack/Teams on new inquiry. Configurable webhook URL.
- [ ] **Analytics integration** — Privacy-respecting analytics (Plausible/Umami). Config-driven, no third-party cookies.

---

## Project Overview

Alashore Marine Exports is a full-stack Rust web application for an Indian frozen seafood exporter. It uses Dioxus 0.6 (frontend SPA), Axum 0.7 (backend API), and a shared types crate, all in a Cargo workspace.

## Design Principles

1. **Config-Driven**: All data (products, stats, certs, FAQs, company info) lives in `frontend/src/config.rs`. Never hardcode data in components.
2. **Reusable Components**: UI elements are generic and data-driven. Pass config structs to components.
3. **TDD-First**: Write tests before implementing features. Run `cargo test --workspace` before any commit.
4. **Modular**: Each concern (handlers, middleware, components, pages) has its own file.

## Workspace Structure

```
frontend/content/
  products/               # Markdown files with TOML frontmatter (one per product)
  certifications.toml     # Config-driven cert badges (layout + entries)

frontend/src/
  main.rs           # Route enum, App component
  config.rs         # Config-driven data (derives products from markdown, certs from TOML)
  certifications/
    mod.rs           # include_str! TOML loading + public API
    parser.rs        # TOML parser for certifications config
  products/
    mod.rs           # include_str! product loading + public API
    parser.rs        # TOML frontmatter parser + markdown-to-HTML renderer
  components/
    mod.rs           # Component re-exports
    layout.rs        # Page layout wrapper
    navbar.rs        # Navigation bar
    footer.rs        # Page footer
    icons.rs         # SVG icon components
    product_card.rs  # Renders ProductConfig
    stat_counter.rs  # Renders StatItem
    cert_badge.rs    # Renders CertConfig
    inquiry_cta.rs   # CTA linking to inquiry page
  pages/             # One file per route (home.rs, about.rs, etc.)
  utils/             # Helpers (canonical URL, etc.)

backend/src/
  main.rs            # Axum server setup, router, middleware stack
  lib.rs             # Re-exports for integration tests
  config.rs          # Environment config (SMTP, DB, etc.)
  db.rs              # Database connection pool
  handlers/          # Route handlers (inquiry, newsletter, health, products, llms)
  middleware/
    security.rs      # Security headers middleware
    validate.rs      # Input validation & sanitization
  models/            # DB models
  routes/            # Route definitions

shared/src/lib.rs    # InquiryPayload, NewsletterPayload, ApiResponse, etc.
```

## Development Workflow

```bash
pnpm install              # Install Node deps (Tailwind, concurrently)
pnpm dev                  # Run frontend + backend + Tailwind watcher
cargo test --workspace    # Run all 106 tests
cargo clippy --workspace  # Lint
cargo check --workspace   # Type check
```

## Config-Driven Data

`frontend/src/config.rs` contains:
- `all_products()` -> `Vec<ProductConfig>` (6 products: shrimp, cephalopods, fish, dried)
- `company_stats()` -> `Vec<StatItem>` (6 stats)
- `certifications()` -> `Vec<CertEntry>` (5 certs, from `content/certifications.toml`)
- `cert_config()` -> `CertificationsConfig` (layout + certs from TOML)
- `cert_layout()` -> `CertLayout` (direction, rows, stamp_size, gap)
- `nav_items()` -> `Vec<NavItem>` (7 nav links)
- `faqs()` -> `Vec<FaqItem>` (4 FAQs)
- `inquiry_products()` -> inquiry dropdown options
- `COMPANY` -> `CompanyInfo` const (name, emails, address, codes)

## Reusable Components

| Component | Props | Purpose |
|---|---|---|
| `ProductCard` | `ProductConfig` | Product tile with category, certs, HS code |
| `StatCounter` | `StatItem` | Metric display (label + value + desc) |
| `CertBadge` | `CertConfig` | Certification with icon |
| `InquiryCta` | none | Call-to-action block for inquiry form |

## Security (OWASP Compliance)

Implemented in `backend/src/middleware/`:
- [x] Security headers (CSP, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy, Permissions-Policy)
- [x] Input validation: regex email (`^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)
- [x] HTML entity encoding (sanitize `<`, `>`, `&`, `"`, `'`)
- [x] Field length limits (truncation)
- [x] CORS restricted to allowed origins
- [x] 1 MB request body size limit
- [x] Rate limiting (governor, configurable RPM, 429 response)

## Testing

| Category | Count | File |
|---|---|---|
| Backend unit (validation) | 9 | `backend/src/middleware/validate.rs` |
| Backend unit (email) | 3 | `backend/src/services/email.rs` |
| Backend unit (rate limit) | 3 | `backend/src/middleware/rate_limit.rs` |
| Backend integration (API) | 18 | `backend/tests/api_tests.rs` |
| Frontend config | 30 | `frontend/tests/config_tests.rs` |
| Cert TOML parser | 7 | `frontend/src/certifications/parser.rs` |
| Product content & parser | 20 | `frontend/tests/product_content_tests.rs` + `frontend/src/products/parser.rs` |
| Shared types | 5 | `frontend/tests/shared_types_tests.rs` |
| SSR landing page | 2 | `frontend/tests/landing_page_tests.rs` |

`backend/src/lib.rs` re-exports modules so integration tests can `use backend::handlers` etc.

## Coding Conventions

- **IBM Carbon Design System**: Use Carbon spacing (16px grid), IBM Plex fonts, monochrome/blue palette.
- **Dioxus 0.6 RSX**: Use `rsx! { div { class: "...", ... } }` syntax. No HTML-style closing tags.
- **Route enum naming**: All variants end with `Page` and use struct syntax: `HomePage {}`, `ProductsPage {}`, `AboutPage {}`.
- **Tailwind classes**: Applied via `class:` attribute in RSX.
- **Error handling**: Use `Result` types. Backend handlers return `ApiResponse<T>` from shared crate.

## Visual Design System (MANDATORY — apply to every UI change)

### Color Scheme
- **Page backgrounds**: Light purple gradient (`#f8fafc → #ede9fe → #e0e7ff → #f0f9ff`). Never use plain black backgrounds for page sections.
- **Footer**: Always carbon black (`#161616` / `var(--color-carbon-gray-100)`), regardless of light/dark mode.
- **Hero section**: Deep ocean gradient (`#0a1628 → #122240 → #1a3355 → #2a6b8a`).
- **Dark mode**: Only activates when user explicitly toggles. When dark: page sections use `#0f172a → #1e1b4b` gradients. Footer stays carbon black.
- **Accent colors**: IBM Carbon Blue `#0f62fe`, Purple `#6929c4`, Teal `#009d9a`. Rotate per-element for visual variety.

### Typography & Uniformity
- **All section headings** (`h2`) MUST use the CSS class `section-title` for consistent font size (3xl mobile / 3rem desktop), weight (light), and color.
- **All section subtitles** MUST use the CSS class `section-subtitle` for consistent sizing.
- **MANDATORY: Every `section-title` MUST be wrapped in a `div { class: "section-header" }`** container. This ensures uniform `max-width: 1584px`, `margin: 0 auto 3rem`, and consistent left-alignment across all pages. Never place a `section-title` directly inside a section without this wrapper. This is a hard rule — violating it causes misaligned headings.
- **Font sizes must be uniform** across all pages. Do NOT use arbitrary inline font sizes for headings. Use the established CSS classes.
- **Font families**: `IBM Plex Sans` for body, `IBM Plex Mono` for data/metrics/badges.
- **Section header blocks** use class `section-header` for consistent max-width and spacing.

### Layout Rules
- **Section headers are left-aligned** inside `section-header` (1584px max-width, auto margins).
- **Text must be visible**: Ensure sufficient contrast in both light and dark modes. Light text on dark backgrounds, dark text on light backgrounds.
- **Spacing**: Use Carbon 16px grid. Section padding: `--spacing * 20` to `--spacing * 24` vertical.

### Component Design
- **Cards**: Use soft color gradients per card for visual interest, NOT plain white/gray.
- **Interactive components**: Must have hover/active/focus states with smooth transitions (0.3s cubic-bezier).
- **Animations**: Use CSS `@keyframes` or `IntersectionObserver`. Never use JS `setInterval` for visual animations.
- **Inline styles for custom components**: When CSS classes don't apply (e.g., positioning), use inline `style:` in RSX. But use CSS classes for anything shared across components.

### SEO & Accessibility
- Every page must have `PageSeo` component with unique title, description, and Open Graph tags.
- All sections must have `aria-labelledby` pointing to their heading `id`.
- Use `schema.org` structured data (itemscope, itemtype, itemprop) on all content sections.
- Images must have `alt` text. Decorative elements must have `aria-hidden: "true"`.
- Keyboard navigation must work on interactive components.

### Testing
- **TDD mandatory**: Write failing tests FIRST, then implement.
- **Unit tests for all config data**: Every new config function needs tests for count, field validity, uniqueness.
- **SSR landing page tests**: Mock components and verify rendered HTML contains expected classes and content.
- **Code coverage >95%** for backend and frontend.

## How To: Add a New Product

1. Add a `ProductConfig` entry to `all_products()` in `frontend/src/config.rs`.
2. Add a matching entry to `inquiry_products()` in the same file.
3. Add a CSS class in `frontend/assets/css/main.css` if custom styling is needed.
4. Tests in `frontend/tests/config_tests.rs` will automatically cover it (they test `all_products()` length and properties).
5. Run `cargo test --workspace`.

## How To: Add a New Page

1. Create `frontend/src/pages/new_page.rs` with a component function.
2. Add the route variant to the `Route` enum in `frontend/src/main.rs`: `#[route("/new-path")] NewPagePage {}`.
3. Add a `NavItem` to `nav_items()` in `frontend/src/config.rs`.
4. Export the page in `frontend/src/pages/mod.rs`.
5. Write a test in `frontend/tests/`.

## How To: Add a New API Endpoint

1. Create a handler in `backend/src/handlers/`.
2. Register the route in `backend/src/main.rs` router.
3. Re-export in `backend/src/lib.rs` if integration tests need access.
4. Add shared types to `shared/src/lib.rs` if the endpoint has a request/response body.
5. Write integration tests in `backend/tests/api_tests.rs`.

## Common Pitfalls

- **Dioxus `base_path` configuration (CRITICAL)**: The `base_path` setting in `frontend/Dioxus.toml` under `[web.app]` changes where the app is served. If set to `base_path = "some-path"`, the app will only load at `http://localhost:8045/some-path/` and will return 404 at the root `/`. **For development, NEVER set `base_path`** - omit it entirely so the app serves from root. Only use `base_path` for production GitHub Pages deployment where a subdirectory is required. If the UI returns 404 during development, check `Dioxus.toml` first and remove any `base_path` line.
- **CSS source of truth is `frontend/input.css`**, NOT `frontend/assets/css/main.css`. The `main.css` file is **generated** by Tailwind and will be overwritten. Always edit `input.css` and run `pnpm build:tailwind` to regenerate.
- **External SVGs with background rects**: Many SVGs include `<rect width="100%" fill="#FFF"/>` which will cover parent content. Always check SVG files for background rects before using them as overlays. Prefer inline SVGs in RSX for separators/decorations.
- **`id` in RSX for-loops**: Dioxus reserves `key` for list reconciliation. Use `key: "{item.id}"` on the outermost element in `for` loops. Do NOT use `id` as a variable name that shadows Dioxus internals.
- **Route variant naming**: Dioxus 0.6 Router derive macro requires each route variant to have a matching component function. `HomePage {}` needs a `fn HomePage() -> Element` function.
- **lib.rs for tests**: Backend integration tests cannot access `main.rs` modules directly. `backend/src/lib.rs` exists to re-export `handlers`, `middleware`, etc.
- **Cargo workspace warnings**: Profile settings in member `Cargo.toml` files are ignored; define them in the root `Cargo.toml`.
- **Body size limit**: The backend enforces 1 MB max. Large file uploads will need a separate endpoint with increased limits.
