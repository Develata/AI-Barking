#!/usr/bin/env bash
# usage: d-fetch.sh <name> <url> [ext]   (name must start with d-)
# Saves the response body to <name>.<ext> (default html) and appends a record to d-http-records.jsonl.
# Beijing time = UTC+8.
set -u
cd "$(dirname "$0")"
name="$1"; url="$2"; ext="${3:-html}"
case "$name" in d-*) ;; *) echo "d- prefix required"; exit 2;; esac
UA='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36'
tu=$(date -u +%Y-%m-%dT%H:%M:%SZ); t=$(date -u -d "+8 hours" +%Y-%m-%dT%H:%M:%S+08:00)
meta=$(curl -sL -A "$UA" -m 60 -o "$name.$ext" -w '%{http_code}\t%{url_effective}\t%{size_download}\t%{content_type}' "$url")
code=$(printf '%s' "$meta" | cut -f1); eff=$(printf '%s' "$meta" | cut -f2); sz=$(printf '%s' "$meta" | cut -f3); ct=$(printf '%s' "$meta" | cut -f4)
printf '{"time_utc":"%s","time_bj":"%s","tool":"curl","url":"%s","http":"%s","final_url":"%s","bytes":%s,"content_type":"%s","file":"%s.%s"}\n' "$tu" "$t" "$url" "$code" "$eff" "$sz" "$ct" "$name" "$ext" | tee -a d-http-records.jsonl
