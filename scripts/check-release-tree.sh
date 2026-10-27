#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"
failures=0

while IFS= read -r path; do
  case "$path" in
    .git/*|target/*) continue ;;
    .beads/*|.codex/*|.claude/*|.agent-state/*|.session-state/*)
      printf 'private tooling state: %s\n' "$path" >&2
      failures=1
      ;;
    .env|.env.*|*.pem|*.key|*credentials*|*secret*)
      printf 'private configuration or credential-like file: %s\n' "$path" >&2
      failures=1
      ;;
  esac
  if [[ -d "$path" ]]; then
    continue
  fi
  # Exclude this checker because its split-up pattern below intentionally names
  # the path forms it is looking for.
  [[ "$path" == scripts/check-release-tree.sh ]] && continue
  local_path_re='/Users/|/home/|/work/|/tmp/|/Volumes/|/mnt/|(^|[^[:alnum:]])[A-Za-z]:[/\\]'
  if rg -n -I "$local_path_re" -- "$path" >/dev/null 2>&1; then
    printf 'local path reference: %s\n' "$path" >&2
    failures=1
  fi
  if rg -n -I 'carbon|phosphorus' -- "$path" >/dev/null 2>&1; then
    printf 'machine-specific hostname reference: %s\n' "$path" >&2
    failures=1
  fi
done < <(git ls-files --cached --others --exclude-standard)

if (( failures )); then
  exit 1
fi
printf 'release-tree hygiene: clean\n'
