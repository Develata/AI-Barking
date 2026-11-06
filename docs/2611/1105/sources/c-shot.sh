#!/bin/bash
# usage: c-shot.sh <out-name> <anchor text|img:figN> [height] ; opencli shot of the welfare-regard page at zoom 2, viewport 2400 px wide
S=/tmp/claude-1000/-home-develata-gitclone-AI-Barking/5e94c559-e021-48dd-8ae4-b537958b3b2a/scratchpad
OUT=$1; A=$2; H=${3:-1600}
opencli browser --session c-shot eval "(function(){var a=$(printf '%s' "$A" | python3 -c 'import json,sys;print(json.dumps(sys.stdin.read()))');var el;if(a.indexOf('fig:')==0){var fs=document.querySelectorAll('figure');for(var j=0;j<fs.length;j++){var c=fs[j].querySelector('figcaption');if(c&&c.textContent.indexOf('Figure '+a.slice(4)+':')==0){el=fs[j];break;}}}else if(a.indexOf('img:')==0){el=document.querySelector('img[src=\"'+a.slice(4)+'.png\"]');}else{var xs=document.querySelectorAll('p,h1,h2,h3,figcaption,li');for(var i=0;i<xs.length;i++){if(xs[i].textContent.indexOf(a)>=0){el=xs[i];break;}}}if(!el)return 'NOTFOUND';el.scrollIntoView();window.scrollBy(0,-20*2);return Math.round(el.getBoundingClientRect().top);})()" | tail -1
sleep 1
opencli browser --session c-shot screenshot $S/$OUT.png --width 2400 --height $H | tail -1
