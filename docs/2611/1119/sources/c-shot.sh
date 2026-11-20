#!/bin/zsh
# usage: c-shot.sh <url> <out.png> <js expression returning element> <height> [offset css px] [zoom]
# 用 opencli 后台窗口打开页面，隐藏 fixed/sticky 浮层（顶栏、底部导航、提示条），把目标元素滚到视口上缘后截图；页面内容不改动。
S=c-shot
URL="$1"; OUT="$2"; EXPR="$3"; H="${4:-1400}"; OFF="${5:-20}"; Z="${6:-1.4}"
opencli browser --session $S --window background open "$URL" >/dev/null
sleep 6
opencli browser --session $S eval "(()=>{document.documentElement.style.zoom='$Z';document.querySelectorAll('*').forEach(e=>{const p=getComputedStyle(e).position;if(p==='fixed'||p==='sticky'){e.style.display='none'}});const t=($EXPR);const y=t.getBoundingClientRect().top+scrollY-$OFF*$Z;window.scrollTo(0,y);return 'ok '+Math.round(y)})()"
sleep 2
opencli browser --session $S screenshot "$OUT" --width 1400 --height $H
