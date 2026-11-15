import sys, json, subprocess, time
# 用法：python3 -I a-opencli-save.py <name> <url>
name, url = sys.argv[1], sys.argv[2]
subprocess.run(["opencli", "browser", "--session", "a-cn", "open", url, "--window", "background"], capture_output=True, timeout=90)
time.sleep(2)
r = subprocess.run(["opencli", "browser", "--session", "a-cn", "extract"], capture_output=True, text=True, timeout=90)
try:
    d = json.loads(r.stdout)["data"]
except Exception as e:
    print(name, "ERR", e, r.stdout[:200]); sys.exit(1)
c = d["content"]
open("a-%s-opencli.txt" % name, "w").write("# 经 opencli browser extract 取得（%s）\n%s" % (d["url"], c))
print(name, len(c), d["title"]); print(c[:600].replace("\n", " "))
