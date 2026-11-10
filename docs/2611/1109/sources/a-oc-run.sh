#!/bin/zsh
# a-oc-run.sh <session> <jsfile> [VAR=VALUE ...]  —— 把 jsfile 里的 __VAR__ 替换后交给 opencli browser eval（去掉 // 注释行）。
S=$1; F=$2; shift 2
JS=$(grep -v '^//' "$F")
for kv in "$@"; do k=${kv%%=*}; v=${kv#*=}; JS=${JS//__${k}__/$v}; done
opencli browser --session "$S" eval "$JS"
