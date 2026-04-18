# CI Commands Quick Reference

## 🚀 Development

```bash
pnpm dev              # Start dev servers (frontend + backend + Tailwind)
pnpm kill-ports       # Kill processes on ports 8045/8046
```

## 🧪 Testing Locally (FREE - No GitHub Actions costs)

```bash
pnpm ci:local         # FULL CI PIPELINE (~5-10 min)
                      # Runs everything GitHub Actions would run:
                      # - Security audit
                      # - Format check
                      # - Clippy linting
                      # - All tests
                      # - Production build
                      # - Security verification

pnpm ci:quick         # QUICK CHECKS (~2-3 min)
                      # - Format check
                      # - Clippy linting
                      # - All tests

pnpm ci:security      # Security audit only
pnpm ci:format        # Format check only
pnpm ci:format:fix    # Auto-fix formatting
pnpm precommit        # Pre-commit checks (auto-runs on commit)
```

## 📦 Building

```bash
pnpm build:tailwind   # Build Tailwind CSS (minified)
pnpm build            # Build production frontend
pnpm preview:dist     # Preview production build locally (port 8080)
```

## 🔍 Code Quality

```bash
pnpm check            # cargo check --workspace
pnpm lint             # cargo clippy (zero warnings)
pnpm test             # cargo test --workspace
pnpm test:unit        # Unit tests only
pnpm test:integration # Integration tests only
pnpm verify           # check + lint + test
```

## 🚢 Deployment

**Cost-Optimized: Manual trigger only!**

1. Run local CI: `pnpm ci:local`
2. If all passes, push: `git push origin main`
3. Manually trigger deployment:
   - Go to: https://github.com/cyfen-code/alashore-marine-pvt-ltd/actions
   - Click: "Deploy to GitHub Pages (Production)"
   - Click: "Run workflow" → "Run workflow"

**GitHub Actions cost:** ~15 minutes only when you deploy (not on every push)

## 📝 Pre-Commit Hooks

**Automatically runs on `git commit`:**
- Code formatting check
- Clippy linting
- Unit tests

**To skip (not recommended):**
```bash
git commit --no-verify
```

## 💰 Cost Savings

| Action | Before | After |
|--------|--------|-------|
| Every push | 15 min GitHub Actions | 0 min (runs locally) |
| Monthly usage | ~750 min ($4-8) | ~45 min ($0 free tier) |
| **Savings** | - | **~90% reduction** |

## 🆘 Help

```bash
pnpm run              # List all available commands
```

**Documentation:**
- [docs/COST_OPTIMIZATION.md](docs/COST_OPTIMIZATION.md) - Full cost-saving guide
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) - Deployment guide
- [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) - Common issues
- [CLAUDE.md](CLAUDE.md) - Development guide

---

**Key Principle:** Test locally (free), deploy manually (controlled cost).
