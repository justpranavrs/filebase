#!/bin/sh
# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.

set -eu

# Scout Configuration Variables
IMAGE="${IMAGE:-filebase:latest}"
OUT="${SARIF_FILE:-docker/scout.sarif}"

# Build the local container image
docker build -t "$IMAGE" .

# Display quick overview of image and base image vulnerabilities
docker scout quickview "local://$IMAGE"

# Display remediation recommendations and base image updates
docker scout recommendations "local://$IMAGE"

# Export fixable critical and high CVEs as a SARIF report
docker scout cves \
    --only-severity critical,high \
    --only-fixed \
    --format sarif \
    --output "$OUT" \
    "local://$IMAGE"

# Exit with non-zero status if fixable critical or high CVEs are found
docker scout cves \
    --only-severity critical,high \
    --only-fixed \
    --exit-code \
    "local://$IMAGE"
