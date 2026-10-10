#!/bin/sh
# Prints the Homebrew formula for a release to stdout.
# Usage: scripts/render-formula.sh VERSION SHA256 OWNER/REPO
set -eu

version="${1:?usage: scripts/render-formula.sh VERSION SHA256 OWNER/REPO}"
sha256="${2:?usage: scripts/render-formula.sh VERSION SHA256 OWNER/REPO}"
repo="${3:?usage: scripts/render-formula.sh VERSION SHA256 OWNER/REPO}"

sed -e "s|@VERSION@|$version|g" -e "s|@SHA256@|$sha256|g" -e "s|@REPO@|$repo|g" \
  "$(dirname "$0")/../packaging/homebrew/dockjail.rb.in"
