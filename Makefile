# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.

.DEFAULT_GOAL := help

CARGO ?= cargo
DOCKER_COMPOSE ?= docker compose

##@ General

.PHONY: help
help: ## Show this help message
	@echo ""
	@echo "Usage: make [target]"
	@awk 'BEGIN {FS = ":.*?## "} \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } \
		/^[a-zA-Z_-]+:.*?## / { printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@echo ""

##@ Development

.PHONY: run build check clean
run: ## Run the application locally
	$(CARGO) run

build: ## Build release binary
	$(CARGO) build --release

check: ## Fast compile-check without code generation
	$(CARGO) check

clean: ## Remove build artifacts
	$(CARGO) clean

.PHONY: fmt clippy test lint
fmt: ## Format Rust code with rustfmt
	$(CARGO) fmt

clippy: ## Run Clippy linter
	$(CARGO) clippy -- -D warnings

test: ## Run unit and integration tests
	$(CARGO) test

lint: fmt clippy ## Run formatting and clippy linting

.PHONY: css
css: ## Bundle and minify CSS with lightningcss
	lightningcss --minify --bundle --targets ">= 0.25%" assets/css/app.css -o static/app.min.css

.PHONY: sqlx-prepare
sqlx-prepare: ## Prepare offline query metadata for SQLx
	$(CARGO) sqlx prepare

##@ Docker

.PHONY: docker-up docker-down docker-logs docker-restart docker-build docker-scout
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

docker-scout: ## Run Docker Scout vulnerability scan and export SARIF
	./docker/scout.sh

.PHONY: scout
scout: docker-scout ## Alias for docker-scout
