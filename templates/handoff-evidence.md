# 取证任务：{MMDD} 期（{TITLE_GROUPS}）

## 目标

为“AI 吠点”{MMDD} 期取证：把下列说法逐条回溯到一手来源，存档页面、截取候选配图，产出事实清单。你只负责取证，不写正文、不下结论。账号的核心卖点是“识破吠点”，事实错误是最贵的失败，所以宁可标“未找到”，也不要凑。

本文件分{GROUP_COUNT_ZH}组（{GROUP_LETTERS}），各由一个执行方并行做。**你只做派给你的那一组**（派工消息会写明）；其他组的文件不要碰。

## 先读

仓库 `{REPO}`（分支 `{BRANCH}`，基线 commit `{BASELINE}`）。{UNTRACKED}。{DIRS_NOTE}。

1. `AGENTS.md`：交付格式。
2. `EDITORIAL.md`：重点读“事实分级”（L1–L7，含平台名一条）与“速览”两节。额度重置不进速览，也不要取证。
3. `templates/codex-evidence.md`：取证要求（2 倍像素比截图、官方图表截全、evidence 表格格式与状态用词、不暴露抓取者信息）。路径里的 `{{YYMM}}/{{MMDD}}` 即 `{YYMM}/{MMDD}`。
4. 存档、截图、热度统计用仓库自带的 `barking` 子命令，不要为同一件事另写脚本：`barking fetch <期次目录> <组前缀> <清单文件|->`（按 `name<TAB>url` 清单批量存档网页与 PDF：正文、`.headers`、`.txt` 一次写好，限时并发，结果记入 `sources/<组>-fetch-log.jsonl`；blocked/failed 条目改用 opencli 取；curl 只用于个别一次性请求）、`barking html2txt <in.html> <out.txt>`（HTML 存档转文本）、`barking shot <期次目录> <组前缀> render|pdf|lines|crop|cropbox|stack`（headless Chrome 截图、PDF 渲页、按锚点或像素框裁切与拼接，每次动作自动追加到 `sources/<组>-shots-log.jsonl`）、`barking heat <期次目录> <组前缀>`（Hacker News 与 Reddit 热度）。调用形式 `cargo run --release --manifest-path tools/barking/Cargo.toml -- <子命令> …`，用法见不带参数运行的输出。上期的存档与截图记录可参考（只读，不改原文件）：{PREV_ISSUES}。本期自写的脚本放到 `docs/{YYMM}/{MMDD}/sources/`，文件名以组号开头（{PREFIXES}）。踩过的坑：HTTP 200 不等于拿到正文（可能是 challenge 页或付费墙）；WebFetch 对 businessinsider.com、theverge.com、arstechnica.com 被拒，改用 curl 或 firecrawl_scrape（firecrawl 额度有限，能用 curl 就别用它；firecrawl 的 `query`/`directQuote` 返回的句子曾不在原页上，只当线索，引用前 grep 存档）；X 帖子与 X 长文可用 `https://api.fxtwitter.com/status/<id>` 取公开 JSON（不含抓取者信息），或 X 官方嵌入 `https://platform.twitter.com/embed/Tweet.html?id=<id>` 在匿名 headless Chrome 渲染后截帖子卡片；PDF 渲页截图用 `barking shot pdf`（内部调用 Scoop 的 poppler），转文本存档用同一 poppler 的 `pdftotext -enc UTF-8 -layout`；headless Chrome：`--headless=new --user-data-dir=<临时目录> --force-device-scale-factor=2 --window-size=700,H --timeout=30000`，输出用 Windows 绝对路径，重页面别用 `--virtual-time-budget`；需要审批而被拒的操作如实记录，不要硬绕。
5. 本期要避免与之重复的已发内容：`daily-scan.md` 文末“已发过”与“近 3 天速览已收”{UNPUBLISHED}。

## 通用规则

- URL 一律去掉 utm_source 等查询参数。时间一律写北京时间（UTC+8），括号保留原时区；官方页只有日期时如实写“官方未给时刻”。本机时区为美东（{LOCAL_TZ}），北京时间 = 本机时间 + {BJ_SHIFT} 小时。
- 下面的“已知情况”来自两份扫描回贴（ChatGPT、WorkBuddy）与 Claude 的初步核对，**可能有错**。你必须独立复核；与你存档不符的，以原文为准，并写进“扫描说法勘误”。
- 每条说法按 `templates/codex-evidence.md` 第 4 条格式记一行，状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。逐字摘句用原语言原文。
- 同一指标不同来源数值不一致时，全部记录，不要自行取舍。
- 图片（仅 {SHOT_GROUPS} 组）：每组在自己的编号区间内用 `NN-简短英文名.png`（{IMAGE_RANGES}），2 倍像素比，宽度不超过 1400 px，保留完整背景与上下文（标题、前后文、图表标题与图例），不要只截一行；裁切边界落在文字行之间。不重绘、不改数字、不用转载图。深色底页面也可以截，但在日志里注明。{ROUNDUP_SHOT_NOTE}
- 平台名：X 等大陆无法访问的平台上的帖子，只截帖子卡片本身；存档写盘前把抓取者的头像链接、显示名与 handle 替换为占位符；只在登录态可见的页面不存档。交付前对 `docs/{YYMM}/{MMDD}/` grep 一遍确认无泄露。
- 私人个体：只记录其公开的职业身份与公开发言，不收集住址、家庭等个人信息。
- 浏览器已登录会话可以使用，但不要自行登录、输入凭据或接受非必要 Cookie；不发帖、不点赞、不评论、不关注。**付费墙不绕过**，打不开就记失败并继续；不用镜像/转载站或代理域名（web.archive.org 快照只可用于查看“某页面以前写了什么”，须注明）。不要运行会读取本机浏览器 Cookie 的工具。
{ISSUE_NOTES}

## 授权与范围

- 只在 `docs/{YYMM}/{MMDD}/` 下新增文件，文件名带本组前缀；不改动仓库其他文件；不 commit、不 push、不删除任何文件（自己刚生成的试验文件除外，须记入日志）。
- 不写 `_publish.txt`、`images/README.md`、`images/cards.toml`、`fact-check.md`、`sources/README.md`。
- 每组交付 `docs/{YYMM}/{MMDD}/sources/evidence-<组>.md` 与 `docs/{YYMM}/{MMDD}/sources/capture-log-<组>.md`（每次抓取的 URL、工具、结果/失败原因、北京时间）；Claude 事后合并。
- PDF、HTML 原件、图片、音视频只存本地（不入库，Claude 事后传网盘）；文本存档（txt/md/json）可以入库，但单个文件超过 1 MB 的先在回报里告知。
- 时间与范围（2026-10-15 起，Develata 要求提速；1013 期 A/B/C 组各跑了约 3 小时）：每组目标 60 分钟内交回，超过 90 分钟就先交已完成部分并在回报里列出未完成项。只截正文可能用到的图（每组 3–6 张）；媒体报道每个事件只存 3–5 篇代表性的（首发、最权威、最典型的夸大实例），不逐家归档；社区热度只记主帖数字与 2–3 条有理有据的评论。

## 验收

- 本组清单每条在 `evidence-<组>.md` 有一行，状态为四种之一。
- 所有被引截图在 `docs/{YYMM}/{MMDD}/images/` 本组区间内实际存在，并已逐张打开目视确认文字清楚、范围正确。
- 另附本组的“可能的吠点”（原文里有、但线索摘要没提或说反的条件与限制，每条一句注明出处；再扫该事件在 Hacker News、Reddit 等社区的高赞评论，摘出有理有据的质疑，注评论链接与得分，仅作线索 L6）、“夸大说法实例”（把事件写歪的中英文标题或文章：原站 URL、原句逐字、发布时间、发布方；找不到就写找不到）、“扫描说法勘误”三节。{ROUNDUP_ACCEPT_NOTE}
- `git status --short` 中只新增 `docs/{YYMM}/{MMDD}/` 下本组文件（及本派工单）{KEEP_CHANGES}。

## 回报

最终消息写：你的模型 ID；新增文件列表；本组关键发现（含与“已知情况”不符之处）；各发布时刻；失败的抓取；你做的假设。

---

{GROUP_SECTIONS}
<!-- skeleton:main -->
# {GROUP} 组：{TOPIC}

主帖第{NTH}条。

## 已知情况（Claude {DATE} 已读，须独立复核）

- （待填：Claude 已读到的事实与出处；执行方须独立复核）

## 取证清单

| # | 说法 | 要求 |
|---|---|---|
| {GROUP}1 | （待填） | （待填） |

候选配图（{GROUP} 组 {RANGE}）：（待填）
<!-- skeleton:roundup -->
# {GROUP} 组：{TOPIC}

速览聚合图的非主帖条目，规则见 `EDITORIAL.md`“速览”一节：每条只写事实与限定词，来源 L1–L4。**不需要截图**；每条给：一句事实（照原文措辞，保留限定词）、一手来源 URL 与存档文件（html + txt）、发布时刻（北京时间与原时区）、级别、能改变含义的限定词原句、`barking heat` 热度（分数/评论/时刻）。找不到一手材料的写“未找到”。

| # | 条目 | 已知线索（须核） | 要求 |
|---|---|---|---|
| {GROUP}1 | （待填） | （待填） | （待填） |

{GROUP} 组另做：对每条用 `barking heat` 记热度（分数/评论/时刻）。**不收**：（待填：本期已决定不收的条目与原因）
