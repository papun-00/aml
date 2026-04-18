# Cost Optimization Guide — GitHub Actions Billing

**Status:** ✅ Implemented
**Monthly Savings:** ~90-95% reduction in GitHub Actions minutes

---

## Problem

GitHub Actions charges for compute time:
- **Free tier:** 2,000 minutes/month for private repos
- **Cost:** $0.008/minute after free tier
- **Our pipeline:** ~15 minutes per run
- **Frequent pushes:** Could exhaust free tier quickly

**Example:**
- 10 pushes/day × 15 min = 150 min/day
- 150 min/day × 30 days = 4,500 min/month
- **Overage:** 2,500 minutes = **$20/month** (for development)

---

## Solution: Run CI Locally

**New Workflow:**
1. ✅ Run all checks **locally** before pushing (free, instant feedback)
2. ✅ GitHub Actions **manual trigger only** (zero cost until deployment)
3. ✅ Pre-commit hooks catch issues **before commit** (prevent waste)

**Result:**
- **Development:** 0 GitHub Actions minutes used
- **Deployment:** Only ~15 minutes when manually triggered
- **Monthly usage:** ~15-45 minutes (1-3 deployments/month)
- **Cost:** **$0** (well within free tier)

---

## Local CI Commands

### Full CI Pipeline (runs everything GitHub Actions would run)

```bash
pnpm ci:local
```

**What it does:**
1. 🔒 Security audit (`cargo audit`)
2. ⚙️ Code formatting check
3. ⚙️ Clippy linting (zero warnings enforced)
4. ✅ Unit tests
5. ✅ Integration tests
6. 📦 Production build (Tailwind + Dioxus release)
7. 🔍 Security verification (no leaked secrets/source files)
8. 👀 Preview instructions

**Duration:** ~5-10 minutes (on your machine, free)

---

### Quick Checks (faster, runs before every push)

```bash
pnpm ci:quick
```

**What it does:**
1. Code formatting check
2. Clippy linting
3. All tests

**Duration:** ~2-3 minutes

---

### Individual Commands

```bash
# Security audit only
pnpm ci:security

# Format check
pnpm ci:format

# Format fix (auto-fix formatting issues)
pnpm ci:format:fix

# Pre-commit checks (format + lint + unit tests)
pnpm precommit

# Preview production build locally
pnpm preview:dist  # Opens http://localhost:8080
```

---

## Automated Pre-Commit Hooks

**Already configured!** Every commit automatically runs:
1. ✅ Code formatting check
2. ✅ Clippy linting
3. ✅ Unit tests

**Commit will be blocked if checks fail.**

To skip (not recommended):
```bash
git commit --no-verify
```

---

## Deployment Workflow (Cost-Optimized)

### Step 1: Develop Locally
```bash
pnpm dev  # Regular development server
```

Make your changes, test in browser.

### Step 2: Run Local CI
```bash
pnpm ci:local
```

This runs **the exact same checks** as GitHub Actions, but on your machine (free).

**If anything fails:** Fix it locally, re-run.

### Step 3: Commit Changes
```bash
git add .
git commit -m "Your changes"
```

Pre-commit hook automatically runs basic checks (~30 seconds).

### Step 4: Push to GitHub
```bash
git push origin main
```

**No GitHub Actions triggered automatically!** (Cost savings)

### Step 5: Manual Deployment (when ready)

**Option A: GitHub UI**
1. Go to https://github.com/cyfen-code/alashore-marine-pvt-ltd/actions
2. Click "Deploy to GitHub Pages (Production)"
3. Click "Run workflow" → Select `main` branch → "Run workflow"

**Option B: GitHub CLI**
```bash
gh workflow run deploy-pages.yml
```

**Cost:** ~15 minutes of GitHub Actions time (only when you explicitly deploy)

---

## Cost Comparison

### Before (Automatic Deployment)

| Activity | Frequency | Minutes/Month | Cost |
|----------|-----------|---------------|------|
| Every push to main | 50 pushes | 750 min | $4.00 |
| Total | - | **750 min** | **$4.00** |

### After (Manual Deployment)

| Activity | Frequency | Minutes/Month | Cost |
|----------|-----------|---------------|------|
| Local CI (on your machine) | 50 runs | 0 min | **$0.00** |
| Manual deployment | 3 deployments | 45 min | **$0.00** (free tier) |
| Total | - | **45 min** | **$0.00** |

**Savings:** $4.00/month (90% reduction for this usage pattern)

**For heavy usage:**
- 200 pushes/month before: 3,000 min = **$8.00**
- 200 pushes/month after: 45 min = **$0.00**
- **Savings: $96/year**

---

## GitHub Actions Configuration

The workflow is now **manual trigger only**:

```yaml
on:
  workflow_dispatch:  # Manual trigger only
  # push:
  #   branches: [main]  # ← COMMENTED OUT to save costs
```

**To re-enable automatic deployment** (not recommended):
Uncomment the `push:` section in `.github/workflows/deploy-pages.yml`

---

## Best Practices

### ✅ DO

1. **Always run `pnpm ci:local` before manual deployment**
2. **Let pre-commit hooks do their job** (don't skip with `--no-verify`)
3. **Use `pnpm ci:quick` frequently** during development
4. **Deploy manually only when releasing** (not every push)
5. **Preview locally first** with `pnpm preview:dist`

### ❌ DON'T

1. **Don't push without running local checks** (wastes time debugging later)
2. **Don't skip pre-commit hooks** (catches issues early)
3. **Don't re-enable automatic deployment** (increases costs)
4. **Don't deploy for every small change** (batch changes into releases)

---

## Troubleshooting

### Pre-commit hook not running?

```bash
# Re-configure hooks
bash scripts/setup-hooks.sh

# Verify
git config core.hooksPath
# Should output: .githooks
```

### Want to disable hooks temporarily?

```bash
# Commit without hooks (use sparingly)
git commit --no-verify
```

### Local CI script fails?

```bash
# Check dependencies
which cargo        # Should exist
which dx           # Should exist
which tailwindcss  # Should exist
which cargo-audit  # Should exist

# Install missing tools
cargo install dioxus-cli --locked
cargo install cargo-audit --locked
pnpm install -g tailwindcss
```

### GitHub Actions workflow not appearing?

The workflow still exists but won't run automatically. To manually trigger:
1. Go to Actions tab
2. Select "Deploy to GitHub Pages (Production)"
3. Click "Run workflow"

---

## Monitoring Usage

### Check GitHub Actions usage:

1. Go to https://github.com/settings/billing
2. Click "Plans and usage"
3. See "Actions minutes used this month"

**Goal:** Stay under 100 minutes/month (95% buffer)

### Local build artifacts

All build outputs go to:
- `frontend/dist/` - Production build artifacts
- `target/` - Rust build cache

These are ignored by `.gitignore` (not committed).

---

## Advanced: CI/CD for Teams

If working in a team, consider:

### Option 1: Branch Protection Rules
```
Settings → Branches → main → Require status checks
```
But this costs GitHub Actions minutes per PR.

### Option 2: Local CI Requirement (Free)
Add to `CONTRIBUTING.md`:
> Before opening a PR, run `pnpm ci:local` and include output in PR description.

### Option 3: Self-Hosted Runner (Advanced)
Run GitHub Actions on your own server (free minutes, but requires server maintenance).

---

## Future Enhancements

### Caching for Faster Local Builds

Already implemented:
- Cargo incremental compilation
- `target/` reused across builds

To clear cache:
```bash
cargo clean
```

### Parallel Testing

```bash
# Run tests in parallel (faster)
cargo test --workspace -- --test-threads=8
```

### Docker-based CI (Optional)

For reproducible builds:
```dockerfile
FROM rust:1.84
RUN cargo install dioxus-cli
# ... rest of Dockerfile
```

```bash
docker build -t alashore-ci .
docker run -v $(pwd):/app alashore-ci pnpm ci:local
```

---

## Summary

**Before:**
- ❌ GitHub Actions run on every push
- ❌ ~750 minutes/month used
- ❌ Potential overage costs

**After:**
- ✅ Local CI runs for free
- ✅ ~45 minutes/month used
- ✅ Zero overage risk
- ✅ Faster feedback (local is instant)
- ✅ Manual control over deployments

**Key Commands:**
```bash
pnpm ci:local      # Full CI pipeline (before deploy)
pnpm ci:quick      # Quick checks (during development)
pnpm precommit     # Pre-commit checks (automatic)
pnpm preview:dist  # Preview production build
```

**Deployment:**
Only trigger GitHub Actions manually when you're ready to deploy to production.

---

**Last Updated:** 2026-04-18
**Estimated Monthly Savings:** $4-8 (depending on usage)
**ROI:** ∞ (one-time setup, perpetual savings)
