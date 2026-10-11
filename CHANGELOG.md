# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Publishing container images to GitHub Container Registry (`ghcr.io`) alongside Docker Hub.
- Automated security analysis with CodeQL Advanced and Dependabot dependency management.

### Security
- Hardened CI/CD supply chain by pinning GitHub Actions to immutable commit SHAs and enforcing least-privilege workflow permissions.

### Planned
- Stale directory view renders and back/forward browser navigation desync via `Cache-Control` headers and HTMX cache settings.
- Streaming file uploads and downloads using `tokio::fs` and `ReaderStream`.
- ZIP folder downloads with on-the-fly streaming archive generation.
- Recursive folder uploads preserving directory hierarchies.
- Soft deletes and trash bin with recovery window.
- Multi-field sorting by name, date modified, and size.
- Storage quota and disk usage breakdown widget.
- Temporary public share links with configurable expiration and access limits.
- Document expiry tracking with automated alert notifications via WhatsApp and Telegram.

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
* [Unreleased]: https://github.com/justpranavrs/filebase/compare/v0.1.0-alpha...HEAD
* [0.1.0-alpha] : [Initial Release](https://github.com/justpranavrs/filebase/commit/cdd7082a7783546426d327cf9c0e554d1899cc18)
