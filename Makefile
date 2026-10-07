.DEFAULT_GOAL := help

CARGO ?= cargo
DOCKER_COMPOSE ?= docker compose

.PHONY: help
help: ## Show this help message
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""

.PHONY: run dev build check
run: ## Run the application locally
	$(CARGO) run

dev: ## Run with automatic reload (requires cargo-watch)
	$(CARGO) watch -x run

watch: dev

.PHONY: css
css: ## Bundle and minify CSS with lightningcss
	lightningcss --minify --bundle --targets ">= 0.25%" assets/css/app.css -o static/app.min.css

build: ## Build release binary
	$(CARGO) build --release

check: ## Fast compile-check without code generation
	$(CARGO) check

.PHONY: sqlx-prepare
sqlx-prepare: ## Prepare offline query metadata for SQLx
	$(CARGO) sqlx prepare

.PHONY: docker-up docker-down docker-logs docker-restart docker-build
docker-up: css ## Start Docker Compose services in the background
	$(DOCKER_COMPOSE) up --build -d

docker-down: ## Stop and remove Docker Compose containers
	$(DOCKER_COMPOSE) down

docker-logs: ## Follow Docker Compose logs
	$(DOCKER_COMPOSE) logs -f

docker-restart: ## Restart Docker Compose services
	$(DOCKER_COMPOSE) restart

docker-build: css ## Build Docker images without running
	$(DOCKER_COMPOSE) build

.PHONY: fmt clippy test lint
fmt: ## Format Rust code with rustfmt
	$(CARGO) fmt

clippy: ## Run Clippy linter
	$(CARGO) clippy -- -D warnings

test: ## Run unit and integration tests
	$(CARGO) test

lint: fmt clippy ## Run formatting and clippy linting

.PHONY: clean
clean: ## Remove build artifacts
	$(CARGO) clean
