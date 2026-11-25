#!/bin/zsh
# A 组：AA 视频榜四个子榜的表格与渲染后 DOM 存档（opencli，JS 切换 With/No Audio）
# 用法：a-aa-capture.sh <image-to-video|text-to-video> <audio|silent>
S=docs/2611/1124/sources
PAGE=$1; MODE=$2
opencli browser --session a-shot --window foreground open "https://artificialanalysis.ai/video/leaderboard/$PAGE" >/dev/null
sleep 4
if [ "$MODE" = silent ]; then
  opencli browser --session a-shot eval "(()=>{const b=[...document.querySelectorAll('button[role=radio]')].find(x=>x.innerText.trim()=='No Audio'); b.dispatchEvent(new MouseEvent('mousedown',{bubbles:true}));b.click();return 1})()" >/dev/null
  sleep 2
fi
OUT=$S/a-aa-$PAGE-$MODE
date +%FT%T%z > $OUT.captured-at.txt
opencli browser --session a-shot eval "(()=>{const t=document.querySelector('table');const hdr=location.href+'\n'+document.querySelector('h2').innerText.replace(/\n/g,' ')+'\n'+([...document.querySelectorAll('p')].find(x=>x.innerText.startsWith('Ranks'))||{innerText:''}).innerText+'\n'+[...document.querySelectorAll('button[role=radio]')].map(b=>b.innerText+'='+b.getAttribute('aria-checked')).join(', ')+'\n';return hdr+[...t.querySelectorAll('tr')].map(r=>[...r.children].map(c=>c.innerText.replace(/\n/g,' ')).join(' | ')).join('\n')})()" | python3 -c "import sys,json; print(json.load(sys.stdin)['data'])" > $OUT.table.txt
opencli browser --session a-shot eval "document.documentElement.outerHTML" | python3 -c "import sys,json; print(json.load(sys.stdin)['data'])" > $OUT.dom.html
wc -l $OUT.table.txt; head -6 $OUT.table.txt
