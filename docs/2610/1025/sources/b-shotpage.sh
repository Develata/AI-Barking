#!/bin/sh
# usage: b-shotpage.sh <url> <out-win-path.png> <height> [js-before-shot]
# opencli background window, page zoom 2 (700 CSS px layout, 2x pixels), screenshot 1400 wide
url="$1"; out="$2"; h="$3"; js="${4:-0}"
opencli browser b-s open "$url" --window background >/dev/null 2>&1
sleep 7
opencli browser b-s eval "document.documentElement.style.zoom='2'; 'ok'" >/dev/null 2>&1
sleep 2
if [ "$js" != "0" ]; then opencli browser b-s eval "$js" >/dev/null 2>&1; sleep 2; fi
opencli browser b-s screenshot "$out" --width 1400 --height "$h" 2>&1 | grep -i "saved\|error"
