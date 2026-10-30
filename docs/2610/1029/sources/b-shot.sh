#!/bin/bash
# usage: b-shot.sh <name> <startY> <endY>   (zoomed page coordinates; opencli session bshot already open on the report, zoom 2, nav hidden)
: "${BSHOT_DIR:=$TEMP/bshots}"
N=$1; Y0=$2; Y1=$3; H=$((Y1-Y0))
OUT="$BSHOT_DIR/$N.png"
opencli browser ${S:-bshot} eval "document.documentElement.style.scrollBehavior='auto'; window.scrollTo({top:$Y0,behavior:'instant'}); 'y='+scrollY" 2>/dev/null | head -1
sleep 1
opencli browser ${S:-bshot} screenshot "$OUT" --height $H 2>&1 | head -1
