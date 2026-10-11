# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.0-alpha] - 2026-10-11
Link : [Github Release](https://github.com/justpranavrs/filebase/releases/tag/v0.1.0-alpha)

### Added
- Document storage engine with SHA-256 integrity verification and ETag caching.
- Hierarchical folder navigation with directory creation, file deletion, and folder moves.
- Server-rendered Askama templates paired with HTMX for responsive DOM swaps.
- Asset build pipeline using Lightning CSS, font subsetting via `pyftsubset`, and external SVG icon sprites for browser caching.
- Embedded SQLite metadata store running in WAL mode with compile-time SQLx query caching (`.sqlx/`).
- Automated backup worker (`filebase-backup`) with non-blocking database snapshots and configurable retention pruning.
- Multi-stage Docker builds targeting a static Debian distroless runtime (`gcr.io/distroless/static-debian13:nonroot`).
- Docker Scout container vulnerability scanning with SARIF reporting and severity gating.
- GitHub Actions CI/CD workflows for tests, linting, formatting, and Docker Hub image publishing with SBOM and SLSA provenance attestations.

##
* [0.1.0-alpha] : [Initial Release](https://github.com/justpranavrs/filebase/commit/cdd7082a7783546426d327cf9c0e554d1899cc18)
