#!/usr/bin/env bash
# Regenerate src/generated from spec/openapi.yml with the pinned Progenitor.
#
# Environment:
#   RUSTFMT            rustfmt binary for Progenitor (default: nightly rustfmt;
#                      Progenitor relies on unstable rustfmt options)
#   PROGENITOR         prebuilt cargo-progenitor binary (default: install the
#                      pinned revision into PROGENITOR_ROOT)
#   PROGENITOR_ROOT    install root reused across runs
#                      (default: ${XDG_CACHE_HOME:-$HOME/.cache}/ubicloud-rs/progenitor-<rev>)
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

generate() {
  cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -p generate -- "$@"
}

progenitor_rev="$(generate spec-metadata "$repo_root/Cargo.toml" progenitor_revision)"
generate prepare-spec "$repo_root/spec/openapi.yml" "$tmp_dir/openapi.yml"

rustfmt_path="${RUSTFMT:-$(rustup which --toolchain nightly rustfmt)}"
progenitor="${PROGENITOR:-}"
if [[ -z "$progenitor" ]]; then
  progenitor_root="${PROGENITOR_ROOT:-${XDG_CACHE_HOME:-$HOME/.cache}/ubicloud-rs/progenitor-$progenitor_rev}"
  progenitor="$progenitor_root/bin/cargo-progenitor"
  if [[ ! -x "$progenitor" ]]; then
    cargo install --quiet --locked --root "$progenitor_root" \
      --git https://github.com/oxidecomputer/progenitor.git \
      --rev "$progenitor_rev" cargo-progenitor
  fi
fi

RUSTFMT="$rustfmt_path" "$progenitor" progenitor >"$tmp_dir/progenitor.log" \
  --input "$tmp_dir/openapi.yml" \
  --output "$tmp_dir/generated" \
  --name ubicloud-rs \
  --version 0.1.0 \
  --license-name AGPL-3.0 \
  --interface positional \
  --tags merged \
  --include-client true || { cat "$tmp_dir/progenitor.log" >&2; exit 1; }
mkdir -p "$repo_root/src/generated"
cp "$tmp_dir/generated/src/lib.rs" "$repo_root/src/generated/mod.rs"
cp "$tmp_dir/generated/src/progenitor_client.rs" \
  "$repo_root/src/generated/progenitor_client.rs"
generate patch-generated "$repo_root/src/generated/mod.rs"
# Progenitor formats with nightly rustfmt defaults; normalize to the crate's
# edition-2024 style so `cargo fmt --check` passes on the generated sources.
rustfmt --edition 2024 "$repo_root/src/generated/mod.rs"
