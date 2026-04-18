# Deployment Guide — Alashore Marine Exports

This document explains how to deploy the Alashore Marine Exports website to GitHub Pages with production-grade security and performance.

## Table of Contents

1. [Overview](#overview)
2. [GitHub Pages Setup](#github-pages-setup)
3. [Deployment Pipeline](#deployment-pipeline)
4. [Security Features](#security-features)
5. [Deployment Process](#deployment-process)
6. [Troubleshooting](#troubleshooting)
7. [Manual Deployment](#manual-deployment)

---

## Overview

The project uses GitHub Actions to automatically build and deploy to GitHub Pages whenever code is pushed to the `main` branch. The deployment includes:

- **Automated security audits** using `cargo-audit`
- **Code quality checks** with `clippy` and `rustfmt`
- **Comprehensive testing** (unit + integration)
- **Production-optimized builds** with Rust release mode and minified CSS
- **OWASP security headers** for protection against common attacks
- **Content Security Policy (CSP)** to prevent XSS and injection attacks
- **SPA routing support** with proper 404 handling
- **Post-deployment verification** to ensure site accessibility

---

## GitHub Pages Setup

### Prerequisites

1. **Repository on GitHub:** `https://github.com/cyfen-code/alashore-marine-pvt-ltd`
2. **GitHub Pages enabled** in repository settings
3. **Actions permissions** configured correctly

### Initial Configuration

1. Go to your repository on GitHub
2. Navigate to **Settings → Pages**
3. Under **Source**, select:
   - **Source:** GitHub Actions (not "Deploy from a branch")
4. Save the settings

### Repository Settings

Ensure the following repository settings are configured:

- **Actions permissions:** "Allow all actions and reusable workflows"
- **Workflow permissions:** "Read and write permissions"
- **Pages deployment:** GitHub Actions

---

## Deployment Pipeline

The deployment workflow consists of 5 jobs that run sequentially:

### Job 1: Security Audit
```
✅ Installs cargo-audit
✅ Checks for known vulnerabilities in dependencies
✅ Fails pipeline if vulnerabilities found (--deny warnings)
```

### Job 2: Test & Lint
```
✅ Checks code formatting (cargo fmt)
✅ Runs Clippy with zero warnings enforced
✅ Runs all unit tests
✅ Runs all integration tests
✅ Fails if any test fails or warnings exist
```

### Job 3: Build Production Assets
```
✅ Installs Rust + wasm32-unknown-unknown target
✅ Installs Dioxus CLI
✅ Installs Tailwind CSS v4 standalone CLI
✅ Builds and minifies Tailwind CSS
✅ Applies production Dioxus.toml config (with base_path)
✅ Builds frontend in release mode
✅ Adds .nojekyll file
✅ Creates 404.html for SPA routing
✅ Injects security headers (_headers file)
✅ Verifies all critical files exist
✅ Uploads build artifact
```

### Job 4: Deploy to GitHub Pages
```
✅ Downloads build artifact
✅ Configures GitHub Pages
✅ Uploads to GitHub Pages
✅ Deploys to production
```

### Job 5: Post-Deployment Verification
```
✅ Waits for deployment propagation
✅ Tests site accessibility
✅ Prints deployment summary
```

---

## Security Features

The deployment includes comprehensive security measures:

### 1. OWASP Security Headers

All deployed files include these headers (via `_headers` file):

```
X-Frame-Options: DENY
  → Prevents clickjacking attacks

X-Content-Type-Options: nosniff
  → Prevents MIME type sniffing

X-XSS-Protection: 1; mode=block
  → Enables browser XSS filter

Referrer-Policy: strict-origin-when-cross-origin
  → Controls referrer information leakage

Permissions-Policy: camera=(), microphone=(), geolocation=(), payment=()...
  → Disables unnecessary browser features

Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
  → Enforces HTTPS for 2 years
```

### 2. Content Security Policy (CSP)

```
default-src 'self'
  → Only load resources from same origin

script-src 'self' 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval'
  → Allow scripts from same origin + WASM support

style-src 'self' 'unsafe-inline' https://fonts.googleapis.com
  → Allow styles from same origin + Google Fonts

img-src 'self' https://images.unsplash.com data: https:
  → Allow images from specific trusted sources

frame-ancestors 'none'
  → Prevent embedding in iframes

form-action 'self'
  → Restrict form submissions to same origin

upgrade-insecure-requests
  → Automatically upgrade HTTP to HTTPS
```

### 3. Cross-Origin Isolation

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Resource-Policy: same-origin
```

These headers enable advanced browser features and prevent Spectre-like attacks.

### 4. Dependency Security

- **cargo-audit** checks all dependencies for known CVEs
- Pipeline fails if any vulnerabilities are found
- Automated updates can be configured with Dependabot

### 5. Code Quality Enforcement

- **rustfmt** ensures consistent code formatting
- **clippy** catches common mistakes and anti-patterns
- **zero warnings policy** enforced in CI

---

## Deployment Process

### Automatic Deployment (Recommended)

Simply push to the `main` branch:

```bash
git add .
git commit -m "Your commit message"
git push origin main
```

GitHub Actions will automatically:
1. Run security audit
2. Run tests and linting
3. Build production assets
4. Deploy to GitHub Pages
5. Verify deployment

**Deployment time:** ~10-15 minutes

### Manual Deployment via GitHub UI

1. Go to **Actions** tab in your repository
2. Select "Deploy to GitHub Pages (Production)" workflow
3. Click "Run workflow"
4. Select `main` branch
5. Click "Run workflow"

### Monitoring Deployment

1. Go to **Actions** tab
2. Click on the running workflow
3. Monitor each job's progress:
   - Security Audit → Test & Lint → Build → Deploy → Verify

**Success indicators:**
- All jobs show green checkmarks ✅
- "Post-Deployment Verification" job prints success summary
- Site is accessible at the deployment URL

---

## Deployment Configuration Files

### `frontend/Dioxus.toml` (Development)
```toml
[web.app]
title = "Alashore Marine Exports | Premium Seafood Exporter, India"
# No base_path — serves from root / for local development
```

### `frontend/Dioxus.production.toml` (Production)
```toml
[web.app]
title = "Alashore Marine Exports | Premium Seafood Exporter, India"
base_path = "alashore-marine-pvt-ltd"  # GitHub Pages subdirectory
```

**IMPORTANT:** Never set `base_path` in the main `Dioxus.toml` for development. The production config is automatically applied during CI build.

---

## Troubleshooting

### Pipeline Fails at Security Audit

**Symptom:** Security audit job fails with vulnerability warnings.

**Solution:**
1. Review the audit output to identify vulnerable dependencies
2. Update dependencies:
   ```bash
   cargo update
   cargo audit
   ```
3. If vulnerability is in a direct dependency, check for updates or alternatives
4. If vulnerability is in a transitive dependency, wait for upstream fix or use `cargo-audit` ignore list (not recommended for production)

### Pipeline Fails at Test Stage

**Symptom:** Tests fail during "Test & Lint" job.

**Solution:**
1. Run tests locally to reproduce:
   ```bash
   cargo test --workspace
   ```
2. Fix failing tests
3. Ensure all tests pass locally before pushing

### Pipeline Fails at Clippy Stage

**Symptom:** Clippy finds warnings or errors.

**Solution:**
1. Run clippy locally:
   ```bash
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```
2. Fix all warnings
3. Commit and push

### Deployment Succeeds but Site Shows 404

**Symptom:** Pipeline completes successfully but site is not accessible.

**Possible causes:**

1. **GitHub Pages not enabled** → Check Settings → Pages
2. **Wrong source configured** → Ensure "GitHub Actions" is selected
3. **Propagation delay** → Wait 5-10 minutes after deployment
4. **Wrong URL** → Check the deployment output for correct URL

**Correct URL format:**
```
https://cyfen-code.github.io/alashore-marine-pvt-ltd/
```

### SPA Routes Return 404

**Symptom:** Homepage loads but `/about`, `/products` etc. return 404.

**Solution:**
This is expected with GitHub Pages. The workflow creates a `404.html` that redirects to `index.html`, allowing client-side routing to work. If this doesn't work:

1. Check `frontend/dist/404.html` exists in build artifact
2. Verify the workflow step "Create 404.html for SPA routing" ran successfully
3. Clear browser cache and try again

### Security Headers Not Applied

**Symptom:** Browser dev tools show missing security headers.

**Explanation:**
GitHub Pages does not support custom `_headers` files like Netlify/Vercel. The `_headers` file in the workflow is included for documentation and future use, but **GitHub Pages does not apply it**.

**Alternative:** Security headers must be applied at the application level (backend) or via a CDN like Cloudflare.

**Note:** For GitHub Pages deployments, the backend security headers (configured in `backend/src/middleware/security.rs`) will not apply since the frontend is served statically.

---

## Manual Deployment (Emergency)

If GitHub Actions is unavailable, you can deploy manually:

### Step 1: Build Locally

```bash
# Build Tailwind CSS
cd frontend
tailwindcss -i ./input.css -o ./assets/css/main.css --minify
cp ./assets/css/main.css ./public/assets/css/main.css

# Apply production config
cp Dioxus.production.toml Dioxus.toml

# Build frontend
dx build --release

# Add required files
touch dist/.nojekyll
cp dist/index.html dist/404.html
```

### Step 2: Deploy via gh-pages Branch

```bash
# Install ghp-import (one-time)
pip install ghp-import

# Deploy
ghp-import -n -p -f frontend/dist
```

### Step 3: Restore Development Config

```bash
# Restore original Dioxus.toml
git checkout frontend/Dioxus.toml
```

---

## Production Checklist

Before deploying to production, ensure:

- [ ] All tests pass locally (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No security vulnerabilities (`cargo audit`)
- [ ] Environment variables documented in `.env.example`
- [ ] Production config tested locally
- [ ] GitHub Pages enabled in repository settings
- [ ] GitHub Actions workflow permissions configured
- [ ] Domain configured (if using custom domain)
- [ ] HTTPS enforced in GitHub Pages settings

---

## Post-Deployment Tasks

After successful deployment:

1. **Test the live site:**
   - Homepage loads correctly
   - All routes work (SPA routing)
   - Images and CSS load
   - Forms submit correctly (if backend is connected)

2. **Check browser console** for errors or warnings

3. **Verify security headers** (even though GitHub Pages may not apply `_headers`)

4. **Test on multiple devices/browsers**

5. **Monitor GitHub Actions** for future deployments

6. **Set up monitoring** (optional):
   - GitHub Dependabot for security updates
   - Uptime monitoring service
   - Analytics (privacy-respecting)

---

## Rollback Procedure

If a deployment introduces issues:

### Option 1: Revert via Git

```bash
git revert HEAD
git push origin main
```

This triggers a new deployment with the previous version.

### Option 2: Redeploy Previous Commit

```bash
git checkout <previous-commit-hash>
git push origin main --force
```

**Warning:** Force push requires careful coordination with team.

### Option 3: Manual Rollback via GitHub UI

1. Go to **Actions** tab
2. Find the last successful deployment
3. Click "Re-run all jobs"

---

## Continuous Improvement

### Recommended Enhancements

1. **Custom Domain:**
   - Add `CNAME` file with your domain
   - Configure DNS records
   - Enable HTTPS in GitHub Pages settings

2. **CDN with Security Headers:**
   - Use Cloudflare in front of GitHub Pages
   - Configure security headers at CDN level
   - Enable DDoS protection

3. **Monitoring & Alerts:**
   - Set up GitHub Actions status badges
   - Configure Slack/Discord notifications for failed deployments
   - Use Sentry for frontend error tracking

4. **Performance Optimization:**
   - Add brotli compression (via CDN)
   - Implement service worker for offline support
   - Pre-render critical pages

5. **Automated Dependency Updates:**
   - Enable Dependabot in repository settings
   - Configure automatic PR creation for updates
   - Set up automated testing for dependency PRs

---

**Last Updated:** 2026-04-18
**Maintainer:** Development Team
**Related Docs:** CLAUDE.md, TROUBLESHOOTING.md, README.md
