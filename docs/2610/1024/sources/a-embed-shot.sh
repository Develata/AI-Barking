#!/bin/sh
# usage: a-embed-shot.sh <tweet-id> <out-win-path> [height]
# opens the anonymous X embed card in opencli background window, 2x zoom screenshot, 1400 wide
id="$1"; out="$2"; h="${3:-2400}"
opencli browser a-shot open "https://platform.twitter.com/embed/Tweet.html?id=$id&theme=light" --window background >/dev/null 2>&1
sleep 12
opencli browser a-shot eval "(function(){document.documentElement.style.zoom='2'; var b=[...document.querySelectorAll('a,button,span')].find(e=>e.textContent.trim()=='Show more'); if(b) b.click(); return [...document.querySelectorAll('img')].map(i=>i.complete).join(',')})()" 2>&1 | grep -v "pdate\|Run:"
sleep 12
opencli browser a-shot screenshot "$out" --width 1400 --height "$h" 2>&1 | grep -i "saved\|error"
