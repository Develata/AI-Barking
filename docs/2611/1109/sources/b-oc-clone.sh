#!/bin/zsh
# b-oc-clone.sh <session> <url> <out.png> <caption-text> [up]  把含该文字的祖先（up 层）克隆为 body 唯一内容后整页截图（W=2800、zoom 2）
S=(opencli browser --session $1)
URL=$2; OUT=$3; T=$4; UP=${5:-2}
cur=$($S get url 2>/dev/null | tr -d '\n')
$S open "about:blank" --window background >/dev/null 2>&1; sleep 1; $S open "$URL" --window background >/dev/null 2>&1; sleep 7
$S eval "(()=>{const T=$(printf '%s' "$T" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))');let s=[...document.querySelectorAll('body *')].filter(e=>e.textContent.includes(T)&&![...e.children].some(c=>c.textContent.includes(T)))[0];for(let k=0;k<$UP;k++)s=s.parentElement;const c=s.cloneNode(true);c.querySelectorAll('*').forEach(x=>{x.style.opacity='1';x.style.visibility='visible'});const w=document.createElement('div');w.style.cssText='width:700px;margin:40px auto;';c.style.display='block';c.style.opacity='1';w.appendChild(c);document.body.replaceChildren(w);document.documentElement.style.zoom='2';document.documentElement.style.width='1400px';window.scrollTo(0,0);const r=c.getBoundingClientRect();return JSON.stringify([r.width,r.height,c.querySelectorAll('svg').length])})()"
sleep 2
$S screenshot "$OUT" --width 2800 --full-page
