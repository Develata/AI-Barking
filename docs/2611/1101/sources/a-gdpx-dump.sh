#!/bin/bash
# a-gdpx-dump.sh: 在 opencli 会话 a-shot 中读取 GDPx 榜单当前视图文本（站点特定的 DOM 读取，供取证记录用）
opencli browser --session a-shot eval "document.querySelector('#leaderboard').innerText" 2>&1 | python3 -I -c "import sys,json;d=json.load(sys.stdin);print(d['data'])"
