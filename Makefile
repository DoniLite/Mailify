.DEFAULT_GOAL := help

TPL_DIR := templates-parser
COMPOSE := docker compose

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage: make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

# ── Setup ─────────────────────────────────────────────────────────
.PHONY: setup
setup: ## Install template deps (bun)
	cd $(TPL_DIR) && bun install

# ── Templates ─────────────────────────────────────────────────────
.PHONY: gen
gen: ## Regenerate .tsx + sidecar files from templates.config.ts
	cd $(TPL_DIR) && bun run generate

.PHONY: build-templates
build-templates: ## Compile React Email → HTML into templates-parser/out
	cd $(TPL_DIR) && bun run build

.PHONY: dev-templates
dev-templates: ## React Email preview server on :3000
	cd $(TPL_DIR) && bun run dev

# ── Rust ──────────────────────────────────────────────────────────
.PHONY: dev
dev: ## Run the server (cargo run) — assumes postgres+mailpit already up
	cargo run --bin mailify

.PHONY: build
build: ## Release build
	cargo build --release --bin mailify

.PHONY: test
test: ## Run workspace tests
	cargo test --workspace

.PHONY: check
check: ## cargo check all targets
	cargo check --workspace --all-targets

.PHONY: clippy
clippy: ## Clippy, warnings = errors
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: fmt
fmt: ## Format all crates
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Verify formatting without writing
	cargo fmt --all -- --check

.PHONY: ci
ci: fmt-check clippy test ## Run the same checks as CI

# ── Docker ────────────────────────────────────────────────────────
.PHONY: up
up: ## Start full stack (postgres + mailpit + mailify) in detached mode
	$(COMPOSE) up -d --build

.PHONY: up-deps
up-deps: ## Start only postgres + mailpit (for local Rust dev)
	$(COMPOSE) up -d postgres mailpit

.PHONY: down
down: ## Stop stack
	$(COMPOSE) down

.PHONY: down-volumes
down-volumes: ## Stop stack AND drop postgres volume (wipes queue data)
	$(COMPOSE) down -v

.PHONY: logs
logs: ## Tail mailify container logs
	$(COMPOSE) logs -f mailify

.PHONY: ps
ps: ## Show stack status
	$(COMPOSE) ps

.PHONY: docker-build
docker-build: ## Build docker image locally (no push)
	docker build -t mailify:local -f docker/Dockerfile .

# ── Ops helpers ───────────────────────────────────────────────────
KEY ?=
ID ?= key
.PHONY: hash-key
hash-key: ## Argon2-hash an API key. Usage: make hash-key KEY=my-secret ID=web
	@test -n "$(KEY)" || { echo "KEY=<plaintext> required"; exit 1; }
	@cargo run --quiet -p mailify-auth --example hash-key -- "$(KEY)" "$(ID)"

SUBJECT ?= dev
SCOPES ?=
.PHONY: issue-token
issue-token: ## Mint a JWT offline using the server's JWT secret. Usage: make issue-token SUBJECT=dev SCOPES=mail:send
	@cargo run --quiet -p mailify-auth --example issue-token -- "$(SUBJECT)" "$(SCOPES)"

.PHONY: openapi
openapi: ## Dump the OpenAPI spec to openapi.json (server must be running)
	curl -s http://localhost:8080/api-docs/openapi.json | jq . > openapi.json
	@echo "→ openapi.json"

# ── Clean ─────────────────────────────────────────────────────────
.PHONY: clean
clean: ## cargo clean + remove compiled templates
	cargo clean
	rm -rf $(TPL_DIR)/out $(TPL_DIR)/.react-email
