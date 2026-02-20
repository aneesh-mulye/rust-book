#!/usr/bin/env bash
set -u
set -o pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

TIERS=(
  "tier_1_struct_basics"
  "tier_2_methods_impl"
  "tier_3_multi_structs"
  "tier_4_ownership_and_structs"
  "tier_5_integration"
)

failed_tiers=()

for tier in "${TIERS[@]}"; do
  tier_dir="${SCRIPT_DIR}/${tier}"

  echo
  echo "=== ${tier} ==="

  if [[ ! -d "${tier_dir}" ]]; then
    echo "Missing directory: ${tier_dir}"
    failed_tiers+=("${tier} (missing directory)")
    continue
  fi

  if (cd "${tier_dir}" && cargo test "$@"); then
    echo "Result: PASS (${tier})"
  else
    echo "Result: FAIL (${tier})"
    failed_tiers+=("${tier}")
  fi
done

echo
if [[ ${#failed_tiers[@]} -gt 0 ]]; then
  echo "Tier test run completed with failures in:"
  for tier in "${failed_tiers[@]}"; do
    echo "- ${tier}"
  done
  exit 1
fi

echo "Tier test run completed successfully."
