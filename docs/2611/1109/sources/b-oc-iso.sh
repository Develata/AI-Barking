#!/bin/zsh
# b-oc-iso.sh <session> <url> <out.png> <start-text> <end-text> [open]
# 只保留 start..end 之间的 DOM，整页截图（W 默认 2800，zoom 2：版式 1400 CSS px、2 倍像素）。
S=(opencli browser --session $1)
URL=$2; OUT=$3; START=$4; END=$5
cur=$($S get url 2>/dev/null | tr -d '\n')
$S open "about:blank" --window background >/dev/null 2>&1; sleep 1; $S open "$URL" --window background >/dev/null 2>&1; sleep 7
if [[ "${PRESCROLL:-1}" == "1" ]]; then $S eval "(async()=>{const h=document.documentElement.scrollHeight;for(let y=0;y<h;y+=3000){window.scrollTo(0,y);await new Promise(r=>setTimeout(r,60));}window.scrollTo(0,0);return 1})()" >/dev/null; sleep 2; fi
j() { printf '%s' "$1" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))' | sed 's/[|&]/\\&/g'; }
JS=$(sed -e "s|__START__|$(j $START)|" -e "s|__END__|$(j $END)|" -e "s|__UP__|${UP:-0}|" -e "s|__ZOOM__|${Z:-2}|g" -e "s|__W__|${W:-2800}|g" ${0:A:h}/b-oc-iso.js | grep -v '^//')
$S eval "$JS"
sleep 1.5
$S screenshot "$OUT" --width ${W:-2800} --full-page
