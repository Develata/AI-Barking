#!/bin/bash
# 1102 期 B 组自写：在 opencli 会话 b（已打开 S-1/A 页面且 documentElement.style.zoom='1.7'）里按文本定位并截图。
# 用法：SCR=<暂存目录> ./b-shot.sh "<行首文本>" <第几处，默认0> <顶部留白px，默认40> <输出.png> [视口高度，默认900]
# 定位脚本 b-find.js（同目录）：在文本节点里找含指定文本的第 k 处，scrollIntoView 后回退 off 像素。
# 截图后由 barking shot cropbox/stack 裁切出 images/ 下的成品。
HERE="$(cd "$(dirname "$0")" && pwd)"
J=$(cat "$HERE/b-find.js")
opencli browser --session b eval "($J)('$1',${2:-0},${3:-40})" | head -c 400; echo
sleep 1
opencli browser --session b screenshot "$4" --width 1400 --height ${5:-900} | head -c 80; echo
