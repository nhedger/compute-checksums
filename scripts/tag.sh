#!/usr/bin/env bash

set -euo pipefail

VERSION="${1:-}"

# Strip leading 'v' if present
VERSION="${VERSION#v}"

# Validate version format: MAJOR.MINOR.PATCH (numeric only)
if [[ -z "$VERSION" || ! "$VERSION" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
  echo "Invalid version provided: $VERSION" >&2
  echo "Version must be a valid semver version in the format of MAJOR.MINOR.PATCH"
  exit 1
fi

MAJOR="${BASH_REMATCH[1]}"
MINOR="${BASH_REMATCH[2]}"
PATCH="${BASH_REMATCH[3]}"

TAGS=(
  "v${MAJOR}"
  "v${MAJOR}.${MINOR}"
  "v${MAJOR}.${MINOR}.${PATCH}"
)

echo "Creating tags..."
for TAG in "${TAGS[@]}"; do
  git tag "$TAG" --force -s -m "chore: tag $TAG"
  echo "- Created tag: $TAG"
done

echo "Tags created successfully. Run \`git push --tags --force\` to push them to the remote."