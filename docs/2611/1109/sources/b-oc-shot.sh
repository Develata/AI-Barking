#!/bin/zsh
# a-oc-shot.sh <session> <url> <out.png> <anchor> <pad> <height> [nth] [openAll]
# 用 opencli 浏览器（后台窗口）按文本锚点定位并截 1400 px 宽、2 倍像素版式的视口图。
S=(opencli browser --session $1)
URL=$2; OUT=$3; ANCHOR=$4; PAD=$5; H=$6; NTH=${7:-0}; OPEN=${8:-0}
cur=$($S get url 2>/dev/null | tr -d '\n')
if [[ "${cur%/}" != "${URL%/}" ]]; then $S open "$URL" --window background >/dev/null 2>&1; sleep 4; fi
if [[ "$OPEN" == "1" ]]; then $S eval "(()=>{document.querySelectorAll('details').forEach(d=>{if(d.innerText.indexOf('On this page')!==0)d.open=true});return 1})()" >/dev/null; fi
JS=$(sed -e "s|__ANCHOR__|$(printf '%s' "$ANCHOR" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))' | sed 's/[|&]/\\&/g')|" -e "s|__PAD__|$PAD|" -e "s|__NTH__|$NTH|" -e "s|__ZOOM__|${Z:-2}|" -e "s|__W__|${W:-1400}|" ${0:A:h}/b-oc-pos.js | grep -v '^//')
if [[ "${PRESCROLL:-1}" == "1" ]]; then $S eval "(async()=>{const h=document.documentElement.scrollHeight;for(let y=0;y<h;y+=4000){window.scrollTo(0,y);await new Promise(r=>setTimeout(r,60));}window.scrollTo(0,0);return 1})()" >/dev/null; sleep 2; fi
$S eval "$JS" >/dev/null; sleep 2; $S eval "$JS" >/dev/null; sleep 1; $S eval "$JS"
$S screenshot "$OUT" --width ${W:-1400} --height $H
