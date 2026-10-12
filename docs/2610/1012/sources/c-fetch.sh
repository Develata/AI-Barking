#!/bin/bash
# 1012 C 组：curl 抓取并转文本。用法：bash c-fetch.sh <name> <url>  → c-<name>.html/.headers/.txt，打印状态与大小
n=$1; u=$2
curl -s -L -m 60 -A "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/130 Safari/537.36" -D c-$n.headers -o c-$n.html "$u"
python c-html2txt.py c-$n.html c-$n.txt
echo "$n $(grep -a -m1 -E '^HTTP/[0-9.]+ [0-9]+ ' c-$n.headers | tail -1 | tr -d '\r') final:$(grep -a -E '^HTTP/' c-$n.headers | tail -1 | tr -d '\r') html:$(wc -c < c-$n.html) txt:$(wc -c < c-$n.txt) $(date -u +%FT%TZ)"
