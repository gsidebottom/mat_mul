#!/bin/bash
# Check every scheme in schemes/ with tools/check_scheme.py (standard library only) and, if PLINOPT is set to a
# built PLinOpt checkout, with PLinOpt's own tools: SLPchecker on every program against its matrix, MMchecker on
# every L/R/P triple, ALTchecker on the alternative-basis files.
#   tools/check_all.sh                       # our checker only
#   PLINOPT=~/src/plinopt tools/check_all.sh # both
cd "$(dirname "$0")/.." || exit 1
fail=0
for d in schemes/*/; do
  for L in "$d"*_L.sms; do
    prefix=${L%_L.sms}
    case "$prefix" in *-ALT|*-CoB) continue ;; esac
    opts=()
    case "$prefix" in *mod_2*) opts+=(--mod 2) ;; esac
    [ -e "${prefix}-ALT_L.sms" ] && opts+=(--alt)
    python3 tools/check_scheme.py "${opts[@]}" "$prefix" || fail=1
    if [ -n "${PLINOPT:-}" ]; then
      q=(); mm=(-b 64); case "$prefix" in *mod_2*) q=(-q 2); mm=(-q 2) ;; esac
      for f in "$prefix"*.slp; do
        sms=${f%.slp}.sms
        printf '  SLPchecker %s: ' "${f##*/}"
        "$PLINOPT/bin/SLPchecker" "${q[@]}" -M "$sms" "$f" 2>&1 | grep -oE 'SUCCESS|ERROR|FAIL' | head -1
      done
      for i in 1 2 3; do
        printf '  MMchecker run %s: ' "$i"
        "$PLINOPT/bin/MMchecker" "${mm[@]}" "${prefix}_L.sms" "${prefix}_R.sms" "${prefix}_P.sms" 2>&1 | grep -oE 'SUCCESS|ERROR|FAIL' | head -1
      done
      if [ -e "${prefix}-ALT_L.sms" ]; then
        for s in L R; do printf '  ALTchecker CoB+ALT %s: ' $s; "$PLINOPT/bin/ALTchecker.sh" "${prefix}-CoB_$s.slp" "${prefix}-ALT_$s.slp" -M "${prefix}_$s.sms" 2>&1 | grep -oE 'SUCCESS|ERROR|FAIL' | tail -1; done
        printf '  ALTchecker ALT+CoB P: '; "$PLINOPT/bin/ALTchecker.sh" "${prefix}-ALT_P.slp" "${prefix}-CoB_P.slp" -M "${prefix}_P.sms" 2>&1 | grep -oE 'SUCCESS|ERROR|FAIL' | tail -1
      fi
    fi
  done
done
echo "ALL: $([ $fail = 0 ] && echo PASS || echo FAIL)"
exit $fail
