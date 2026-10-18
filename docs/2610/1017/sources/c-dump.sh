#!/bin/bash
# usage: c-dump.sh <name> <url>  -- anonymous headless Chrome DOM dump (fresh profile, no cookies)
CH="/c/Program Files/Google/Chrome/Application/chrome.exe"
TMP=$(cygpath -w "$(mktemp -d)")
timeout 100 "$CH" --headless=new --user-data-dir="$TMP" --disable-gpu --timeout=30000 --window-size=1000,2000 --lang=en-US --accept-lang=en-US --dump-dom "$2" > "c-$1.html" 2>/dev/null
echo "$1 $(date -u +%H:%M:%S)Z $(wc -c < c-$1.html)"
