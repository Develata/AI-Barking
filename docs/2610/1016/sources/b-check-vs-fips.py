# B2 附加核对：把原文代码输出的种子与 NIST FIPS 186-2 PDF 文本（b-fips1862.txt）里的 s= 行逐条比对。
import re, subprocess, sys
fips = open('docs/2610/1016/sources/b-fips1862.txt', encoding='utf-8').read()
cur = None; pub = {}
for line in fips.splitlines():
    m = re.search(r'Curve ([PB]-\d+)\s*$', line)
    if m: cur = m.group(1)
    m = re.match(r'\s*s=\s*((?:[0-9a-f]{8}\s*){5})$', line)
    if m and cur:
        pub[cur] = re.sub(r'\s+', '', m.group(1)).upper()
out = subprocess.run([sys.executable, '-I', 'docs/2610/1016/sources/b-repro-seeds.py'], capture_output=True, text=True, check=True).stdout
gen = {l.split()[0]: l.split()[-1] for l in out.splitlines()}
for k in sorted(pub):
    g = gen.get(k)
    print(f"{k:6} FIPS186-2={pub[k]}  repro={g}  {'MATCH' if g == pub[k] else ('NOT-REPRODUCED' if g is None else 'MISMATCH')}")
