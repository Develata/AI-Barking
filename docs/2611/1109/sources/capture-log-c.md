# C 组 capture-log（北京时间，UTC+8，本机时间）

执行方 claude-sonnet-5-5；时段 2026-11-10 15:29–16:12。工具：`barking fetch/html2txt/heat/shot cropbox|stack`、opencli browser（会话 c1–c9，`--window background`，未登录、未接受 Cookie、未发帖）、curl（一次性）、WebSearch（只作线索，未存档）。机器日志：`c-fetch-log.jsonl`、`c-shots-log.jsonl`、`c-heat.json`。SEC 页面一律走 opencli 浏览器，没有用 curl 请求 SEC。

| 时间 | URL（已去查询参数） | 工具 | 结果 |
|---|---|---|---|
| 15:29 | https://matteoraso.github.io/im-not-convinced-the-ai-bubble-exists.html | barking fetch（清单 c-list1.tsv） | 200 ok，`c-blog.html/.txt`。写盘后把文末竞选与募捐段（含其外链）在 html 与 txt 中替换为占位说明；复查两文件均无该段 |
| 约 15:35 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm（首版 S-1） | opencli open → get html → html2txt | ok，`c-s1-first.html`（13.8 MB）、`c-s1-first.txt`（46,388 行，1.6 MB，超 1 MB） |
| 约 15:37 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm（S-1/A 修订一） | 同上 | ok，`c-s1a.html`（14.0 MB）、`c-s1a.txt`（47,246 行，1.67 MB，超 1 MB） |
| 约 15:50 | 上述两份 txt | grep / 手工读 | 摘录为 `c-s1-excerpts.txt`（23 KB）。注意：“2.08x”在图里，txt 搜不到 |
| 15:53 | Alphabet FY2025 10-K PDF（cloudfront）、Meta PDF（cloudfront）、BBC、isaiprofitable（www）、archive.li、semianalysis（www.newsletter）、GitHub 作者页（清单 c-list2.tsv） | barking fetch | Alphabet 200 ok（1.26 MB PDF，4,951 行 txt）；BBC ok；GitHub ok；**Meta PDF 403**；**isaiprofitable 证书不匹配（www）**；**archive.li 200 但为 CAPTCHA**（blocked，不绕过）；**semianalysis（www.newsletter）连接失败** |
| 15:57 | Meta cloudfront PDF | curl，通用 UA（`Mozilla/5.0 (Macintosh) research-fetch`） | 403 `<Code>AccessDenied</Code>`（S3），同一来源第二次失败，停止；存的 111 字节错误体 `c-meta-pdf.pdf`、`c-meta-pdf.headers`、`c-meta-pdf.headers.tmp` 列入待删除 |
| 15:57 | https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=1326801&type=10-K … 及 filing 索引 | opencli | ok，找到 FY2025 10-K（Acc-no 0001628280-26-003942，2026-01-29） |
| 15:58 | https://www.sec.gov/Archives/edgar/data/1326801/000162828026003942/meta-20251231.htm | opencli get html → html2txt | ok，`c-meta-10k.html`（2.4 MB）、`c-meta-10k.txt`（4,546 行，550 KB） |
| 15:58 | stockanalysis.com 的 GOOGL/META/NVDA statistics 页（清单 c-list3.tsv） | barking fetch | 3 个 200 ok，`c-pe-googl/meta/nvda.*`，页面 “Last updated: Nov 10, 2026”，收盘 2026-11-09 美东 |
| 15:59 | https://isaiprofitable.com/ 与 https://newsletter.semianalysis.com/p/google-we-have-no-moat-and-neither（清单 c-list4.tsv，`--force`） | barking fetch | semianalysis 200 ok（`c-semianalysis.*`，208 行，未用于核数）；isaiprofitable 空壳（blocked，正文 18 字符，JS 渲染） |
| 约 16:00 | https://isaiprofitable.com/ | opencli browser 渲染 + `eval document.body.innerText` | ok，`c-isaiprofitable.txt` |
| 16:00 | https://news.ycombinator.com/item?id=50418975 | `barking heat … --item 50418975 --hn …` | HN ok：550 分、944 评、发帖 11-09 07:44（北京），`c-hn-50418975.json`、`c-heat.json`；Reddit 搜索被重定向登录页，**失败**（已记入 c-heat.json errors） |
| 约 16:02 | https://www.archive.li/DgKzp | opencli 打开 | CAPTCHA 页（“Please complete the security check”），停止，**失败** |
| 16:03 | “AI companies committed $3 trillion” | WebSearch | 仅线索，结果未存档、未打开 |
| 约 16:00–16:09 | 上述网页的截图 | `c-shot.sh`（opencli open → CSS zoom → 隐藏 fixed 元素与翻译插件图标 → screenshot）+ `barking shot cropbox|stack` | 25c–36 共 12 张 + 4 张废/备用图，详见 evidence-c.md 第 5 节；SEC 页 zoom 1.68（页宽 816px 需完整装进 1400px），博客 zoom 2，stockanalysis zoom 1.5，HN zoom 1.6。HN 页面显示 login，未登录；全部浅色页面 |
| 约 16:10 | 读取 SEC 图片 `mda3aa.jpg` 的尝试 | opencli eval fetch | 会话已被重置（about:blank），未成功；改为在 S-1 页面内滚动到图直接截图，已满足取证，未再重试 |

脚本：`c-shot.sh`（截图辅助）。`c-list1.tsv`–`c-list4.tsv` 为 fetch 清单。

自查（交付前）：对 `docs/2611/1109/` 搜竞选与募捐段的关键词，博客存档与 HN json 中的相关文字已替换为占位，其他文件无命中（HN json 中 若干条评论涉及，已替换）；截图 25c–36 均目视确认不含头像、昵称、登录栏、翻译插件图标；浏览器本机会话信息未写入存档（fetch 的 `.headers` 由 barking 自动裁剪）。
