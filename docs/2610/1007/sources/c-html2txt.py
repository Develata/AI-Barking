"""C 组：把存档的 HTML 抽成带标题层级的纯文本（只用标准库）。用法：python c-html2txt.py in.html out.txt [起始标记]"""
import sys, re, html
from html.parser import HTMLParser
BLOCK = {"p","div","li","ul","ol","h1","h2","h3","h4","h5","h6","br","tr","table","section","article","blockquote","figure","figcaption","header","footer","main"}
SKIP = {"script","style","noscript","svg","head"}
class P(HTMLParser):
    def __init__(s):
        super().__init__(convert_charrefs=True); s.out=[]; s.skip=0; s.h=None
    def handle_starttag(s,t,a):
        if t in SKIP: s.skip+=1
        if t in BLOCK: s.out.append("\n")
        if re.fullmatch(r"h[1-6]",t) and not s.skip: s.out.append("\n"+"#"*int(t[1])+" ")
        if t=="li" and not s.skip: s.out.append("- ")
        if t=="a" and not s.skip:
            d=dict(a); 
            if d.get("href"): s.out.append("")  # 链接在文末另列
    def handle_endtag(s,t):
        if t in SKIP and s.skip: s.skip-=1
        if t in BLOCK: s.out.append("\n")
    def handle_data(s,d):
        if not s.skip: s.out.append(d)
p=P(); p.feed(open(sys.argv[1],encoding="utf-8").read())
txt="".join(p.out)
txt=re.sub(r"[ \t]+\n","\n",txt); txt=re.sub(r"\n{3,}","\n\n",txt).strip()+"\n"
open(sys.argv[2],"w",encoding="utf-8",newline="\n").write(txt)
print(len(txt))
