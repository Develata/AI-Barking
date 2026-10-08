#!/usr/bin/env bash
# usage: d-dumpdom.sh <name> <url> [budget_ms]  -- headless Chrome (anonymous temp profile) rendered DOM -> <name>.html + record
set -u
cd "$(dirname "$0")"
name="$1"; url="$2"; budget="${3:-8000}"
case "$name" in d-*) ;; *) echo "d- prefix required"; exit 2;; esac
prof=$(mktemp -d)
tu=$(date -u +%Y-%m-%dT%H:%M:%SZ); t=$(date -u -d "+8 hours" +%Y-%m-%dT%H:%M:%S+08:00)
"/c/Program Files/Google/Chrome/Application/chrome.exe" --headless=new --disable-gpu --user-data-dir="$(cygpath -w "$prof")" --timeout=$budget --dump-dom "$url" > "$name.html" 2> "$name.stderr"
rc=$?
sz=$(wc -c < "$name.html")
printf '{"time_utc":"%s","time_bj":"%s","tool":"headless chrome --dump-dom (anonymous temp profile)","url":"%s","exit":%s,"bytes":%s,"file":"%s.html"}\n' "$tu" "$t" "$url" "$rc" "$sz" "$name" | tee -a d-http-records.jsonl
rm -f "$name.stderr"
