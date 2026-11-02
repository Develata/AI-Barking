#!/usr/bin/env python3
# a-gdpx-domains.py: 站点特定脚本（取证用）——通过 opencli 会话 a-shot 在 gdpx.semianalysis.com 榜单上用页面内 JS 点击切换域名/指标，读出各视图的榜单文本。
import subprocess, json, sys, time
S = "a-shot"
def oc(*a):
    r = subprocess.run(["opencli", "browser", "--session", S, *a], capture_output=True, text=True)
    try: return json.loads(r.stdout)
    except Exception: return {"ok": False, "raw": r.stdout[:300], "err": r.stderr[:300]}
def js(code):
    r = oc("eval", code)
    return r.get("data") if r.get("ok") else ("ERR " + json.dumps(r)[:300])
def click_text(sel, startswith, nth=0):
    code = f"(()=>{{const l=[...document.querySelectorAll({json.dumps(sel)})].filter(x=>x.textContent.trim().startsWith({json.dumps(startswith)}));if(!l[{nth}])return 'none';l[{nth}].click();return 'ok'}})()"
    return js(code)
def board():
    return js("document.querySelector('#leaderboard').innerText")
def table(t):
    return t[t.find("\n#\n"):]
def checked():
    return json.loads(js("JSON.stringify([...document.querySelectorAll('[role=menuitemcheckbox]')].map(x=>[x.getAttribute('aria-checked')=='true',x.textContent]))"))
def open_domain_menu():
    click_text("#leaderboard button", "Domain"); time.sleep(0.6)
def set_only(target):
    for ok, n in checked():
        want = (n == target)
        if ok != want:
            click_text("[role=menuitemcheckbox]", n); time.sleep(0.4)
if __name__ == "__main__":
    pass
