#!/bin/bash
# Check every scheme in large/ (compressed JSON, catalog layout with sparse rows).
#   tools/check_large.sh              # tools/check_json.py: random matrices modulo four primes (standard library)
#   EXACT=1 tools/check_large.sh      # also tools/kronexact: every Brent equation in exact integers
#                                     # (build once: cargo build --release --manifest-path tools/kronexact/Cargo.toml)
cd "$(dirname "$0")/.." || exit 1
fail=0
for f in large/*/*_Q.json.gz; do
  python3 tools/check_json.py "$f" 2 || fail=1
  if [ -n "${EXACT:-}" ]; then
    tools/kronexact/target/release/kronexact check "$f" | grep -E 'Brent equations|changed|RESULT' | sed 's/^/  /'
    [ "${PIPESTATUS[0]}" -eq 0 ] || fail=1
  fi
done
[ $fail -eq 0 ] && echo "ALL LARGE SCHEMES PASS" || { echo "SOME LARGE SCHEME FAILED"; exit 1; }
