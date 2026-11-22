#!/bin/bash
# Check every scheme in large/, or in the folder named as the argument (compressed JSON, catalog layout with sparse rows).
#   tools/check_large.sh              # tools/check_json.py: random matrices modulo four primes (standard library)
#   EXACT=1 tools/check_large.sh      # also tools/kronexact: every Brent equation in exact integers
#                                     # (build once: cargo build --release --manifest-path tools/kronexact/Cargo.toml)
#   tools/check_large.sh products     # the same for products/
cd "$(dirname "$0")/.." || exit 1
dir=${1:-large}
fail=0
for f in "$dir"/*/*x*x*_m*.json.gz; do
  python3 tools/check_json.py "$f" 2 || fail=1
  if [ -n "${EXACT:-}" ]; then
    tools/kronexact/target/release/kronexact check "$f" | grep -E 'Brent equations|changed|RESULT' | sed 's/^/  /'
    [ "${PIPESTATUS[0]}" -eq 0 ] || fail=1
  fi
done
name=$(echo "$dir" | tr a-z A-Z)
[ $fail -eq 0 ] && echo "ALL $name SCHEMES PASS" || { echo "SOME $name SCHEME FAILED"; exit 1; }
