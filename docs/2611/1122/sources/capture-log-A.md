# 1122 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，UTC+8。时间为北京时间（本机时钟）；批量抓取的逐条结果在 `a-fetch-log.jsonl`，截图动作在 `a-shots-log.jsonl`，热度在 `a-heat.json`。批量清单 `a-fetch-list-1.tsv … -13.tsv`。`export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"` 后用 `barking`。请求均走 `barking fetch`（每条限时 60 s，PDF 180 s）；个别一次性请求用 `curl --max-time 60`。不用 firecrawl，不读取浏览器 Cookie，未登录、未发帖。

## 一、网页与 PDF 存档（barking fetch）

| 批 | 北京时刻（约） | 清单 | 结果 |
|---|---|---|---|
| 1 | 00:11:11 | Lean 博客页与 RSS、Xena、Sutherland、Tao 博文与首页、AHM、Clay RH（索引页）/规则/首页、alpo.ge 首页、Lean FRO、Mathlib Initiative | 13 条全部 ok。Clay `…/millennium-problems/riemann-hypothesis/` 只是索引页，改取 `…/millennium/riemann-hypothesis/` |
| 2 | 00:12:52 | Clay RH 正页（clay-rh2）、Clay 新闻页、clay-qualifying（与 rules 同页，重复） | ok |
| 3 | 00:13 | Clay 规则 PDF | ok（PDF，不入库） |
| 4 | 00:13 | alpo.ge/remark.pdf | ok；SHA-256 `3fc97784b71ddd44ff8aa0a177e99622a1c61b13cfa843a0989ee4a7a0cfeb8d`，与 1120 存档相同 |
| 5 | 00:14 | Quanta Transformation 专栏页、Science、phys.org、TechCrunch、OfficeChai | Quanta、TechCrunch、OfficeChai ok；**Science 403（Cloudflare 质询）、phys.org 403：blocked，未绕过** |
| 6 | 00:15 | philarchive Nielsen、Leo de Moura 博客、Quanta /update/169569 与 /169564、Aaronson 博客 | Leo、Aaronson ok；**Nielsen 403（Cloudflare）：改用 opencli 读摘要**（见下）；Quanta 两个单条链接都重定向到专栏页（存档与 quanta-updates 完全相同） |
| 7 | 00:16 | Lipponen README 与 docs（raw） | ok；另 `curl` 取 GitHub API 提交列表 `a-lipponen-commits.json` |
| 8 | 00:16 | SciAm 两篇、micjm（HN 帖链接） | ok |
| 9 | 00:16:58 | 中文：新浪科技两篇、腾讯两篇、第一财经、返朴（观察者网）、科学网、新浪新闻、什么值得买、墨子沙龙 | 10 条全部 ok |
| 10 | 00:17 | 中文：投资界、凤凰网、新华网客户端、腾讯“多位数学家发声”、新浪财经 | 5 条全部 ok |
| 11 | 00:19 | Anthropic 新闻、研究页、claude.com 博客 | ok；三页均无 RH 条目 |
| 12 | 00:24 | `leanprover-community/riemann-hypothesis` README、james-shore/rh-proof-dependencies README | ok |
| 13 | 00:25 | lean-lang.org/fro/about | ok |
| 14 | 00:31 | Aaronson 博文页（?p=10344）、Clay Poincaré 页（对照“Solved”标签）、Xena 11-21 帖 | 3 条全部 ok |

另：北京 00:37 重取 Sutherland 页，last updated 仍为 2026-11-23 14:35 UTC，HTML SHA-256 与 00:11 取的相同。

`.txt` 均已打开抽查为正文（非质询页）。

## 二、个别请求（curl / API）

- GitHub API：`leanprover-community/blog` 的提交列表、PR #143/#149/#150/#153/#154 元数据、提交 7fc0bd63/29fb4b8d/d01066fd 的 diff（`a-gh-leanblog-*`），约 00:12。
- `curl -sI https://alpo.ge/remark.pdf`：ETag `e447a79e04d634f9391306d44d3f1561`，响应无 Last-Modified；PDF 内 CreationDate Fri Nov 20 20:52:48 2026 +08。
- Hacker News Algolia 搜索与条目：约 00:14–00:18（`barking heat` 取 7 个条目，`a-hn-*.json`）。
- Lean Zulip：匿名 web-public 接口分页取话题全文（沿用 `docs/2611/1120/sources/a-zulip-fetch.py` 的方法，699 条，最后一条 北京 2026-11-23 19:19（UTC 11:19），取于北京 11-24 00:20 前后）到 scratchpad，不入库；交付只留被引用的 16 条 `a-zulip-excerpts.md`（已去掉一条对具体个人下断语的帖子）。

## 三、失败

| 对象 | 原因 | 处理 |
|---|---|---|
| Science 文章（www.science.org/content/article/ai-may-have-solved-math-s-greatest-mystery-…） | 403，Cloudflare 质询 | 不绕过；未用 |
| phys.org 两篇 | 403 | 未用 |
| philarchive Nielsen（curl） | 403，Cloudflare 质询 | 用 opencli 打开读页面可见摘要（匿名公开页，无登录），存 `a-nielsen-philarchive.txt`，约 00:20 |
| Reddit（`barking heat --reddit`） | 搜索接口跳转 `old.reddit.com/login` | 记失败，不用登录态；Reddit 热度：未取到 |
| 单独的 Quanta 条目页 | 重定向到专栏页 | 在专栏页用 opencli 点 “Read More” 展开后截图 |
| Anthropic 官方表态 | 新闻页/研究页/博客无条目 | “未找到一手来源”，未查 X |

## 四、截图（`barking shot` render + cropbox/stack；11 号用 opencli）

均以 headless Chrome 渲染（`render`，匿名，无登录栏），再用 `cropbox`/`stack` 只裁切、拼接，不改像素。

| 文件 | 来源页面 | 渲染参数 | 备注 |
|---|---|---|---|
| 01、02 | Lean 博客声明 | `--width 700 --height 2600`（DPR 2） | 约 00:23 |
| 03、04、05 | math.mit.edu/~drew/grh/ | 先 `--width 700`（DPR 2）渲出，表格“Detail”列被截断（如 “9 h 8 mir”“is sti”），弃用；改 `--width 1000 --scale 1.4`（1400 px 宽、DPR 1.4） | 深色底；04 由表头与连续行两段拼接 |
| 06（作废）、06-…-2x | Tao 博文 | `--width 700` 版式把正文栏右侧截掉，弃用；先 `--width 1000 --scale 1.4`（760×872，作废），再 `--width 1000 --scale 2`（DPR 2）得 06-tao-grh-digestion-2x.png | 只留正文栏（1060×1260） |
| 07 | Clay RH 页 | `--width 700`（DPR 2） | 页面右下有 Cookie 横幅，裁切区避开；手稿图被略去（拼接） |
| 08（作废） | Xena | `--width 700 --height 12000`（DPR 2，1400×24000） | Question 1 标题第二行被裁掉，作废，由 12 替代 |
| 09-zh-pedaily-ai-pojie | 投资界转载文（旧文件名 09-zh-pedaily-baidu-ai-pojie.png 重复，待删除） | `--width 700 --height 1800`（DPR 2，原图 1400×3600，裁成 670 px 宽） | 只截文章本身；页面右侧“AI 阅读助手”栏、导航裁掉 |
| 10 | Clay 规则页 | `--width 700 --height 2000` | |
| 11（作废）、11-…-2x | Quanta 专栏（opencli） | 在 opencli 浏览器会话里展开 “Read More”，隐藏 fixed/sticky 元素（页眉、Cookie 横幅）；第一版 zoom 1.4、视口 1400（800 px 宽，作废）；第二版 zoom 2、视口 2000（`screenshot --width 2000`）两次后拼接；**浏览器翻译扩展的图标出现在页面右缘，只取左侧文章栏裁掉**；会话均已 close | 1100×964 |
| 12 | Xena | 同 08 渲染（`xena`），Question 1 标题区间加高 | |

每张已逐张打开目视：文字清楚、边界落在行间；01–05、07、09–12 版面完整。06 左侧留有极细边。已知限制：03–06、11 不是严格 2 倍像素比（宽度 ≥ 1000 才能让页面版式完整），已在 evidence 的截图清单注明。

## 五、交付前自查

- `grep -rIn` 对 `docs/2611/1122/sources/a-*` 与 `evidence-A.md`、`capture-log-A.md`：无本机用户名/路径/邮箱；X 平台内容未存档；GitHub PR/提交 JSON 内是公开仓库的 PR 作者账号与官方 bot 邮箱，不含抓取者信息。
- 抓取响应头只存 barking 过滤后的版本（`.headers` 注明“其余 N 个响应头未存档”）。
- Zulip 摘录只含 web-public 频道公开帖；未存邮箱与头像字段（接口返回的完整 JSON 留在 scratchpad，不入库）。
- 单个文件 > 1 MB：只有 `a-hn-50574968.json`（1,099,867 字节，HN 原帖约 2000 条评论的 Algolia 全量 JSON），入库前请 Claude 决定是否留（或删，evidence 只引其分数与评论数）；`a-hn-50589618.json` 592 KB，`a-tao.html` 609 KB，`a-claude-blog.html` 604 KB（HTML 本不入库）。

## 六、待删除文件（由 Claude 收尾时一次删除；本组未执行 rm）

- `docs/2611/1122/images/08-xena-four-questions.png`（作废，由 12 替代）
- `docs/2611/1122/sources/a-quanta-169569.html`、`.headers`、`.txt`；`a-quanta-169564.html`、`.headers`、`.txt`（与 quanta-updates 重复）
- `docs/2611/1122/sources/a-clay-qualifying.html`、`.headers`、`.txt`（与 clay-rules 重复）
- `docs/2611/1122/sources/a-science.*`、`a-phys.*`、`a-nielsen.*`（blocked 的质询页壳，如有残留文件）
- `docs/2611/1122/images/06-tao-grh-digestion.png`、`09-zh-pedaily-baidu-ai-pojie.png`、`11-quanta-radziwill-nonsensical.png`（被 06-…-2x、09-zh-pedaily-ai-pojie、11-…-2x 取代）
- 仓库外临时文件 4 个（本机临时目录，Claude 收尾时已删）
- 可选：`a-techcrunch.*`、`a-micjm.*`、`a-sciam-claims.*`（未被 evidence 引用，只作线索）
