# 1016 期 B 组抓取日志（Claude 还原 NIST 椭圆曲线种子）

执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。北京时间（UTC+8）；本机美东 EDT = 北京 − 12 小时。开工 20:48，主体 21:00 前完成，补做脱敏与重取到 21:45。工具：`barking fetch / shot / heat`、curl（匿名，浏览器 UA，设 `--max-time`）、HN Algolia、Lobsters JSON、fxtwitter 公开接口（仅 Weis 的一条帖）、Exa 与 WebSearch（找报道线索）、`python -I`。未用 opencli（Browser Bridge 未连接，按指示直接用 `barking shot` headless Chrome）。未登录任何账号，未输入凭据，未用镜像站。Firecrawl 返回 402（额度用尽），未能使用。

## 抓取（`barking fetch`，记录在 `b-fetch-log.jsonl`）

| 约时刻 | name → URL | 结果 | 存档 |
|---|---|---|---|
| 20:48 | anthropic → https://www.anthropic.com/research/ecdsa-curve-origins | 200 ok | `b-anthropic.html/.txt/.headers`（注意：`.txt` 不含页内图，如时间戳表、候选表） |
| 20:50 | bounty → https://words.filippo.io/seeds-bounty/ | 200 ok | `b-bounty.*` |
| 20:50 | solved → https://words.filippo.io/seeds-solved/ | 200 ok | `b-solved.*` |
| 20:50 | filippo-home → https://words.filippo.io/ | 200 ok | `b-filippo-home.*` |
| 20:50 | saweis → https://saweis.net/posts/nist-curve-seed-origins.html | 200 ok | `b-saweis.*` |
| 20:50 | fips1862 → https://csrc.nist.gov/files/pubs/fips/186-2/final/docs/fips186-2.pdf | 200 ok（NIST 存档版 73 页） | `b-fips1862.pdf/.txt` |
| 20:50 | riddle → https://eprint.iacr.org/2015/1018.pdf | 403 blocked（Cloudflare 质询）；未重试，改取摘要页 | 失败文件已删 |
| 20:53 | riddle2 → https://eprint.iacr.org/2015/1018 | 200 ok（只有摘要页，没取到论文全文，Koblitz–Menezes §3.1 未核） | `b-riddle2.*` |
| 20:53 | safecurves → https://cr.yp.to/papers/safecurves-20240809.pdf | 200 ok（附录 C 邮件在第 64–65 页） | `b-safecurves.pdf/.txt` |
| 20:53 | schneier → 2023 年悬赏文章 | 200 ok | `b-schneier.*` |
| 20:53 | ithome、ifeng、icwork、bitcoincom-zh、coindesk | 200 ok | `b-<name>.*` |
| 20:53 | investing-cn → https://cn.investing.com/news/economy-news/article-3613025 | 403 blocked（Cloudflare）；未重试 | 失败文件已删 |
| 20:55 | investing-en → https://www.investing.com/news/economy-news/anthropic-says-claude-solved-25year-cryptography-mystery-in-30-minutes-4954741 | 403 blocked；未重试，仅用搜索摘要里的标题与时间（已在 evidence 标“未验证”） | 失败文件已删 |
| 20:55 | techmeme、unite-en、blockchainnews、filippo-go（GitHub raw `FiloSottile/mostly-harmless/main/seeds/seeds.go`） | 200 ok | `b-<name>.*`（`b-filippo-go.txt` 为 Go 源码，**未运行**） |
| 21:40 | coindesk、bitcoincom-zh 重取（`--force`） | 200 ok | 重取原因见下“脱敏” |

## 其他请求

| 约时刻 | URL / 对象 | 工具 | 结果 |
|---|---|---|---|
| 20:51 | HN：Claude reveals ECDSA curve origins；HN 条目 50122800、50122920 | `barking heat` | 成功：912 分/338 评（50122800，北京 10-17 20:56 取）；50122920 为 52 分；`b-heat.json`、`b-hn-50122800.json`、`b-hn-50122920.json` |
| 20:51 | Reddit 搜索 | `barking heat --reddit` | 失败：响应为登录页，已记入 `b-heat.json` errors |
| 20:55 | Lobsters https://lobste.rs/s/ddtqtn.json | curl | 成功 `b-lobsters-ddtqtn.json`（85 分、14 评；含各评论分） |
| 20:56 | https://api.fxtwitter.com/status/2111168083784245588（Steve Weis 的帖） | curl | 成功 `b-fx-sweis.json`（公开 JSON，未含抓取者信息） |
| 20:56 | Bluesky 公共搜索接口 | curl | 403，未重试，Bluesky 帖未检索 |
| 21:05 | blog.cryptographyengineering.com、blog.cr.yp.to、schneier.com 首页 | curl | 成功，仅用于判断是否有相关新文章，不存档；结果：无 |
| 21:40 | https://old.reddit.com/r/crypto/comments/1x7q90l.json（Techmeme 列出的帖） | curl（一次） | 返回 “Welcome to Reddit” 登录页，视为失败，不再重试 |
| 20:52–21:00 | 报道线索检索 | Exa web_search ×2、WebSearch ×3、HN Algolia ×6 | 成功（WebSearch 对 10-16 之后的新闻索引不全） |
| 20:50 | Firecrawl search ×2 | — | 402 额度用尽 |

## 本机运行（`python -I`，Python 3.15.0）

| 文件 | 内容 |
|---|---|
| `b-repro-seeds.py`、`b-repro-basepoints.py` | 从 `b-anthropic.html` 的 `<pre>` 块原样提取原文两段代码（只做 HTML 实体反转义，不改逻辑）；输出 `b-repro-*.out.txt`，与原文所附输出 `b-repro-*.expected.txt` 逐 token 比对：一致 |
| `b-check-vs-fips.py` | 把脚本输出与 FIPS 186-2 `s=` 行比对，输出 `b-check-vs-fips.out.txt`：9/10 一致，B-163 无输出 |
| `b-prob-check.py` | 核 “1/5810”的口径，输出 `b-prob-check.out.txt` |

## 图片（`images/`；2 倍像素，宽 ≤ 1400 px；只裁切不改像素；已逐张打开目视确认）

建议 13–17 作**正式候选**（符合“每组 3–6 张”）；18–24 作**备用**，由 Claude 取舍，不删。

| 文件 | 内容 | 来源与渲染 | 角色 |
|---|---|---|---|
| `13-claude-found-seeds-half-hour.png` | “Claude found the seeds” 全节：人工起点、“In under half an hour …”、lrand48、1842、5 行时间戳表 | Anthropic 原文页，`barking shot render`（700 CSS px，DPR 2，高 8600）后 cropbox；页面浅色底 | 正式 |
| `14-not-a-crypto-break.png` | “What this means” 整节（“This is not a cryptographic break …”） | 同上 | 正式 |
| `15-open-questions-candidate-876.png` | “Open questions” 首段（876 号候选）+ 候选分布图（含图例、坐标轴） | 同上 | 正式 |
| `16-skipped-candidates-table.png` | 候选属性表（含图例、全部行、脚注） | 同上 | 备用（正式候选里的 15 的补充） |
| `17-b163-timestamp-search.png` | “Because the number of skipped candidates is small …”到 “B-163 … remains unexplained … base points” | 同上 | 正式 |
| `18-bounty-authors.png` | “The bounty”（$36,864）与 “Who we are” 署名 | 同上 | 备用 |
| `19-article-head-no-action.png` | 标题、日期、副题与 “There is no action to be taken” | 同上 | 备用 |
| `20-filippo-bounty-claimed.png` | Valsorda 页：标题、日期 16 Oct 2026、申领时刻与捐赠 | https://words.filippo.io/seeds-solved/ ，headless Chrome，浅色 | 备用 |
| `21-filippo-undocumented-criteria.png` | “two additional undocumented criteria”“roughly twice as many valid seeds” | 同上 | 备用 |
| `22-filippo-bernstein-lange-30-bits.png` | “70 bits”“barely 30 bits of freedom” | 同上 | 备用 |
| `23-fips186-2-p256-seed.png` | FIPS 186-2 P-256 参数页（`s = c49d3608 …`），PDF 第 34 页（印刷页码 31） | `barking shot pdf`（poppler，144 dpi，1224 px 宽） | 备用 |
| `24-solinas-email-2015-ark.png` | Bernstein–Lange 论文附录 C：Solinas 2015-06-17 邮件（含 Subject/From/Date/To） | `barking shot pdf` 第 65 页；含 Solinas 与 Bernstein 的公开邮箱，见 evidence 提示 | 备用 |

渲染中间图在 `%TEMP%\barking-shot\26101016-b\`（`anth`、`anthfull`、`anthfull2`、`anthfull1x`、`anthfull8400`、`anthfull8600`、`filippo`、`fips34`、`sc65`），不在仓库内。渲染坑：Anthropic 页含 “Expand” 折叠的代码块与 5 分钟视频；折叠的代码输出被截断，所以没有截代码输出块，改用页内的时间戳图表；同一页在不同高度渲染时，中段版面会因视频区占位而整体偏移，所以最终裁切只以 `anthfull8600` 的 OCR 行坐标为准。页面总高超过 Chrome 单图上限的话会截断，8600 CSS px（17200 px）成功，最后一屏含 “Code to reproduce the seeds” 标题，未到页尾（页尾无需截）。

## 脱敏

- `b-coindesk.html/.txt`：页内登录链接的 `returnTo` 参数含 CDN 估计的访客位置（城市、州/省、边缘节点）。已把 `Irvine`、`sfo1`、`countryRegion=CA` 的值替换为 `REDACTED`，`.txt` 里整个 `returnTo=` 值替换为 `REDACTED`；第一次替换误伤了 SVG 的 `fill-opacity`，所以用 `barking fetch --force` 重取了 coindesk 与 bitcoincom-zh，再用更窄的替换处理。`b-bitcoincom-zh.*` 无地理参数。
- 自查（交付前）：见回报。

## 删除的文件（均为自己生成的失败/试验文件，普通 `rm` 加精确路径）

- `images/ztmp-tail.png`、`ztmp-a.png`、`ztmp-b.png`、`ztmp-c.png`（预览用裁图）。
- 重裁前的旧图：`13-claude-found-seeds-half-hour.png`（下沿裁到卡片圆角，重裁）、`14-not-a-crypto-break.png`、`15-open-questions-candidate-876.png`、`16-skipped-candidates-table.png`（三张先按旧版面坐标裁，发现偏移后重裁）、`21-filippo-undocumented-criteria.png`（首行被切，重裁）、`24-solinas-email-2015-ark.png`（先后两次重裁，最终版保留 Subject/From/Date/To）。
- `sources/b-heat.json`、`b-hn-50122800.json`（第一次 `heat` 运行产物，重跑前删）。
- 被拦截的抓取残留：`b-investing-cn.html/.headers`、`b-investing-en.html/.headers`、`b-riddle.html/.headers`。`b-fetch-log.jsonl` 仍保留这些请求的记录。
- `/tmp/r.json`（Reddit 登录页）。

## 单个文件超过 1 MB

`b-anthropic.html` 约 171 KB；`.html`/`.pdf` 原件（`b-coindesk.html` 1.7 MB、`b-bitcoincom-zh.html` 约 980 KB、`b-safecurves.pdf` 约 1 MB、`b-fips1862.pdf` 约 366 KB、`b-icwork.html` 等）按规则只存本地，不入库。文本存档（`.txt/.json/.md/.py`）均未超过 1 MB（见回报的自查输出）。

## 假设

- 北京时间换算：UTC + 8；HN/Lobsters/fxtwitter 的 UTC 时间已换算。
- Anthropic 页面日期与 `datePublished` 取自存档 HTML；Valsorda 页只写日期 “16 Oct 2026”，没有时刻。
- 搜索引擎摘要给出的发布时间（ic.work、Investing.com 系）未在页面上核实，已在 evidence 中标注。
