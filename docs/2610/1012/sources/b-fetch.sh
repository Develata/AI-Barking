#!/bin/sh
# 1013 B 组：批量 curl（浏览器 UA，匿名）并转文本。用法：sh b-fetch.sh <name> <url>  → b-m-<name>.{headers,html,txt}
UA="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36"
n=$1; u=$2
printf '%s\t%s\t' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$u"
curl -sS -L -m 60 -A "$UA" -D "b-m-$n.headers" -o "b-m-$n.html" -w "%{http_code}\t%{size_download}\n" "$u" 2>&1
python -I b-html2txt.py "b-m-$n.html" "b-m-$n.txt" 2>/dev/null
