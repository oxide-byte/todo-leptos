#!/usr/bin/env sh
set -euo pipefail

# Ensure we are at project root when running inside container
cd "$(dirname "$0")/.."
sleep 5m
# Run SonarScanner with provided settings
exec /usr/bin/sonar-scanner \
  -Dproject.settings=docker-build/sonar-project.properties