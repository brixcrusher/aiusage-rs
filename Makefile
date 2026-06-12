.PHONY: all build test check fmt lint clean help

CARGO = cargo

all: check test build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

fmt_check:
	$(CARGO) fmt --check

lint:
	$(CARGO) clippy -- -D warnings

lint_all:
	$(CARGO) clippy --all-targets -- -D warnings

clean:
	$(CARGO) clean

doc:
	$(CARGO) doc --no-deps

audit:
	$(CARGO) audit 2>/dev/null || echo "cargo-audit not installed (run: cargo install cargo-audit)"

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  build       Build the project (debug)"
	@echo "  release     Build the project (release)"
	@echo "  test        Run all tests"
	@echo "  check       Check code compiles (no output)"
	@echo "  fmt         Format code with rustfmt"
	@echo "  fmt_check   Check formatting without modifying"
	@echo "  lint        Run clippy lints"
	@echo "  lint_all    Run clippy on all targets"
	@echo "  clean       Remove build artifacts"
	@echo "  doc         Build documentation"
	@echo "  audit       Run security audit (if cargo-audit installed)"
	@echo "  all         Run check, test, and build"
	@echo "  help        Show this message"
