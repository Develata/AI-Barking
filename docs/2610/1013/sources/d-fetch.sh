#!/bin/sh
# 1013 D 组抓取：d-fetch.sh <name> <url>  -> d-<name>.html / .headers，HTML 经 d-html2txt.py 转 d-<name>.txt
name="$1"; url="$2"
UA="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36"
echo "start_utc $(date -u +%Y-%m-%dT%H:%M:%SZ) $url"
curl -sS -L --max-time 60 -A "$UA" -D "d-$name.headers" -o "d-$name.html" -w "%{http_code} %{size_download}B %{url_effective}\n" "$url"
case "$(grep -i '^content-type' "d-$name.headers" | tail -1)" in
  *html*) python -I d-html2txt.py "d-$name.html" "d-$name.txt" ;;
esac
