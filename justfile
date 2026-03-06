# List available recipes
default:
    @just --list

# Run all checks (fmt, clippy, test)
check: fmt-check clippy test

# Build the project
build:
    cargo build

# Run tests
test:
    cargo test --all-targets
    cargo test --doc

# Run clippy lints
clippy:
    cargo clippy --all-targets

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Run clippy and auto-fix
fix:
    cargo clippy --all-targets --fix --allow-dirty
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean

# Build docs
doc:
    cargo doc --no-deps --open
