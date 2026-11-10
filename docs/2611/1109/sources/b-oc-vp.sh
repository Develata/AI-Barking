#!/bin/zsh
# b-oc-vp.sh <session> <url> <out.png> <anchor-text> [pad-css-px]  原生视口截图：加载后把含锚点文字的元素滚到视口 pad 处，不改缩放、不重设视口（避免重排）
S=(opencli browser --session $1)
URL=$2; OUT=$3; T=$4; PAD=${5:-60}
$S open "about:blank" --window background >/dev/null 2>&1; sleep 1; $S open "$URL" --window background >/dev/null 2>&1; sleep 6
J=$(printf '%s' "$T" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))')
$S eval "(async()=>{const T=$J;for(let i=0;i<30&&!document.body.textContent.includes(T);i++)await new Promise(r=>setTimeout(r,500));const p=document.getElementById('immersive-translate-popup');if(p)p.remove();document.querySelectorAll('body *').forEach(x=>{const c=getComputedStyle(x).position;if(c==='fixed'||c==='sticky'){if(!x.contains(document.body.querySelector('p'))&&!x.textContent.includes(T))x.style.visibility='hidden'}});let s=[...document.querySelectorAll('body *')].filter(e=>e.textContent.includes(T)&&![...e.children].some(c=>c.textContent.includes(T))&&e.getBoundingClientRect().height>0)[0];s.scrollIntoView({block:'start'});await new Promise(r=>setTimeout(r,1200));window.scrollBy(0,-$PAD);await new Promise(r=>setTimeout(r,1000));const r=s.getBoundingClientRect();return JSON.stringify([r.top,r.left,r.width,r.height,innerWidth,innerHeight,devicePixelRatio])})()"
$S screenshot "$OUT"
