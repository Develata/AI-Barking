#!/bin/zsh
# usage: b-opencli-dump.sh <session> <url> <out.txt> [wait_s]
# 用 opencli 打开页面，等待渲染后把 document.body.innerText 写入 <out.txt>（开头写入 URL 与北京时间）
s=$1; u=$2; o=$3; w=${4:-6}
opencli browser --session "$s" open "$u" --window background >/dev/null 2>&1
sleep "$w"
{ echo "URL: $u"; echo "Captured (Beijing time): $(date '+%Y-%m-%d %H:%M:%S %z')"; echo "---"; opencli browser --session "$s" eval "document.body.innerText" 2>/dev/null | python3 -I -c 'import json,sys; d=json.load(sys.stdin); print(d["data"])'; } > "$o"
wc -c "$o"
