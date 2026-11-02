import json,sys
t=json.load(open('b-longcat-eval-table.json'))[0]
hdr=t[0][1:]; rows=[]; grp=None
for r in t[1:]:
    if len(r)==11: grp=r[0]; r=r[1:]
    rows.append((grp,r[0],r[1:]))
def num(s):
    if s=='-': return None
    return float(s.rstrip('*')), s.endswith('*')
names=hdr[1:]
print('n rows',len(rows), 'cols',names)
hi=lo=eq=na=0
out=[]
for g,n,v in rows:
    lc=num(v[0]); o5=num(v[5])
    if lc is None or o5 is None: na+=1; out.append((g,n,v[0],v[5],'无对比格'));continue
    d=round(lc[0]-o5[0],2)
    s='高' if d>0 else ('低' if d<0 else '平')
    if d>0: hi+=1
    elif d<0: lo+=1
    else: eq+=1
    out.append((g,n,v[0],v[5],s,d,'Opus5带*' if o5[1] else 'Opus5无*(美团自测)'))
for o in out: print(*o,sep=' | ')
print('高',hi,'低',lo,'平',eq,'Opus5缺',na)
# unstarred cells overall per model col
print()
for i,nm in enumerate(names):
    if i==0: continue
    ns=[r[1] for r in rows if r[2][i]!='-' and not r[2][i].endswith('*')]
    st=[r[1] for r in rows if r[2][i].endswith('*')]
    print(nm,'无*',len(ns),'带*',len(st),'缺',len(rows)-len(ns)-len(st))
