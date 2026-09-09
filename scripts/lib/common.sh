#!/usr/bin/env bash
# Common shared functions for EphemeralAct scripts
set -euo pipefail

coverage_json_exists() {
  [ -f cov.json ]
}

abort_if_coverage_json_is_missing() {
  if ! coverage_json_exists; then
    echo "ERROR: cov.json not found. cargo-llvm-cov failed to produce JSON output." >&2
    exit 1
  fi
}

extract_coverage_totals_from_json() {
  local totals
  totals=$(coverage_rows_from_json | awk -F'\t' '
    {
      lines_count += $2
      lines_covered += $2 - $3
      functions_count += $7
      functions_covered += $8
      regions_count += $9
      regions_covered += $10
    }
    END {
      printf "%d\t%d\t%d\t%d\t%d\t%d\n", \
        lines_count, lines_covered, functions_count, functions_covered, \
        regions_count, regions_covered
    }
  ')
  IFS=$'\t' read -r lines_count lines_covered functions_count \
    functions_covered regions_count regions_covered <<< "$totals"

  lines_count=${lines_count:-0}
  lines_covered=${lines_covered:-0}
  functions_count=${functions_count:-0}
  functions_covered=${functions_covered:-0}
  regions_count=${regions_count:-0}
  regions_covered=${regions_covered:-0}
  lines_percent=$(awk -v covered="$lines_covered" -v count="$lines_count" \
    'BEGIN { if (count > 0) printf "%.1f", covered / count * 100; else print "0" }')
  functions_percent=$(awk -v covered="$functions_covered" -v count="$functions_count" \
    'BEGIN { if (count > 0) printf "%.1f", covered / count * 100; else print "0" }')
  regions_percent=$(awk -v covered="$regions_covered" -v count="$regions_count" \
    'BEGIN { if (count > 0) printf "%.1f", covered / count * 100; else print "0" }')

  export lines_count lines_covered lines_percent functions_percent regions_percent
}

normalize_path() {
  local path="$1"
  case "$path" in
    */src/*) printf 'src/%s\n' "${path##*/src/}" ;;
    */tests/*) printf 'tests/%s\n' "${path##*/tests/}" ;;
    src/*|tests/*) printf '%s\n' "$path" ;;
    *) printf '%s\n' "$path" ;;
  esac
}

coverage_rows_from_json() {
  jq -r '
    def missing_lines:
      .segments as $segs
      | [
          range(0; $segs | length)
          | . as $i
          | $segs[$i]
          | select(.[2] == 0 and .[3] == false)
          | { start: .[0], end: ($segs[$i + 1] // .[0:1])[0] }
        ]
      | group_by(.start)
      | map(.[0])
      | map(
          if .start == .end then (.start | tostring)
          else "\(.start)-\(.end)"
          end
        )
      | join(", ");
    .data[0].files[]
    | select(.filename | test("(^|/)src/"))
    | select(.summary.lines.count > 0)
    | [
        .filename,
        (.summary.lines.count | tostring),
        ((.summary.lines.count - .summary.lines.covered) | tostring),
        ((.summary.lines.covered / .summary.lines.count * 100) | tostring),
        (.summary.functions.count | tostring),
        (.summary.functions.covered | tostring),
        (.summary.regions.count | tostring),
        (.summary.regions.covered | tostring),
        (if .summary.lines.count > .summary.lines.covered then missing_lines else "" end)
      ]
    | @tsv
  ' cov.json | while IFS=$'\t' read -r raw_path stmts miss pct function_count \
    function_covered region_count region_covered missing_lines; do
    local norm
    norm=$(normalize_path "$raw_path")
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$norm" "$stmts" "$miss" "$pct" "$raw_path" "$missing_lines" \
      "$function_count" "$function_covered" "$region_count" "$region_covered"
  done | awk -F'\t' '
    !($1 in best_miss) || $3 + 0 < best_miss[$1] {
      best_miss[$1] = $3 + 0
      best_row[$1] = $0
    }
    END {
      for (path in best_row) print best_row[path]
    }
  '
}

print_coverage_table() {
  extract_coverage_totals_from_json

  printf "\n"

  local tmp_rows
  tmp_rows=$(mktemp)

  coverage_rows_from_json | sort > "$tmp_rows"

  local max_name_len
  max_name_len=$(awk -F'\t' '{print length($1)}' "$tmp_rows" | sort -n | tail -1)
  local name_col=$(( max_name_len > 4 ? max_name_len : 4 ))

  local max_missing_len=7
  while IFS=$'\t' read -r norm stmts miss pct raw_path missing_lines \
    function_count function_covered region_count region_covered; do
    if [ "$miss" -gt 0 ]; then
      local mlen=${#missing_lines}
      [ "$mlen" -gt "$max_missing_len" ] && max_missing_len=$mlen
    fi
  done < "$tmp_rows"

  if [ "$max_missing_len" -gt 80 ]; then
    max_missing_len=80
  fi

  local sep
  sep=$(printf '%*s' $(( name_col + 28 + max_missing_len )) '' | tr ' ' '-')

  printf "%-${name_col}s  %6s  %4s  %6s  %-${max_missing_len}s\n" \
    "Name" "Stmts" "Miss" "Cover" "Missing"
  echo "$sep"

  while IFS=$'\t' read -r norm stmts miss pct raw_path missing_lines \
    function_count function_covered region_count region_covered; do
    if [ "$miss" -gt 0 ] && [ "${#missing_lines}" -gt "$max_missing_len" ]; then
      missing_lines="${missing_lines:0:$((max_missing_len-4))} ..."
    fi
    printf "%-${name_col}s  %6s  %4s  %5.1f%%  %-${max_missing_len}s\n" \
      "$norm" "$stmts" "$miss" "$pct" "$missing_lines"
  done < "$tmp_rows"

  echo "$sep"
  printf "%-${name_col}s  %6s  %4s  %5.1f%%\n" \
    "TOTAL" "$lines_count" "$(( lines_count - lines_covered ))" "$lines_percent"

  printf "\n"
  printf "  Functions: %.1f%%\n" "$functions_percent"
  printf "  Regions:   %.1f%%\n" "$regions_percent"

  rm -f "$tmp_rows"
}

abort_if_line_coverage_is_below_threshold() {
  extract_coverage_totals_from_json
  local threshold="$1"
  local passes
  passes=$(awk -v p="$lines_percent" -v t="$threshold" \
    'BEGIN{ if (p+0 >= t+0) print 1; else print 0 }')
  if [ "$passes" -eq 1 ]; then
    printf "Coverage check: PASS (>= %s%%)\n" "$threshold"
  else
    printf "Coverage check: FAIL (< %s%%)\n" "$threshold"
    exit 1
  fi
}
