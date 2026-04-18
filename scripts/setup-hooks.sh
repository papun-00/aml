#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Setup Git Hooks — Alashore Marine Exports
# Installs pre-commit hooks to run checks before every commit
# ═══════════════════════════════════════════════════════════════════════

set -e

echo "🔧 Setting up Git hooks..."

# Configure Git to use our hooks directory
git config core.hooksPath .githooks

echo "✅ Git hooks configured!"
echo ""
echo "Pre-commit hook will now run on every commit:"
echo "  • Code formatting check"
echo "  • Clippy linting"
echo "  • Unit tests"
echo ""
echo "To skip hooks (not recommended): git commit --no-verify"
echo ""
