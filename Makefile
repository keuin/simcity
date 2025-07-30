.PHONY: clean backend frontend dev
.ONESHELL:

build: backend frontend

clean:
	rm -rf target || true
	rm -rf frontend/dist || true

backend:
	cargo build --release

frontend:
	cd frontend
	pnpm i
	pnpm run build

dev: frontend
	RUST_LOG=info exec cargo run