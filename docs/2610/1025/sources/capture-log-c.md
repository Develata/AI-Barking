# 1025 期 C 组抓取日志（速览）

时间均为北京时间（UTC+8），为取证过程的约略时刻（精确到分钟者以 fetch-log 与 heat 输出为准）。执行方：claude-haiku-5-5。未登录任何账号、未输入凭据、未接受 Cookie；只读。无截图（C 组按派工单不截图）。

## 批量存档（barking fetch，清单 `c-list.tsv`、`c-list-ibm2.tsv`；每条结果见 `c-fetch-log.jsonl`）

| 时刻 | name | URL | 结果 |
|---|---|---|---|
| 20:33 | c-register | https://www.theregister.com/systems/2026/10/22/ibm-ceo-says-open-weight-models-are-now-irresistibly-good-and-cheap-on-prem/5304640 | ok（首次连接失败，已重试；`c-c-register.html/.txt`） |
| 20:33 | c-ibm-q3 | https://www.ibm.com/investor/earnings/3q2026 | ok（`c-c-ibm-q3.html/.txt`） |
| 20:33 | c-peerless-page | https://gking.harvard.edu/peerless/ | ok（`c-c-peerless-page.html/.txt`） |
| 20:33 | c-peerless-pdf | https://gking.harvard.edu/files/peerless.pdf | ok，PDF（`c-c-peerless-pdf.pdf/.txt`） |
| 20:33 | c-wiz | https://www.wiz.io/blog/gheist-attempted-scaled-abuse-of-stolen-github-tokens-stumbles | ok（`c-c-wiz.html/.txt`） |
| 20:33 | c-thinkrail | https://github.com/JetBrains/thinkrail | ok（`c-c-thinkrail.html/.txt`） |
| 20:33 | c-nanbeige | https://huggingface.co/Nanbeige/Nanbeige4.3-33B | ok（`c-c-nanbeige.html/.txt`） |
| 20:33 | c-nanbeige-api | https://huggingface.co/api/models/Nanbeige/Nanbeige4.3-33B | ok，JSON（`c-c-nanbeige-api.json`） |
| 20:33 | c-capcut-fx | https://api.fxtwitter.com/status/2114062937094823955 | ok，JSON（`c-c-capcut-fx.json`；原 X 帖未直接访问） |
| 20:33 | c-capcut-codex | https://www.capcut.com/tools/capcut-x-codex | ok（`c-c-capcut-codex.html/.txt`） |
| 20:33 | c-dailycal | https://www.dailycal.org/news/campus/academics/ai-use-leads-uc-berkeley-math-professor-to-see-no-point-in-grading-homework/article_bdf1eae2-c08b-491f-b9f1-81a7cea177de.html | ok（`c-c-dailycal.html/.txt`） |
| 20:33 | c-hf-thinkrail-api | https://api.github.com/repos/JetBrains/thinkrail | ok，JSON（`c-c-hf-thinkrail-api.json`） |
| 20:34 | c-ibm-q3-remarks | https://www.ibm.com/downloads/documents/us-en/17f55bddaac4eb80 | ok，PDF（首次失败，已重试；`c-c-ibm-q3-remarks.pdf/.txt`）。Prepared remarks 中无该句 |

## 其他请求

| 时刻 | 工具 | 对象 | 结果 |
|---|---|---|---|
| 20:36 | WebFetch | meta.com 博客 `an-update-on-meta-for-work` | SSL 解密错误，失败；改 curl（下一行） |
| 20:37 | curl --max-time 60 | 同上 | 200，`c-meta-blog.html`（2026-01 旧文，与 C7 无关，待删除） |
| 20:37 | WebSearch（两次） | Meta 条款生效日 / 官方页 | 检索结果均报 10/30 生效，未返回 Meta 官方条款页，未见 11/20 |
| 20:40 | WebFetch | meta.com 博客（再次） | 失败（同上未返回有效内容） |
| 20:41 | curl | niemanlab.org 报道 | 000，失败，未入库 |
| 20:44 | opencli reddit read 1xelcuf | r/opensource 原帖 | 成功，JSON（`c-reddit-opensource-1xelcuf.json`）；score 字段返回 0，未采信 |
| 20:44 | opencli reddit subreddit-info opensource | r/opensource 社区规模 | 394350 订阅 |
| 20:44 | opencli reddit search | 原帖与转载 | 原帖 2580 评论；另有 r/fossdroid 等转载 |
| 20:45 | curl GitHub releases | JetBrains/thinkrail releases | 200，`c-c-thinkrail-releases.json`（最近三条为 nightly） |
| 20:46 | curl GitHub commits?page=563 | 最早提交 | 200，`c-c-thinkrail-firstcommit.json`（2026-06-22T16:58:26Z，initial specs） |
| 20:47 | curl --max-time 60 | theverge.com 报道 | 200，`c-c-verge.html`（Oct 25, 2026 1:25 PM UTC） |
| 20:47 | curl | thinkrail.ai | 000，失败；未生成文件 |
| 20:48 | WebSearch | IBM Q3 2026 电话会转录 | 未返回 Q3 2026 转录 |
| 20:48 | WebSearch | Kotaku / Help Net Security / Verge 关于 r/opensource 的报道 | 未找到 Kotaku、Help Net Security 报道；Verge 报道已由 HN 链接确认 |

## 热度（barking heat）

| 时刻 | 调用 | 结果 |
|---|---|---|
| 20:42 | `heat … c --hn "IBM CEO open weight models" --hn "Gheist GitHub tokens" --hn "ThinkRail" --hn "Nanbeige" --hn "Borcherds grading homework" --hn "gking peerless AI reviews" --hn "Meta commercial terms" --hn "CapCut ChatGPT plugin" --reddit "your tokens arent valuable"` | 写入 `c-heat.json`；Reddit 搜索被登录页拦截（失败，已记入 errors）。后该文件被 mv 改名为 `c-heat-part1.json` 以便再写第二批 |
| 20:47 | `heat … c --hn "r/opensource AI" --hn "Your tokens aren't valuable" --hn Borcherds --hn "Berkeley homework AI grading" --hn "Gary King AI peer review" --hn CapCut --hn Gheist` | 写入 `c-heat.json`（第二批，HN 7 组）。Borcherds、Gary King、CapCut、Meta 关键词命中 0 条 |

注：`barking heat` 对已存在的 `c-heat.json` 拒绝覆盖，因此第一批改名为 `c-heat-part1.json`（mv，未删除）。两份文件合起来才是本组全部 HN 热度。

## 失败与未取到

- Meta 官方条款页与公告：未取到（C7 无一手来源）。
- IBM 电话会转录：未取到（C1 只有 L4 转述）。
- Kotaku、Help Net Security 报道：未找到（C3）。
- thinkrail.ai：连接失败（C5）。
- Nieman Lab：连接失败（C7 的转述，未入库）。
- Reddit 原帖 created_utc：未取到（C3）。

## 待删除文件（精确路径，由 Claude 收尾时一次删除）

- `E:\gitclone\AI-Barking\docs\2610\1025\sources\c-meta-blog.html`（与 C7 无关的 Meta 旧博客）
- `E:\gitclone\AI-Barking\docs\2610\1025\sources\c-reddit-err.txt`（opencli 升级提示的 stderr）
