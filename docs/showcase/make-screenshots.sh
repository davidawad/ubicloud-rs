#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$repo_root"
cargo run --quiet --example mock_create_vm -- --svg docs/showcase/create-vm.svg
