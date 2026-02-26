.PHONY: all build install clean test help

CARGO := cargo
TARGET := target/release/aish

all: build

build:
	@echo "Building aish..."
	$(CARGO) build --release

install: build
	@echo "Installing aish..."
	$(CARGO) install --path .
	@echo "Installation complete!"
	@echo ""
	@echo "Setup instructions:"
	@echo "1. Set your API key:"
	@echo "   export OPENAI_API_KEY='your-api-key'"
	@echo ""
	@echo "2. (Optional) Add shell integration to your ~/.bashrc or ~/.zshrc:"
	@echo "   source $(PWD)/ai-integration.sh"
	@echo ""
	@echo "3. Test it:"
	@echo "   aish 'Hello'"

dev:
	$(CARGO) build

run: dev
	$(CARGO) run

clean:
	@echo "Cleaning..."
	$(CARGO) clean

test:
	@echo "Running tests..."
	$(CARGO) test

help:
	@echo "Aish - AI Shell Helper - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  make build    - Build the project (debug)"
	@echo "  make install  - Build and install to ~/.cargo/bin"
	@echo "  make dev      - Build in debug mode"
	@echo "  make run      - Run the project"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make test     - Run tests"
	@echo "  make help     - Show this help"
	@echo ""
	@echo "Quick start:"
	@echo "  1. make install"
	@echo "  2. export OPENAI_API_KEY='your-key'"
	@echo "  3. aish 'Hello, AI!'"
