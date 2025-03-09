#!/bin/bash
set -e

# Output the target dir used by 'cargo'.
#
# This can be overridden in many ways, and the right way to query is from cargo, itself.
# Normally, the folder is './target/' but e.g. in virtualization it might be beneficial to use a global target dir.
#
# Requires:
#   - jq
#
cargo metadata --format-version=1 | jq .target_directory --raw-output
