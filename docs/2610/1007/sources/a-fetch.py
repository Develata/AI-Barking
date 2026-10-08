"""1007 A 组通用抓取：普通浏览器 UA 的 urllib GET，存 a-<slug>.html（本地，不入库）与 a-<slug>.txt（抽出的标题/日期元数据 + 文本），并追加 a-fetch-log.jsonl。
用法：python a-fetch.py slug=URL [slug=URL ...]"""
import sys, re, json, html, datetime, urllib.request
UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
def now_bj():
    return (datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00"
for arg in sys.argv[1:]:
    slug, url = arg.split("=", 1)
    rec = {"time_bj": now_bj(), "slug": slug, "url": url}
    try:
        req = urllib.request.Request(url, headers={"User-Agent": UA, "Accept-Language": "en-US,en;q=0.9"})
        r = urllib.request.urlopen(req, timeout=40)
        raw = r.read()
        rec.update({"status": r.status, "final_url": r.geturl(), "bytes": len(raw), "content_type": r.headers.get("content-type")})
        s = raw.decode("utf-8", "replace")
        open(f"a-{slug}.html", "w", encoding="utf-8").write(s)
        meta = {}
        for k in ["article:published_time", "article:modified_time", "og:title", "og:description", "datePublished", "dateModified"]:
            m = re.search(r'(?:property|name)="%s"[^>]*content="([^"]*)"' % re.escape(k), s) or re.search(r'"%s"\s*:\s*"([^"]*)"' % re.escape(k), s)
            if m: meta[k] = html.unescape(m.group(1))
        t = re.sub(r"(?is)<(script|style|noscript|svg|nav|footer|header)[^>]*>.*?</\1>", "", s)
        t = re.sub(r"(?i)</?(p|div|br|li|h[1-6]|tr|section|article|blockquote)[^>]*>", "\n", t)
        t = re.sub(r"<[^>]+>", "", t); t = html.unescape(t); t = re.sub(r"[ \t\xa0]+", " ", t); t = re.sub(r"\n\s*\n+", "\n", t).strip()
        open(f"a-{slug}.txt", "w", encoding="utf-8").write("SOURCE: %s\nFETCHED_BJ: %s\nMETA: %s\n\n%s\n" % (url, rec["time_bj"], json.dumps(meta, ensure_ascii=False), t))
        rec["meta"] = meta; rec["txt_chars"] = len(t)
    except Exception as e:
        rec["error"] = repr(e)[:200]
    print(json.dumps(rec, ensure_ascii=False)[:400])
    open("a-fetch-log.jsonl", "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
