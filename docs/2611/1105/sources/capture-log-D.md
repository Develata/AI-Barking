# 1105 期 D 组（速览）抓取与热度日志

时区：北京时间（UTC+8）。抓取日志机器可读版见 `d-fetch-log.jsonl`；热度见 `d-heat.json`。本组没有截图，也没有 opencli 或 headless Chrome 操作。

## 网页与 PDF 存档（`barking fetch`，清单 `d-fetch-manifest.tsv`）

开始于 2026-11-06T19:33:00+08:00，并发 6，同域名至多 2。

| name | URL（去查询参数） | 结果 | 字节 | 备注 |
|---|---|---|---|---|
| d1-inkling-official | https://thinkingmachines.ai/news/inkling-2-small/ | 200 ok | 740,294 | |
| d1-aa-article | https://artificialanalysis.ai/articles/thinking-machines-releases-inkling-2-small-highest-scoring-open-weights-model-outside-of-chinia-upon-weights-release-it | 200 ok | 385,129 | 派工单 URL 原样使用，其中 “chinia” 为原 URL 拼写 |
| d2-metr-review | https://metr.org/blog/2026-11-05-rd-section-anthropic-risk-report-august-2026-review/ | 200 ok | 80,701 | |
| d3-cursor-changelog | https://cursor.com/changelog/build-live-apps | 200 ok | 122,535 | |
| d3-cursor-docs-apps | https://cursor.com/docs/cloud-agent/apps | 200 ok | 245,085 | |
| d4-meta-about | https://about.fb.com/news/2026/11/introducing-muse-ai-voice-expressive-avatars/ | 200 ok | 459,070 | curl 直接取到正文，未改用 opencli |
| d5-linum-fieldnote | https://www.linum.ai/field-notes/pjit-21k | 200 ok | 345,108 | |
| d6-pangram | https://www.pangram.com/blog/pangram-image-1 | 404 failed | 67,656 | 派工单 URL 不存在，无 txt；留作失败记录 |
| d6-pangram-new | https://www.pangram.com/blog/introducing-pangram-image-1-0 | 200 ok | 232,103 | 原 URL 404 后改用，来自 HN 讨论指向的地址（19:34:53 单独抓取） |
| d7-openclaw | https://openclaw.ai/blog/shattering-silos-with-openclaw-multiplayer | 200 ok | 55,262 | |
| d8-openai-gpt6cyber | https://openai.com/index/introducing-gpt-6-cyber/ | 200 ok | 522,859 | curl 直接取到正文，未改用 opencli |
| d9-cnn | https://www.cnn.com/2026/11/05/investing/tech-heavy-nasdaq-plunges | 200 ok | 5,670,565 | html 原件约 5.7 MB，仅本地存放，不入库；`.txt` 28 KB |
| d9-guardian | https://www.theguardian.com/business/2026/nov/05/nasdaq-heading-for-worst-day-in-18-months-as-wall-street-sustains-widespread-losses | 200 ok | 355,603 | |

汇总：第一次批量 ok 11、failed 1（d6-pangram 404）；第二次单独抓 d6-pangram-new，ok。无 blocked。

文本存档超过 1 MB 的：无（最大为 `d-d1-inkling-official.txt` 51 KB）。

## AP（apnews.com）

- 派工单要求：打不开就记失败。
- 尝试：先用 `curl` 访问 https://apnews.com/，并带一个浏览器样式的 User-Agent。该 Bash 命令被权限系统以“机器人检测绕过”为由拒绝，未执行，也没有产生 `d-ap-root.html`。
- 处理：不改用其他 UA 或代理重试，记为失败。
- 搜索：WebSearch 查 “AP Nasdaq worst day November 5 2026 AI stocks fall”，未找到 2026 年 11 月 5 日的 AP 报道；返回的 AP 相关条目日期不明或为其他日期。
- 结论：AP 一手来源未取到，D9 不以 AP 为依据。

## 其他非网页抓取

- TechCrunch 同日 Muse 报道：WebSearch 查询 “TechCrunch Meta Muse voice expressive avatars November 2026”，只返回二手转述，无 TechCrunch 原页 URL，未存档，D4 中相关说法记为未找到一手来源。
- Pangram 与 AP 的 WebSearch 结果只作线索，不作证据。

## 热度（`barking heat docs/2611/1105 d`，写入 `d-heat.json`）

取得时刻 2026-11-06T19:34:12+08:00；HN 搜索自 2026-10-30T19:34:12+08:00 起。Reddit 本组未查（未给 --reddit 查询）。

| 查询 | 命中 | 取前 | 记录 |
|---|---|---|---|
| Inkling 2 Small | 2 | 2 | 50386338 · 213 分 / 70 评 · 11-06 08:00 · thinkingmachines 官方页；50388824 · 2 分 / 1 评 · 11-06 14:18 · AA 文章 |
| METR Anthropic risk report | 0 | 0 | |
| Cursor build live apps | 3 | 3 | 三条均与 Cursor 无关（Ask HN、Kino AI、Hanging Context），不作为热度记录 |
| Muse voice avatars Meta | 0 | 0 | |
| Linum 21K text-to-image | 1 | 1 | 50368208 · 239 分 / 22 评 · 11-05 00:45 · Show HN |
| Pangram Image | 2 | 2 | 50383294 · 113 分 / 54 评 · 11-06 03:27；50368458 · 5 分 / 0 评 · 11-05 01:03（旧 URL 的重复提交） |
| OpenClaw Multiplayer | 0 | 0 | |
| GPT-6 Cyber | 2 | 2 | 50381590 · 54 分 / 7 评 · 11-06 01:07 · openai 官方页；50381914 · 21 分 / 5 评 · 11-06 01:37 · X 镜像 |
| Nasdaq worst day AI | 1 | 1 | 50390352 · 3 分 / 5 评 · 11-06 17:55 · Guardian 文 |

第二次替代查询（METR、Muse Spark voice、OpenClaw Multiplayer、Muse voice、Inkling）：`barking heat` 因 `d-heat.json` 已存在而拒绝覆盖（工具设计如此），未再运行。改用 HN Algolia 只读接口（curl，`tags=story`，`numericFilters=created_at_i>1793000000`，约 2026-10-30 起）补查：
- “METR Anthropic risk report”、“OpenClaw Multiplayer”：0 条。
- “Muse voice avatar”：50383465 · 1 分 / 0 评 · 11-06 03:41 · “Give Your Muse a Voice”（about.fb.com 页）。
- “Inkling 2 Small”、“Linum 21K”：与上表一致。

注：上面的补查结果是临时 curl 输出，没有写入文件，本日志只记录结果。

## 其他记录

- 1104 期比对：`docs/2611/1104/sources/a-openai-paths-to-root.txt` 第 51 行，“upcoming cyber model”（用于 D8 的关系判断）。
- 查重：`daily-scan.md` 的“已发过”表中，无 Inkling、METR、Cursor Live Apps、Muse 语音、Linum、Pangram、OpenClaw Multiplayer、GPT-6 Cyber、11 月 5 日美股的条目。只有 1003 期的 “Meta Muse-Spark 与数学家合作”，与 Muse 语音无关。
- 未用 opencli；未用 headless Chrome；未截图；未访问任何需要登录的页面。

## 待删除文件（本组自己生成，待 Claude 收尾时一次删除）

- `/home/develata/gitclone/AI-Barking/docs/2611/1105/sources/d-fetch-manifest.tsv`（第一次抓取清单，内容已记入 `d-fetch-log.jsonl`）
- `/home/develata/gitclone/AI-Barking/docs/2611/1105/sources/d-fetch-manifest-d6b.tsv`（Pangram 新地址的单条清单，同上）

以下文件保留，不建议删除：`d-d6-pangram.*`（404 失败记录）、`d-heat.json`（热度原始数据）、`d-fetch-log.jsonl`（抓取日志）。
