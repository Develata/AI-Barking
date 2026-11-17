#!/bin/zsh
# usage: b-opencli-save.sh <name> <url>  -- 用 opencli（真实 Chrome，未登录）打开页面，存 innerText 与 outerHTML 到 b-<name>.txt/.html
name=$1; url=$2; D=$(cd "$(dirname "$0")" && pwd)
opencli browser --session b-save open "$url" --window background >/dev/null 2>&1
sleep 6
opencli browser --session b-save eval "JSON.stringify({u:location.href,t:document.title,pub:(document.querySelector('meta[property=\"article:published_time\"]')||{}).content||null,ld:[...document.querySelectorAll('script[type=\"application/ld+json\"]')].map(s=>s.textContent).join('\n').match(/\"(datePublished|dateModified)\":\"[^\"]*\"/g),txt:document.body.innerText,html:document.documentElement.outerHTML})" > "$D/.b-opencli-$name.json" 2>/dev/null
python3 -I - "$D" "$name" <<'PY'
import json,sys
D,name=sys.argv[1:3]
r=json.load(open(f"{D}/.b-opencli-{name}.json"))
d=json.loads(r["data"])
open(f"{D}/b-{name}.txt","w").write(f"URL: {d['u']}\nTITLE: {d['t']}\nPUB_META: {d['pub']}\nLD: {d['ld']}\n\n{d['txt']}\n")
open(f"{D}/b-{name}.html","w").write(d["html"])
print(name,len(d["txt"]),d["t"],d["pub"],d["ld"])
PY
