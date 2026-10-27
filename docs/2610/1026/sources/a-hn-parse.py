import json,sys,re,html
d=json.load(open(sys.argv[1],encoding='utf-8'))
def txt(t):
    t=re.sub(r'<p>','\n',t or '');t=re.sub(r'<[^>]+>','',t);return html.unescape(t)
out=[]
def walk(n,depth=0):
    for c in n.get('children',[]):
        out.append((depth,c))
        walk(c,depth+1)
walk(d)
top=[(dp,c) for dp,c in out if dp==0]
print('total',len(out),'toplevel',len(top))
# no score in algolia; list by order with length filter
for dp,c in out:
    t=txt(c.get('text'))
    if len(t)>int(sys.argv[2]) and (re.search(sys.argv[3],t,re.I) if len(sys.argv)>3 else True):
        print(f"--- id={c['id']} d={dp} by {c.get('author')} {c.get('created_at')}\n{t[:int(sys.argv[4]) if len(sys.argv)>4 else 700]}")
