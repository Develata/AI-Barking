#!/bin/zsh
# B 组截图包装：设置 Chrome 路径并调用 barking shot
export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
cd "$(git rev-parse --show-toplevel)"
cargo run -q --release --manifest-path tools/barking/Cargo.toml -- shot docs/2611/1110 b "$@"
