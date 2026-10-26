# 解析 HN 页面存档，按页面顺序（HN 默认按排名）列出顶层评论
import re, html, sys
s = open("a-hn-item.html", encoding="utf-8", errors="replace").read()
rows = re.split(r'<tr class=[\'"]athing comtr', s)[1:]
out = []
for r in rows:
    m_id = re.search(r'id=[\'"](\d+)[\'"]', r)
    ind = re.search(r'indent=[\'"](\d+)[\'"]', r)
    user = re.search(r'class=[\'"]hnuser[\'"]>([^<]*)<', r)
    age = re.search(r'class=[\'"]age[\'"] title=[\'"]([^\'"]*)[\'"]', r)
    body = re.search(r'class=[\'"]commtext[^>]*>(.*?)</div>', r, re.S)
    t = html.unescape(re.sub(r'<[^>]+>', ' ', body.group(1))) if body else ''
    t = re.sub(r'\s+', ' ', t).strip()
    out.append((m_id.group(1) if m_id else '', int(ind.group(1)) if ind else -1, user.group(1) if user else '', age.group(1) if age else '', t))
print(len(out), "comments; top-level:", sum(1 for o in out if o[1]==0))
n=0
for i,(cid,ind,u,a,t) in enumerate(out):
    if ind==0:
        n+=1
        print(f"#{n} pos{i} id={cid} user={u} {a}\n   {t[:420]}\n")

if len(sys.argv) > 1:
    for cid,ind,u,a,t in out:
        if cid in sys.argv[1:]:
            print(f"--- id={cid} indent={ind} user={u} {a}\n{t}\n")
    import re as _re
    print("=== comments mentioning melanoma/sensitiv/threshold/missed ===")
    for i,(cid,ind,u,a,t) in enumerate(out):
        if _re.search(r'melanoma|sensitiv|threshold|false negative|discharge|miss', t, _re.I):
            print(f"id={cid} indent={ind} user={u} {a}: {t[:500]}\n")
