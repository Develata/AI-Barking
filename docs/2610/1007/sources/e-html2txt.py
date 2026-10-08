#!/usr/bin/env python3
"""e-html2txt.py <in.html> <out.txt> : crude HTML -> text (stdlib only). Keeps <title>, meta description, and text of body."""
import re, html, sys
s = open(sys.argv[1], encoding='utf-8', errors='replace').read()
head = []
for pat in (r'<title[^>]*>(.*?)</title>',
            r'<meta[^>]+(?:name|property)="(?:description|og:title|og:description|article:published_time|article:modified_time|og:updated_time)"[^>]*>'):
    for m in re.finditer(pat, s, re.S):
        head.append(html.unescape(re.sub(r'\s+', ' ', m.group(0))))
t = re.sub(r'<script.*?</script>|<style.*?</style>|<noscript.*?</noscript>|<svg.*?</svg>', '', s, flags=re.S)
t = re.sub(r'<(br|/p|/h\d|/li|/div|/tr|/section|/article|/blockquote)[^>]*>', '\n', t)
t = re.sub(r'<[^>]+>', '', t)
t = html.unescape(t)
t = re.sub(r'[ \t\xa0]+', ' ', t)
t = re.sub(r'\n\s*\n+', '\n\n', t)
open(sys.argv[2], 'w', encoding='utf-8').write('\n'.join(head) + '\n\n====\n\n' + t.strip() + '\n')
print(len(t))
