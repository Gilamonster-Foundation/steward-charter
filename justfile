# steward-charter — dev tasks.
# PIPELINE PARITY: `check` mirrors .github/workflows/ci.yml and .githooks/pre-push.
# When editing the steps here, update both of those to match.

check: fmt-check lint test

fmt-check:
    cargo fmt --check

lint:
    cargo clippy --all-targets -- -D warnings

test:
    cargo test

install-hooks:
    git config core.hooksPath .githooks
