#!/bin/zsh
# c-bk.sh: run the compiled barking with Chrome set (macOS). usage: c-bk.sh <barking args...> (run from anywhere)
cd "$(dirname "$0")/../../../.." || exit 1
export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
exec ./tools/barking/target/release/barking "$@"
