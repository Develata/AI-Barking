#!/bin/bash
# usage: b-shot-at.sh <session> <anchor text> <out png> <height> [offsetPx(css, default 40)]
# 先把所有懒加载图片设为立即加载并等待，避免截图时布局位移；滚动两次以稳定位置。
S="$1"; A="$2"; OUT="$3"; H="${4:-1500}"; OFF="${5:-40}"
JS="(()=>{const a=$(printf '%s' "$A" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))');const els=[...document.querySelectorAll('p,li,h1,h2,h3,h4,img,td,th,figure,div')].filter(e=>(e.tagName==='IMG'?(e.alt||''):e.textContent).includes(a)&&(e.tagName==='IMG'||e.textContent.length<4000));if(!els.length)return 'NOT FOUND';els.sort((x,y)=>(x.tagName==='IMG'?0:x.textContent.length)-(y.tagName==='IMG'?0:y.textContent.length));const e=els[0];e.scrollIntoView({block:'start'});window.scrollBy(0,-$OFF*2);const r=e.getBoundingClientRect();return e.tagName+' top='+Math.round(r.top)+' sy='+Math.round(scrollY)})()"
opencli browser "$S" eval "document.querySelectorAll('img').forEach(i=>{i.loading='eager'}); 'ok'" >/dev/null 2>&1
opencli browser "$S" screenshot "E:\gitclone\AI-Barking\docs\2611\1106\sources\b-tmp-warm.png" --width 1264 --height "$H" >/dev/null 2>&1
sleep 3
opencli browser "$S" eval "$JS" 2>&1 | head -1
sleep 2
opencli browser "$S" eval "$JS" 2>&1 | head -1
sleep 1
opencli browser "$S" screenshot "$OUT" --width 1264 --height "$H" 2>&1 | head -1
