.PHONY: dev dev-api dev-web build db

# Development
dev:
	$(MAKE) -j2 dev-api dev-web

dev-api:
	cd apps/api && cargo run

dev-web:
	cd apps/web && npm run dev

# Build
build-api:
	cd apps/api && cargo build --release

build-web:
	cd apps/web && npm run build

build: build-api build-web

# Database
db:
	docker compose up -d db

db-down:
	docker compose down

# Setup
setup:
	cd apps/web && npm install
	cd apps/api && cargo check
