.PHONY: help install build test clean dev deploy-testnet deploy-mainnet

help:
	@echo "Soroban Multi-Sig Treasury Vault"
	@echo ""
	@echo "Available commands:"
	@echo "  make install          - Install all dependencies"
	@echo "  make build            - Build all components"
	@echo "  make build-contracts  - Build smart contracts"
	@echo "  make test             - Run all tests"
	@echo "  make test-unit        - Run unit tests"
	@echo "  make test-integration - Run integration tests"
	@echo "  make test-security    - Run security tests"
	@echo "  make coverage         - Generate coverage report"
	@echo "  make dev              - Start local development"
	@echo "  make deploy-testnet   - Deploy to testnet"
	@echo "  make deploy-mainnet   - Deploy to mainnet"
	@echo "  make clean            - Clean build artifacts"

install:
	@echo "Installing dependencies..."
	cd contracts && cargo build
	cd frontend && npm install
	cd backend && npm install

build: build-contracts
	@echo "Building frontend..."
	cd frontend && npm run build
	@echo "Building backend..."
	cd backend && npm run build

build-contracts:
	@echo "Building Soroban contracts..."
	cd contracts/treasury && cargo build --target wasm32-unknown-unknown --release
	cd contracts/governance && cargo build --target wasm32-unknown-unknown --release
	cd contracts/registry && cargo build --target wasm32-unknown-unknown --release
	@echo "Optimizing WASM..."
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/treasury.wasm
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/governance.wasm
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/registry.wasm

test: test-unit test-integration

test-unit:
	@echo "Running contract tests..."
	cd contracts && cargo test
	@echo "Running frontend tests..."
	cd frontend && npm test
	@echo "Running backend tests..."
	cd backend && npm test

test-integration:
	@echo "Running integration tests..."
	cd tests && cargo test --test integration_tests

test-security:
	@echo "Running security tests..."
	cd tests && cargo test --test security_tests

coverage:
	@echo "Generating coverage report..."
	cd contracts && cargo tarpaulin --out Html --output-dir ../coverage

dev:
	@echo "Starting local development environment..."
	docker-compose up -d postgres
	cd backend && npm run dev &
	cd frontend && npm run dev

deploy-testnet:
	@echo "Deploying to Stellar testnet..."
	./scripts/deploy-testnet.sh

deploy-mainnet:
	@echo "Deploying to Stellar mainnet..."
	@echo "⚠️  WARNING: Deploying to MAINNET"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		./scripts/deploy-mainnet.sh; \
	fi

clean:
	@echo "Cleaning build artifacts..."
	cd contracts && cargo clean
	cd frontend && rm -rf .next node_modules
	cd backend && rm -rf dist node_modules
	rm -rf coverage/
