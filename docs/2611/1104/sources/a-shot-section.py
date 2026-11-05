#!/usr/bin/env python3
"""A 组截图辅助：在 opencli 会话里把含锚点文本的元素滚到视口顶部，截一张 2 倍像素、宽 2400 的视口图到临时目录。
用法：a-shot-section.py <out.png> <锚点文本> [高度=2400] [偏移px=140]
锚点取页面正文里的一小段逐字文字；用 TreeWalker 找第一个包含它的元素。"""
import json, subprocess, sys
out, anchor = sys.argv[1], sys.argv[2]
h = sys.argv[3] if len(sys.argv) > 3 else "2400"
off = int(sys.argv[4]) if len(sys.argv) > 4 else 140
S = "a-shot"
def run(*a):
    r = subprocess.run(["opencli", "browser", "--session", S, *a], capture_output=True, text=True)
    return r.stdout
js = """(()=>{const a=%s;const els=[...document.querySelectorAll('h1,h2,h3,h4,p,li,div,span')].filter(e=>e.children.length<20&&e.textContent.includes(a));
if(!els.length) return 'NOTFOUND';
els.sort((x,y)=>x.textContent.length-y.textContent.length);
const e=els[0]; const hdr=document.querySelector('header'); if(hdr) hdr.style.display='none';
e.scrollIntoView({block:'start'}); window.scrollBy(0,-%d); return 'ok '+e.tagName+' '+e.textContent.length;})()""" % (json.dumps(anchor), off)
print(run("eval", js).strip())
print(run("screenshot", out, "--width", "2400", "--height", h).strip())
