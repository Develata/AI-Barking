#!/bin/zsh
# a-oc-iso.sh <session> <url> <out.png> <zoom> <widthCss> <height> '<specs json>' [预滚动锚点文字]
# 打开页面、（可选）先滚到某文字处等图表绘制完成、隔离元素、截图。
S=(opencli browser --session $1); URL=$2; OUT=$3; Z=$4; W=$5; H=$6; SPECS=$7; PRE=$8
$S open "$URL" --window ${WIN:-background} >/dev/null 2>&1; sleep 5
$S eval "location.reload(); 1" >/dev/null 2>&1; sleep 5
if [[ -n "$PRE" ]]; then
  ${0:A:h}/a-oc-run.sh $1 ${0:A:h}/a-oc-prescroll.js "T=\"$PRE\""
  sleep 1; $S eval "window.scrollBy(0,200);1" >/dev/null; sleep 1; $S eval "window.scrollBy(0,-200);1" >/dev/null; sleep 4
fi
${0:A:h}/a-oc-run.sh $1 ${0:A:h}/a-oc-isolate.js "SPECS=$SPECS" ZOOM=$Z WIDTH=$W
sleep 2
$S screenshot "$OUT" --width 1400 --height $H
