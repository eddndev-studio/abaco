.PHONY: dev dev-api dev-web build setup deploy

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

# Setup
setup:
	cd apps/web && npm install
	cd apps/api && cargo check

# Deploy to vps2
deploy: build
	scp apps/api/target/release/abaco-api vps2:/opt/abaco/abaco-api
	rsync -a apps/web/build/ vps2:/opt/abaco/web/build/
	ssh vps2 "sudo systemctl restart abaco-api abaco-web"
