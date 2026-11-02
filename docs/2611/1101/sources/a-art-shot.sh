#!/bin/bash
# a-art-shot.sh <段落文本片段> <输出png> <高度px> [上移px]
# 站点特定脚本（取证用）：在 opencli 会话 a-art（已打开 SemiAnalysis 长文）里设 body zoom=2，隐藏浏览器侧栏小部件（左侧目录条、固定按钮），把含该片段的段落滚到视口上部，截视口（宽 1520 = 2×760 CSS px）。
FRAG=$(printf '%s' "$1" | python3 -c 'import sys,json;print(json.dumps(sys.stdin.read()))')
opencli browser --session a-art eval "(()=>{document.documentElement.style.zoom='1';document.body.style.zoom='2';const el=[...document.querySelectorAll('p,h2,h3,li')].find(x=>x.textContent.includes($FRAG));if(!el)return 'none';const a=document.elementsFromPoint(22,300);if(a[0]&&/pencraft/.test(a[0].className)&&a[0].tagName==='DIV')a[0].style.display='none';[...document.querySelectorAll('button')].filter(e=>getComputedStyle(e).position==='fixed').forEach(e=>e.style.display='none');el.scrollIntoView({block:'start'});window.scrollBy(0,-${4:-40});return 'ok'})()" | head -1
sleep 1.5
opencli browser --session a-art screenshot "$2" --width 1520 --height "$3" | head -1
