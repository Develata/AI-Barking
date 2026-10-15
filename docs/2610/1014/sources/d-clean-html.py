import re,html,sys
src,dst=sys.argv[1],sys.argv[2]
s=open(src,encoding='utf-8',errors='ignore').read()
s=re.sub(r'(?is)<(script|style|svg)[^>]*>.*?</\1>','',s)
s=re.sub(r'(?i)<br\s*/?>','\n',s)
s=re.sub(r'(?i)</(p|div|h[1-6]|li|section|article|tr|blockquote|pre)>','\n',s)
s=re.sub(r'<[^>]+>',' ',s)
s=html.unescape(s)
s=re.sub(r'[ \t\r\f\v]+',' ',s)
s=re.sub(r' *\n *','\n',s)
s=re.sub(r'\n{2,}','\n',s)
open(dst,'w',encoding='utf-8').write(s)
print(len(s))
