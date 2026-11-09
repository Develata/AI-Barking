#!/bin/zsh
# usage: b-shot.sh <url> <tmp-name> <height-px> [scroll-to-text]
# opencli 截图：zoom 2，宽 2800 px（=1400 CSS px）。只隐藏 position:fixed 的浮层（Cookie 横幅、遮罩、广告），不点击任何同意按钮。
URL="$1"; NAME="$2"; H="${3:-3000}"; TXT="$4"
IMG="${BARKING_REPO:-$PWD}/docs/2611/1108/images"  # 在仓库根目录运行
opencli browser --session b-shot --window background open "$URL" | tail -c 120
sleep 4
opencli browser --session b-shot eval "document.documentElement.style.zoom='2'; [...document.querySelectorAll('*')].forEach(e=>{const s=getComputedStyle(e); if(s.position==='fixed' && e!==document.body){e.style.display='none'}}); document.body.style.overflow='visible'; document.documentElement.style.overflow='visible'; document.querySelectorAll('[id*=onetrust],[class*=consent],[id*=consent],[class*=cookie],[id*=cookie]').forEach(e=>e.style.display='none'); window.scrollTo(0,0); 'ok'" | tail -c 60
if [ -n "$TXT" ]; then
opencli browser --session b-shot eval "(()=>{const w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT); let n; while(n=w.nextNode()){ if(n.nodeValue.includes(\`$TXT\`)){n.parentElement.scrollIntoView({block:'start'}); window.scrollBy(0,-150); return 'scrolled '+scrollY}} return 'notfound'})()" | tail -c 80
fi
sleep 1
opencli browser --session b-shot screenshot "$IMG/$NAME.png" --width 2800 --height "$H" | tail -c 100
