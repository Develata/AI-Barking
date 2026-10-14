"""B 组：把保存的 HTML 去掉 script/style 后转成纯文本（保留 <a> href），并把当事人姓名替换为“[当事人]”。
用法：python -I b-redact.py in.html out.txt        （HTML→txt）
      python -I b-redact.py --text in.txt out.txt   （已是文本/JSON：只做姓名替换）
姓名模式：当事人英文姓名及其大小写、连字符变体，以及中文音译（模式在 NAME 中，拆开写）。国籍词不替换。"""
import re, sys, html
NAME = re.compile("|".join([r"abdul\s?rahman[\s-]+al[\s-]?f[ou]+zan", r"al[\s-]?f[ou]+zan", r"abdul\s?rahman", r"阿卜" + r"杜勒[拉]?赫曼[·・\s]*(?:" + "阿尔" + "福赞)?", "阿尔" + "福赞", "阿勒" + "福赞", "奥尔" + "福赞"]), re.I)   # 姓名拆开写，避免脚本本身把姓名入库
def redact(s): return NAME.sub("[当事人]", s)
args = sys.argv[1:]
if args[0] == "--text":
    s = open(args[1], encoding="utf-8", errors="replace").read()
    open(args[2], "w", encoding="utf-8").write(redact(s)); sys.exit()
s = open(args[0], encoding="utf-8", errors="replace").read()
s = re.sub(r"(?s)<(script|style|noscript|svg)[^>]*>.*?</\1>", "", s)
s = re.sub(r"(?s)<!--.*?-->", "", s)
s = re.sub(r'(?is)<a\s[^>]*href="([^"]*)"[^>]*>(.*?)</a>', lambda m: f"{m.group(2)} [{m.group(1)}]", s)
s = re.sub(r"(?i)</?(p|div|br|li|h[1-6]|tr|section|article|ul|ol|table|header|footer|main)[^>]*>", "\n", s)
s = re.sub(r"<[^>]+>", "", s)
s = html.unescape(s)
s = re.sub(r"[ \t\xa0]+", " ", s)
s = re.sub(r"\n\s*\n+", "\n\n", s).strip()
open(args[1], "w", encoding="utf-8").write(redact(s) + "\n")
