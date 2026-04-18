#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Local CI Pipeline — Alashore Marine Exports
# Runs the full CI pipeline locally to catch issues before deployment
# Saves GitHub Actions billing by running checks on your machine
# ═══════════════════════════════════════════════════════════════════════

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Emoji indicators
CHECK="✅"
CROSS="❌"
INFO="ℹ️"
ROCKET="🚀"
LOCK="🔒"
GEAR="⚙️"
PACKAGE="📦"

# Track time
START_TIME=$(date +%s)

# Print banner
echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🚀 ALASHORE MARINE — LOCAL CI PIPELINE${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Job counter
JOBS_PASSED=0
JOBS_FAILED=0
TOTAL_JOBS=5

# Function to print stage header
print_stage() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo ""
}

# Function to handle job result
job_result() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}${CHECK} $1 PASSED${NC}"
        ((JOBS_PASSED++))
        return 0
    else
        echo -e "${RED}${CROSS} $1 FAILED${NC}"
        ((JOBS_FAILED++))
        return 1
    fi
}

# ═══════════════════════════════════════════════════════════════════════
# JOB 1: Security Audit
# ═══════════════════════════════════════════════════════════════════════
print_stage "🔒 JOB 1/5: Security Audit"

echo -e "${BLUE}${INFO} Checking for cargo-audit...${NC}"
if ! command -v cargo-audit &> /dev/null; then
    echo -e "${YELLOW}⚠️  cargo-audit not found, installing...${NC}"
    cargo install cargo-audit --locked --version 0.22.1
fi

echo -e "${BLUE}${INFO} Running security audit...${NC}"
cargo audit --deny vulnerability
job_result "Security Audit" || true  # Continue even if audit fails

# ═══════════════════════════════════════════════════════════════════════
# JOB 2: Code Quality & Testing
# ═══════════════════════════════════════════════════════════════════════
print_stage "⚙️  JOB 2/5: Code Quality & Testing"

echo -e "${BLUE}${INFO} Checking code formatting...${NC}"
cargo fmt --all -- --check
job_result "Code Formatting" || exit 1

echo -e "${BLUE}${INFO} Running Clippy (zero warnings enforced)...${NC}"
cargo clippy --workspace --all-targets --all-features -- -D warnings
job_result "Clippy Linting" || exit 1

echo -e "${BLUE}${INFO} Running unit tests...${NC}"
cargo test --workspace --lib
job_result "Unit Tests" || exit 1

echo -e "${BLUE}${INFO} Running integration tests...${NC}"
cargo test --workspace --test '*'
job_result "Integration Tests" || exit 1

echo -e "${BLUE}${INFO} Running all tests (no fail fast)...${NC}"
cargo test --workspace --no-fail-fast
job_result "Full Test Suite" || exit 1

# ═══════════════════════════════════════════════════════════════════════
# JOB 3: Build Production Assets
# ═══════════════════════════════════════════════════════════════════════
print_stage "📦 JOB 3/5: Build Production Assets"

echo -e "${BLUE}${INFO} Building Tailwind CSS (minified)...${NC}"
cd frontend
if ! command -v tailwindcss &> /dev/null; then
    echo -e "${YELLOW}⚠️  Tailwind CLI not found. Install with: pnpm install -g tailwindcss${NC}"
    exit 1
fi
tailwindcss -i ./input.css -o ./assets/css/main.css --minify
mkdir -p ./public/assets/css
cp ./assets/css/main.css ./public/assets/css/main.css
cd ..
job_result "Tailwind CSS Build" || exit 1

echo -e "${BLUE}${INFO} Applying production Dioxus config...${NC}"
cd frontend
cp Dioxus.production.toml Dioxus.toml
cd ..
job_result "Production Config" || exit 1

echo -e "${BLUE}${INFO} Building frontend (release mode)...${NC}"
cd frontend
if ! command -v dx &> /dev/null; then
    echo -e "${YELLOW}⚠️  Dioxus CLI not found. Install with: cargo install dioxus-cli --locked${NC}"
    exit 1
fi
dx build --release
cd ..
job_result "Frontend Build" || exit 1

echo -e "${BLUE}${INFO} Adding required files...${NC}"
touch frontend/dist/.nojekyll
cp frontend/dist/index.html frontend/dist/404.html
cat > frontend/dist/robots.txt << 'EOF'
User-agent: *
Allow: /
Sitemap: https://cyfen-code.github.io/alashore-marine-pvt-ltd/sitemap.xml
EOF
job_result "Required Files" || exit 1

echo -e "${BLUE}${INFO} Verifying build artifacts...${NC}"
test -f frontend/dist/index.html || { echo "Missing index.html"; exit 1; }
test -f frontend/dist/404.html || { echo "Missing 404.html"; exit 1; }
test -f frontend/dist/.nojekyll || { echo "Missing .nojekyll"; exit 1; }
test -f frontend/dist/robots.txt || { echo "Missing robots.txt"; exit 1; }
! test -f frontend/dist/.env || { echo "ERROR: .env file leaked!"; exit 1; }
! find frontend/dist -name "*.rs" -o -name "Cargo.toml" -o -name "Cargo.lock" | grep -q . || { echo "ERROR: Source files leaked!"; exit 1; }
job_result "Security Verification" || exit 1

# ═══════════════════════════════════════════════════════════════════════
# JOB 4: Local Preview
# ═══════════════════════════════════════════════════════════════════════
print_stage "👀 JOB 4/5: Local Preview"

echo -e "${BLUE}${INFO} Build artifact location: ${CYAN}frontend/dist/${NC}"
echo -e "${BLUE}${INFO} Total files in dist:${NC}"
find frontend/dist -type f | wc -l

echo -e "${GREEN}${CHECK} Production build ready!${NC}"
echo ""
echo -e "${YELLOW}To preview locally:${NC}"
echo -e "  ${CYAN}cd frontend/dist && python3 -m http.server 8080${NC}"
echo -e "  ${CYAN}# OR${NC}"
echo -e "  ${CYAN}cd frontend && dx serve${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════
# JOB 5: Restore Development Config
# ═══════════════════════════════════════════════════════════════════════
print_stage "🔄 JOB 5/5: Cleanup"

echo -e "${BLUE}${INFO} Restoring development Dioxus config...${NC}"
cd frontend
git checkout Dioxus.toml 2>/dev/null || echo "No changes to restore"
cd ..
job_result "Config Restore" || true

# ═══════════════════════════════════════════════════════════════════════
# SUMMARY
# ═══════════════════════════════════════════════════════════════════════
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🎉 CI PIPELINE COMPLETE${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${GREEN}${CHECK} Jobs Passed:${NC}  ${JOBS_PASSED}/${TOTAL_JOBS}"
echo -e "  ${RED}${CROSS} Jobs Failed:${NC}  ${JOBS_FAILED}/${TOTAL_JOBS}"
echo -e "  ${CYAN}⏱️  Duration:${NC}     ${MINUTES}m ${SECONDS}s"
echo ""

if [ $JOBS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ ALL CHECKS PASSED!${NC}"
    echo ""
    echo -e "${YELLOW}Ready to deploy to GitHub Pages:${NC}"
    echo -e "  1. Push your changes: ${CYAN}git push origin main${NC}"
    echo -e "  2. Go to GitHub Actions: ${CYAN}https://github.com/cyfen-code/alashore-marine-pvt-ltd/actions${NC}"
    echo -e "  3. Select 'Deploy to GitHub Pages (Production)'"
    echo -e "  4. Click 'Run workflow' → 'Run workflow'"
    echo ""
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
    exit 0
else
    echo -e "${RED}❌ SOME CHECKS FAILED!${NC}"
    echo ""
    echo -e "${YELLOW}Fix the issues above before deploying.${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
    exit 1
fi
