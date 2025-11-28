# Justfile for Ratifact by Neura

# Build the project
build:
	cargo build

# Build and run the project
run: build
	#!/bin/bash
	set -e
	echo "Checking for .env file..."
	if [ ! -f .env ]; then
		echo "Generating .env file..."
		PASSWORD=$(openssl rand -base64 12 | tr -d "=+/" | cut -c1-16)
		echo "DATABASE_URL=postgres://ratifact:$PASSWORD@localhost:25851/ratifact" > .env
		echo "POSTGRES_USERNAME=ratifact" >> .env
		echo "POSTGRES_PASSWORD=$PASSWORD" >> .env
		echo "DEBUG_LOGS_ENABLED=true" >> .env
		echo ".env file generated with random password."
	else
		echo ".env file already exists, skipping generation."
	fi
	echo "Checking PostgreSQL status..."
	if ss -tln | grep -q :25851; then
		echo "PostgreSQL port 25851 is in use, assuming running."
	else
		echo "Starting PostgreSQL..."
		docker compose up -d
	fi
	echo "Starting Ratifact application..."
	cargo run

# Run tests
test:
	cargo test

# Check code without building
check:
	cargo check

# Build release version
release:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Lint code
clippy:
	cargo clippy

# Run all checks
all: check test

# Uninstall the application
uninstall:
	@echo "Starting uninstallation process..."
	@echo "Please run the appropriate script for your OS:"
	@echo ""
	@echo "Linux:"
	@echo "  bash src/scripts/linux/uninstall.sh"
	@echo ""
	@echo "macOS:"
	@echo "  bash src/scripts/macos/uninstall.sh"
	@echo ""
	@echo "Windows (PowerShell as Administrator):"
	@echo "  powershell -ExecutionPolicy Bypass -File src/scripts/windows/uninstall.ps1"
	@echo ""
	@echo "Or from remote:"
	@echo ""
	@echo "Linux/macOS:"
	@echo "  curl -fsSL https://raw.githubusercontent.com/adolfousier/ratifact/main/src/scripts/linux/uninstall.sh | bash"
	@echo ""
	@echo "Windows (PowerShell as Administrator):"
	@echo "  powershell -Command \"iwr -useb https://raw.githubusercontent.com/adolfousier/ratifact/main/src/scripts/windows/uninstall.ps1 | iex\""

# Show help
help:
	@echo "Available targets:"
	@echo "  build       - Build the project"
	@echo "  run         - Build and run the project"
	@echo "  test        - Run tests"
	@echo "  check       - Check code without building"
	@echo "  release     - Build release version"
	@echo "  clean       - Clean build artifacts"
	@echo "  fmt         - Format code"
	@echo "  clippy      - Lint code"
	@echo "  all         - Run check and test"
	@echo "  uninstall   - Show uninstall instructions"
	@echo "  help        - Show this help"
