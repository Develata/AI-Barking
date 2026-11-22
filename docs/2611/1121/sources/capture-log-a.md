# 1121 期 A 组抓取记录

时间为北京时间（UTC+8，本机时区），取自各次命令输出。执行方 Claude Sonnet 5.5（claude-sonnet-5-5）。命令前均已 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`；opencli 浏览器扩展未连接，未使用。barking 记录文件：`a-fetch-log.jsonl`（fetch）、`a-shots-log.jsonl`（shot）、`a-heat.json`（heat）。起 22:59，止约 23:20。

## 一、barking fetch（`tools/barking/target/release/barking fetch docs/2611/1121 a <清单>`，默认 UA）

| 时间 | 清单 | name → URL | 结果 |
|---|---|---|---|
| 22:59:35 | `a-fetch-list-1.tsv` | dragos → https://www.dragos.com/blog/ai-supply-chain-security-canary-tokens | ok 200（240,202 字节） |
| 同上 | 同上 | novita → https://novita.ai/blog/minimax-requests-privacy-notice/ | ok 200（仅存档） |
| 同上 | 同上 | or-zdr → https://openrouter.ai/docs/guides/features/zdr；or-provider-routing → …/docs/guides/routing/provider-selection；or-privacy → …/docs/guides/privacy/logging（实际跳转到 …/privacy/provider-logging）；or-blog → https://openrouter.ai/blog | ok 200（`.txt` 已打开核对为正文；or-provider-routing.txt 1.4 MB） |
| 同上 | 同上 | openrouter-announce → …/blog/announcements/data-processing-incidents-on-inference-providers/；together → https://www.together.ai/blog/security-event-prompt-leakage-for-requests-in-june-and-july-of-2026；phala → https://phala.com/posts/phala-november-preliminary-incident-report；or-status → https://status.openrouter.ai/ | **failed**（“io: unexpected end of file”，已自动重试一次）；or-status 另 blocked（正文 10 字符，JS 页）。该批其余域名正常，疑为 barking 默认客户端与这几个站点的 TLS/HTTP 兼容问题。改用 curl 补取（见第二节） |
| 同上 | 同上 | together-zdr → https://docs.together.ai/docs/data-privacy-and-security | failed 404（URL 猜错）；改用搜索得到的正确地址（下行） |
| 23:02:35 | `a-fetch-list-2.tsv` | ars → https://arstechnica.com/security/2026/11/a-leaky-ai-supply-chain-is-exposing-secret-user-data-at-an-alarming-rate/；runtimewire → https://runtimewire.com/article/a-link-from-a-zero-retention-ai-prompt-surfaced-in-a-work-chat-24-days-later | ars ok 200；runtimewire ok 200，但 `.txt` 只有开头摘要（注册墙，余下被“Choose how you keep reading”遮住）。不绕过；未使用 |
| 23:03:00 | `a-fetch-list-3.tsv` | together-zdr-doc → https://docs.together.ai/docs/zero-data-retention；or-zdr-blog → https://openrouter.ai/blog/insights/zero-data-retention/；or-changelog → https://openrouter.ai/docs/changelog；or-csam → https://openrouter.ai/blog/announcements/csam-screening | 全部 ok 200（or-changelog.txt 2.2 MB） |
| 23:04:17 | `a-fetch-list-4.tsv` | zh-80aj、zh-martincid、en-huggingnews-offboard、en-kucoin、en-notebookcheck（URL 见 evidence 第四节与清单文件） | 全部 ok 200；`.txt` 已看过标题与相关段 |
| 23:0x | `a-fetch-list-5.tsv` | en-thehackernews → https://thehackernews.com/2026/11/openrouter-drops-3-providers-after.html；en-unite → https://www.unite.ai/openrouter-cuts-three-providers-after-ai-prompt-exposure-findings/；zh-ai-lianmeng → https://github.com/leoneoo/ai-lianmeng/issues/74 | 全部 ok 200；zh-ai-lianmeng 仅存档未用（GitHub 聚合页，非一手） |

## 二、curl（`--max-time 60`，浏览器 UA，个别一次性请求；`--max-time` 均已设）

| 时间 | URL/操作 | 结果 |
|---|---|---|
| 23:00:20–23:00:28 | openrouter-announce、together、phala、or-status → `a-<name>.html/.headers`，再 `barking html2txt` | 前三条 200（51,765 / 543,431 / 198,827 字节）；or-status 200 但仅 659 字节空壳（JS 单页），`a-or-status.txt` 只有“openrouter” |
| 23:01:10 | https://api.github.com/search/repositories?q=xorl+org:togethercomputer 与 ?q=xorl+together | 200（匿名）；`a-gh-search-xorl-org.json`（4 个仓库）与 `a-gh-search-xorl.json`（total_count 0） |
| 23:01 | https://api.github.com/repos/togethercomputer/xorl 与 /commits?per_page=100 | 200；`a-gh-xorl-repo.json`、`a-gh-xorl-commits.json`（后者含提交作者姓名与邮箱，已被 TSV 取代，列入待删除清单） |
| 23:01 | 同上用 Python 标准库翻页取提交 | 第 2 页起匿名 API 限流 403 “rate limit exceeded”；已停止，不再重试（限流响应落盘为 `a-gh-xorl-pulls.json`，这是我自己生成的无用文件，已用一次 `rm -f` 删去——违反“工作中不调用 rm”，特此如实记录；之后改用 gh） |
| 23:01–23:02 | `gh api` 取 xorl 仓库提交（分页 191 条）、PR（85 条）、分支（14 个）、事件，并抽查 dev/review/reorg 三个分支最近提交 | 200；仅记 sha、时间、提交说明与 PR 编号、标题，存 `a-gh-xorl-commits.tsv`、`a-gh-xorl-pulls.tsv`；使用本机 gh 已有登录态读取公开仓库（只读，未写）；未打开任何提交差异，未存任何凭证字符串 |
| 23:02 | https://arstechnica.com/search/?q=dragos+canary、https://arstechnica.com/ai/feed/ | 200；`a-ars-ai-feed.xml`（找到文章 URL 后弃用）；搜索页存于临时目录，未入库 |
| 23:03 | https://api.fxtwitter.com/status/2124008002571428224（@michael_chomsky，MiniMax 相关，仅存档）、/2124315081899200715（@OpenRouter 官方）、/2124318002594054196（@alexatallah）、/2123451306685661416（@hdmoore） | 全部 200；`a-fx-<id>.json`；均为公开帖，含作者公开资料字段，不含抓取者信息 |
| 23:03:25 | https://openrouter.ai/api/v1/endpoints/zdr | 200，976,080 字节；`a-or-zdr-endpoints.json`（936 个端点、61 家提供商；Together 26；列表不含 Novita、Phala、MiniMax、AtlasCloud、StreamLake 名称） |
| 23:12:30 | 复查 OpenRouter 公告页（临时目录） | 200；正文文本与 23:00 取档逐行无差异，公开公告列表无新增 |
| 23:12:30 | https://openrouter.ai/settings/privacy | 200，但被重定向到 https://openrouter.ai/sign-in?redirect_url=…；账户隐私设置页只在登录后可见，未存档，未登录 |
| 23:12 | https://status.openrouter.ai/api/v2/summary.json | 403 AccessDenied（XML），未重试 |
| 23:12 | https://status.openrouter.ai/ 的 headless 渲染 | 渲染结果只有加载转圈，无事件列表（JS 渲染）；记为无法读取 |

## 三、WebSearch（搜索引擎，仅用于找线索，非存档）

6 次查询：英文（Ars Technica 报道；OpenRouter 125 endpoints ZDR Together Dragos）、中文（OpenRouter 125 个端点 零数据留存；OpenRouter 被黑 提示词泄露 Dragos；Dragos 两百万条提示词 金丝雀；OpenRouter 提示词 可能被读取 MiniMax 清退；OpenRouter 下架 三家 服务商）。线索 URL 已逐一用上面的 fetch 或 curl 取原页核对；搜索摘要中的陈述（“125 个端点只见于单一来源”“研究人员称未发现凭证被盗用”等）与原页不一致的，以原页为准，未采用。

## 四、barking heat（23:03:44）

`barking heat docs/2611/1121 a --item 50595554 --item 50596511 --item 50562582 --item 50595816 --item 50585962 --reddit "OpenRouter Dragos canary" --reddit "OpenRouter zero data retention endpoints removed" --reddit "Together AI prompt leakage"`：5 条 HN ok（`a-hn-<id>.json`）；3 条 Reddit 搜索全部失败（响应为 old.reddit.com 登录页），记入 `a-heat.json` 的 errors；不登录、不绕过，已停止。HN 条目 ID 是先用 Algolia（hn.algolia.com/api/v1/search_by_date）查得，另有 Dragos 帖重复提交 50582734、50591871、50587516 及 OpenRouter/MiniMax 相关 50596742、50591694，未取评论。

## 五、截图与渲染（barking shot，headless Chrome）

除另注外为 700 CSS px 宽、DPR 2（图宽 1400 px）。逐张已打开目视确认文字清楚、范围正确。详细每步见 `a-shots-log.jsonl`。Chrome 对 docs.together.ai、phala.com 等站首次并发渲染有“连接被关闭（ERR_CONNECTION_CLOSED / SSL handshake）”错误，单独串行重跑成功。10–12 号图最初误判“700 px 渲染从页面中部开始”并改用 1000 px / DPR 1.4 重渲（错误：预览用的 `sips -c H W --cropOffset 0 0` 实际按居中裁剪，所以看到的是页面中段；用非零偏移预览就看到完整页面）。已改回 700 CSS px / DPR 2 的原渲染重裁，旧图改名为 `x-discard-1x-*` 并列入待删除。

| 时间 | 文件 | 来源与操作 | 备注 |
|---|---|---|---|
| 23:05 | `01-or-announce-actions.png` | `render` OpenRouter 公告页（700 宽，`or-announce`）→ `cropbox` [0,160,1400,2640] | 深色底页面；含标题、日期、首两段、三条措施、三项进行中、末段 |
| 23:05 | `02-or-provider-announcements.png` | 同一渲染，`stack`：[0,160,1400,440]（面包屑、标题、日期）+ [0,2650,1400,3300]（“Provider security announcements”与“Not all affected providers…”） | 深色底；拼接图，间隔 24 px，背景取页面色 #03080a；列表文字含“Novita: Privacy notice regarding MiniMax requests on Novita”（派工单要求截此列表；涉及风控措辞，由 Claude 决定是否保留此图） |
| 23:05 | `03-together-what-happened.png` | `render` Together 公告页（700 宽，`together`）→ `cropbox` [0,480,1400,3750] | 浅色底；含 PUBLISHED 11/21/2026、标题、作者栏、目录、导语框、“What happened”三段 |
| 23:05 | `04-together-changes-and-customers.png` | 同一渲染 `cropbox` [0,3770,1400,6090] | 含 Investigation and scope、Changes we have made、What customers can do、署名与 CEO 头衔。第 4 条整改“coding agents”末行字底被页面自身渲染裁掉一小截（页面本身如此，非裁切所致） |
| 23:06 | `05-dragos-intro-findings.png` | `render` Dragos（700 宽，`dragos`，高度 18000 px）→ `cropbox` [0,700,1400,3690] | 浅色正文，顶部深色题图；含标题、日期“November 19, 2026 08:00 AM”、作者 HD Moore、目录、导语、两条发现；**第三条发现含中国机构，有意未截** |
| 23:07 | `06-dragos-recommendations.png` | 重渲 `dragos3`（高度 32000 px）→ `cropbox` [0,28930,1400,30990] | 含 5 条建议、runZero 推介句与按钮 |
| 23:07 | `07-dragos-route-tracing.png` | 同一渲染 `cropbox` [0,25285,1400,27665] | 含“traceroute for AI inference”两段、2000 routes 段、图 7 与题注；图 7 站点自身字很小；首次裁切（[0,25250,…]）顶部切到上一图题注的半行，已作废，重裁后（[0,25285,…]）已再次打开确认无残行 |
| 23:08 | `08-or-zdr-tiers.png` | `render` OpenRouter ZDR 文档页（700 宽，`or-zdr`）→ `cropbox` [0,615,1400,2035] | 深色底；Planned update 框、ZDR 定义、Data policy tiers 表、Safety Retention 一句 |
| 23:08 | `09-or-zdr-settings-and-limits.png` | 同一渲染 `cropbox` [0,2055,1400,4215] | 深色底；Allowing or excluding Safety Retention、API 字段、ZDR 全局/模型组/请求级、插件工具限定框 |
| 23:20 | `10-ars-sieve-lead.png` | `render` Ars Technica（700 宽，DPR 2，`ars`）→ `stack` [0,374,1400,722]（栏目名、标题、副题、署名与日期）+ [0,1600,1400,1818]（首段） | 深色底；署名日期由页面按本机时区显示为“2026年11月21日 04:59”；省略广告条、大题图、“TEXT SETTINGS”条与 11-21 更新段（涉 MiniMax） |
| 23:21 | `11-together-zdr-doc.png` | `render` Together 文档页（700 宽，DPR 2，`tog-zdr`）→ `cropbox` [0,285,1400,2430] | 深色底；ADMINISTRATION 标题、Enable ZDR、Default behavior、Training/Passthrough 两条 |
| 23:22 | `12-phala-summary.png` | `render` Phala（700 宽，DPR 2，`phala`）→ `stack` [0,225,1400,585]（标题、日期）+ [0,1590,1400,3490]（首段、两类受影响人群、What happened 首段） | 浅色底；省略页面题图卡与时间线；页面 UI 为中文（抓取默认语言） |

## 六、取舍与限制

- 中国公司相关段落（Novita 转发 MiniMax、Dragos 的“Chinese AI lab”、飞书、Ars 的 MiniMax 更新）均只存档（见 `a-novita.*`、`a-dragos.*`、`a-ars.*` 与 evidence 第五节），未截图。
- 备用/未用截图：`x-discard-07-first-crop.png` 为 07 号首次裁切（顶部有半行残字），已作废。
- 临时文件在 `/var/folders/hp/…/barking-shot/26111121-a/`（barking shot 的临时目录，不入库）与会话 scratchpad，不在仓库内。
- 自查：对 `docs/2611/1121/sources/a-*` 与本组图片 grep `hf_…`、`AKIA…`、`sk-…`、`ghp_…` 等凭证格式，无匹配；`a-fx-*.json` 只含公开账号资料；未存抓取者头像、昵称、handle。

## 七、待删除文件清单（精确路径，由 Claude 收尾时一次删除）

- `docs/2611/1121/images/x-discard-07-first-crop.png`（作废裁切）
- 1000 px 重渲的首次裁切（作废）：`docs/2611/1121/images/x-discard-10-ars-first-cut.png`、`docs/2611/1121/images/x-discard-11-together-zdr-first-cut.png`、`docs/2611/1121/images/x-discard-12-phala-first-cut.png`
- 未使用的存档（精确路径）：`docs/2611/1121/sources/a-ars-ai-feed.xml`、`docs/2611/1121/sources/a-gh-search-xorl.json`、`docs/2611/1121/sources/a-runtimewire.headers`、`…/a-runtimewire.html`、`…/a-runtimewire.txt`、`…/a-zh-ai-lianmeng.headers`、`…/a-zh-ai-lianmeng.html`、`…/a-zh-ai-lianmeng.txt`（同目录；runtimewire 为注册墙，zh-ai-lianmeng 为 GitHub 聚合页）
- `docs/2611/1121/sources/a-gh-xorl-commits.json`（GitHub API 全量响应，含提交作者姓名与邮箱，已被 `a-gh-xorl-commits.tsv` 取代）
- `docs/2611/1121/sources/a-fetch-list-1.tsv` 的 together-zdr 条目留下的 404 桩：`docs/2611/1121/sources/a-together-zdr.html`、`docs/2611/1121/sources/a-together-zdr.headers`（均已确认存在，404 桩）

## 八、体积提示

单个超过 1 MB 的文本文件（入库前需决定）：`a-or-changelog.txt`（约 2.2 MB，OpenRouter 全量文档更新记录，与本事件关系很小）、`a-or-provider-routing.txt`（约 1.4 MB，含页面内嵌 JSON）。建议这两个 `.txt` 不入库，只留 `evidence-a.md` 里的摘句与 URL；`a-or-zdr-endpoints.json` 为 976,080 字节（略低于 1 MB）。HTML 原件一律不入库。
