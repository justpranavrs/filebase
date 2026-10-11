# Security Policy

The Filebase project takes the security of stored records and user data seriously. This document outlines our supported versions and responsible disclosure process.

---

## Supported Versions

Security updates and vulnerability patches are provided only for the latest released minor version.

| Version | Supported |
| :--- | :--- |
| `0.1.x` | Yes |
| `< 0.1.0` | No |

We strongly recommend keeping Filebase up-to-date with the latest release.

---

## Reporting a Vulnerability

If you discover a security vulnerability or potential threat in Filebase, please report it privately:

### 1. GitHub Security Advisories (Preferred)

Use GitHub's built-in [Security Advisories](https://github.com/justpranavrs/filebase/security/advisories/new) feature:
1. Navigate to the **Security** tab of this repository.
2. Under **Reporting**, click **"Report a vulnerability"**.
3. Fill out the advisory form with relevant technical details.

This creates a confidential advisory workspace accessible only to the project maintainers and the reporter.

### 2. Public Issues & Pull Requests

> **Warning**: **Do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.**
> Publicly disclosing an unpatched vulnerability exposes existing deployments to risk before a mitigation is available.

---

## What to Include in a Report

To help us evaluate and address the issue efficiently, please include:
- A clear description of the vulnerability and its potential security impact.
- Affected components (e.g. storage layer, database, file upload handlers).
- Detailed, step-by-step instructions or a minimal proof-of-concept (PoC) to reproduce the behavior.
- Any suggested remediations, mitigations, or patches if available.

Reports generated purely by automated scanners without a demonstrable exploit scenario or reproducible impact are discouraged.

---

## Evaluation and Disclosure Process

Once a vulnerability report is submitted:

1. **Review**: Maintainers will review the report and attempt to reproduce the reported behavior in a private environment.
2. **Remediation**: If confirmed, a fix will be developed and tested in a private repository fork.
3. **Release & Advisory**: A patched release will be published along with a public GitHub Security Advisory detailing the issue, affected versions, and credit to the reporter (unless anonymity is requested).

We ask that you maintain confidentiality and give maintainers an opportunity to publish a fix before sharing details publicly.
