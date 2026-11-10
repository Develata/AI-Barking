#!/bin/zsh
# usage: c-shot.sh <session> <url|-> <text-anchor> <out.png> [height] [offsetPx] [zoom]
# opens url (unless "-"), sets zoom (default 2), scrolls the first element whose text contains anchor to top, screenshots 1400 x height (CSS px of the zoomed viewport).
S=$1; URL=$2; ANCH=$3; OUT=$4; H=${5:-1400}; OFF=${6:-40}; Z=${7:-2}
if [ "$URL" != "-" ]; then opencli browser --session $S open "$URL" --window background >/dev/null; sleep 2; fi
opencli browser --session $S eval "document.documentElement.style.zoom='$Z'; Array.from(document.querySelectorAll('*')).forEach(function(e){var p=getComputedStyle(e).position;if(p==='fixed'||p==='sticky'){e.style.display='none'}}); var t=document.getElementById('immersive-translate-popup'); if(t){t.style.display='none'}; 'ok'" >/dev/null
sleep 1
JS="(function(){var a=$(python3 -I -c 'import json,sys;print(json.dumps(sys.argv[1]))' "$ANCH");var w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);var n;while(n=w.nextNode()){if(n.nodeValue.indexOf(a)>=0){n.parentElement.scrollIntoView({block:'start'});window.scrollBy(0,-$OFF);return 'found'}}return 'NOTFOUND'})()"
opencli browser --session $S eval "$JS"
sleep 1
opencli browser --session $S screenshot "$OUT" --width 1400 --height $H
