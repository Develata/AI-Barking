# D 组抓取日志（速览 D1–D7）

时间为北京时间（UTC+8）；本机为美东 EDT，2026-10-09 23:36–23:59 EDT = 北京 2026-10-10 11:36–11:59。工具：curl（UA 为普通浏览器串）、firecrawl_scrape（MCP）、HN Algolia API。未使用浏览器登录态，未运行读取本机 Cookie 的工具。

| 北京时间 | 目标 URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 10-10 11:36 | https://epoch.ai/publications/innovationeval | curl | HTTP 200，257 KB，正文完整 | d-d1-epoch.html、d-d1-epoch.txt（100 KB） |
| 10-10 11:36 | https://commandline.microsoft.com/microsoft-decision-1-model-foundry/ | curl | HTTP 403，4.5 KB，为 Cloudflare “Attention Required” 挑战页，非正文；该挑战页文件是我刚生成的试验文件，已删除（d-d2-msdecision.html） | 无 |
| 10-10 11:40 | 同上 | firecrawl_scrape（markdown，maxAge 0） | 成功，元数据 statusCode 200，publishedTime 2026-10-09T18:35:51+00:00；markdown 约 204 KB（含内嵌图表 iframe 的 base64 串） | d-d2-msdecision.md（firecrawl 原样）、d-d2-msdecision.txt（剔除 base64 长串后的可读版）。HTML 原件未存：因 curl 被拦，且未绕过 |
| 10-10 11:36 | https://terrytao.wordpress.com/2026/10/09/what-mathematicians-should-know-about-the-lean-theorem-proverquestions-of-reliability-and-ai/ | curl | HTTP 200，180 KB，正文完整 | d-d3-tao.html、d-d3-tao.txt |
| 10-10 11:36 | https://artificialanalysis.ai/evaluations/harvey-lab-aa | curl | HTTP 200，1.2 MB，正文与排行榜表均在 SSR 文本中；页面无发布时刻 | d-d4-harvey.html、d-d4-harvey.txt |
| 10-10 11:36 | https://arena.ai/blog/ai-alignment-index | curl | HTTP 200，2.4 MB，正文完整；meta 发布时间 2026-10-08T17:00:00Z | d-d5-arena.html、d-d5-arena.txt。页面文本含零宽字符（站点反抄袭水印），原样保留 |
| 10-10 11:36 | https://jessewaites.com/blog/post/i-pointed-ai-at-400-years-of-archives/ | curl | HTTP 200，61 KB，正文完整 | d-d6-waites.html、d-d6-waites.txt |
| 10-10 11:36 | https://deno.com/blog/cloudflare | curl | HTTP 200，64 KB，正文完整 | d-d7-deno.html、d-d7-deno.txt |
| 10-10 11:45 | https://blog.cloudflare.com/deno-joins-cloudflare/ | curl | HTTP 200，294 KB，published_time 2026-10-09T12:50:00Z；查有无交易金额：无 | d-d7-cfblog.html、d-d7-cfblog.txt |
| 10-10 11:50–11:57 | https://hn.algolia.com/api/v1/search?tags=story&query=… | curl / Python urllib（d-heat.py） | 成功；结果见 d-heat.json 与 evidence-d.md 热度表。Harvey v1.1 与 Arena 无对应 HN 帖 | d-heat.json |

## 脚本

- d-html2txt.py：改自 1008 期 a-html2txt.py（去 script/style，保留 href），用于把 HTML 转 txt。
- d-heat.py：HN Algolia 热度查询，结果写入 d-heat.json。
- 命令行里的一次性 curl 循环未单独存为脚本，URL 与结果均已记于上表。

## 失败与限制

1. commandline.microsoft.com 对 curl 返回 Cloudflare 挑战页（403）；仅用 firecrawl 取得文本版，没有该页 HTML 原件，也没有尝试绕过挑战。
2. Algolia 搜索对中文/长标题查询返回不相关旧帖，改用 URL 子串查询；Harvey v1.1 与 Arena Alignment Index 的 HN 查询均无命中，不代表 HN 上绝对没有讨论。
3. D 组不截图，未打开浏览器；图表（Microsoft 的延迟/准确率图、AA 排行榜）的数值来自页面 SSR 文本，未目视核对图形。

## 隐私自查

D 组存档均为公开页面，未使用登录会话；d-d6-waites.html 含作者本人的公开社交链接（作者自己页面上的信息，非抓取者）。交付前对 `docs/2610/1009/` 执行 grep（头像链接、handle 等），D 组文件无抓取者信息。
