"""把保存的 HTML 去掉 script/style 后转成纯文本（保留 <a> 的 href 以便追链）。用法：python a-html2txt.py in.html out.txt"""
import re, sys, html
s = open(sys.argv[1], encoding="utf-8", errors="replace").read()
s = re.sub(r"(?s)<(script|style|noscript|svg)[^>]*>.*?</\1>", "", s)
s = re.sub(r"(?s)<!--.*?-->", "", s)
s = re.sub(r'(?is)<a\s[^>]*href="([^"]*)"[^>]*>(.*?)</a>', lambda m: f"{m.group(2)} [{m.group(1)}]", s)
s = re.sub(r"(?i)</?(p|div|br|li|h[1-6]|tr|section|article|ul|ol|table|header|footer|main)[^>]*>", "\n", s)
s = re.sub(r"<[^>]+>", "", s)
s = html.unescape(s)
s = re.sub(r"[ \t\xa0]+", " ", s)
s = re.sub(r"\n\s*\n+", "\n\n", s).strip()
open(sys.argv[2], "w", encoding="utf-8").write(s + "\n")
