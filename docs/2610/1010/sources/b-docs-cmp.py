import re,sys
def norm(s):
    s=re.sub(r'\[([^\]]*)\]\([^)]*\)',r'\1',s)
    s=re.sub(r'\[([^\]]*)\]\s*\[[^\]]*\]',r'\1',s)
    s=re.sub(r'[`*>|#]','',s)
    s=re.sub(r'\s+',' ',s).strip().lower()
    return s
old=norm(open(sys.argv[1],encoding='utf-8').read())
new=open(sys.argv[2],encoding='utf-8').read()
paras=[p for p in re.split(r'\n\s*\n',new) if p.strip()]
miss=0
for p in paras:
    n=norm(p)
    # sentence-level
    sents=re.split(r'(?<=[.!?:])\s+',n)
    bad=[s for s in sents if len(s)>25 and s not in old]
    if bad:
        miss+=1
        print('--NEW/CHANGED:',p.strip()[:600].replace('\n',' '))
print('paras',len(paras),'changed',miss)
