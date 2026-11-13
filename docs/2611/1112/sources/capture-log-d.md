# 抓取与检索日志：D 组（速览，1112 期）

时间为北京时间（UTC+8），表中时刻为会话内约略记录，精确时间以 `d-fetch-log.jsonl` 与 `d-heat.json` 为准。`barking heat` 因 `d-heat.json` 已存在而拒绝覆盖，每次先 `mv` 旧文件再重跑，旧文件保留为 `d-heat-run*.json`。“存档”指写入 `docs/2611/1112/sources/`；“线索”指只用于定位一手来源、未据此引用的检索结果。无截图。`sources/d-fetch-log.jsonl` 由 `barking fetch` 自动追加，与下表互相对应。

| 时间 | 工具 | 对象 | 结果 | 备注 |
|---|---|---|---|---|
| 22:28 | opencli weixin search | “Hy Realtime 腾讯混元 实时语音” | ok，返回 6 条，URL 为搜狗跳转链接 | 存 `d-hy-sogou-search.json`（首次），`d-hy-sogou-search2.json`（再次检索，链接 token 已变） |
| 22:29 | curl（搜狗第 1 条） | `weixin.sogou.com/link?url=…` | 302 到 `weixin.sogou.com/antispider/` 验证页（HTTP 200，验证页） | **blocked**：验证页不绕过，记失败。中间文件 `d-hy-sogou-r1.txt` |
| 22:31 | opencli weixin download（搜狗链接） | 同上 | `invalid URL`（该命令只接受 mp.weixin.qq.com 链接） | failed |
| 22:36 | opencli browser open（session `d-wx`，background） | 搜狗第 3 条链接 | 真实 Chrome 跟随跳转到 `mp.weixin.qq.com`（标题“腾讯混元发布Hy Realtime实时语音模型，支持联网检索与歌唱”，账号“适当的生活节奏”，11-13 21:42） | 账号非腾讯混元，不作一手；只用于拿到 mp 链接 |
| 22:37 | opencli browser open + eval（sessions `d-wx1`、`d-wx2`、`d-wx5`、`d-wx6`） | 搜狗第 1、2、5、6 条 | 同样跳转到 mp.weixin.qq.com；第 2 条账号“腾讯混元”，2026-11-13 16:32 | 确认官方公众号文章 |
| 22:37 | opencli weixin download | 腾讯混元公众号文章（mp 链接，`d-wx-official-url.txt`） | ok，9.8 KB，存为 `sources/d-wx-hy-official/` 下 .md（正文文本，`--download-images false`） | 图片另行 curl 到 `d-wx-hy-img0..3.png`（本地，不入库）；`d-wx-hy-imgurls.txt` 为图片地址清单 |
| 22:37 | curl | 公众号图片（`d-wx-hy-img1.png`、`img2.png` 等） | 200，image/png | 本地读取图件，数字为目视读取；`img0`、`img3` 未目视 |
| 22:29 | barking fetch（清单 `d-list1.tsv`） | elevenlabs.io/blog/scribe-v3 | ok，450 KB，392 行 txt | 文件名为 `d-d-scribe-v3-blog.*`（清单名带了组前缀 d，形成双前缀，保留不改，日志与文件对应） |
| 22:30 | barking fetch（`d-list2.tsv`） | deepmind.google/blog/gemini-robotics-2-5-accelerates-real-world-physical-intelligence | ok，191 KB，727 行 txt | `d-dm-gr25-accelerates.*`。WebFetch 先从 `/blog/` 列表定位 URL（线索） |
| 22:30 | barking fetch（`d-list3.tsv`） | artificialanalysis.ai/speech-to-text/non-streaming | ok，1.77 MB，1033 行 txt | 快照 22:30；Scribe v3 1.9% 与 Fun-Realtime 1.7% 见 txt 第 962 行附近 |
| 22:30 | barking fetch（同上） | artificialanalysis.ai/speech-to-text/models/elevenlabs-scribe-v3（猜测 URL） | **404** | 保留 `d-aa-stt-scribe-v3.*` 作为失败记录（待删除清单） |
| 22:33 | barking fetch（`d-list4.tsv`） | artificialanalysis.ai/speech-to-text/models/elevenlabs | ok，1.5 MB，957 行 txt | 静态文本中未见 Scribe v3 行 |
| 22:31 | WebSearch（线索） | ElevenLabs Scribe v3；AA 相关页；DeepMind Mercedes；Hy Realtime 中英文 | 线索 | 搜索摘要不作引用，引用均回到存档文本 |
| 22:32 | WebFetch | deepmind.google/blog/ | 列表页，定位到“November 10, 2026”的 Gemini Robotics 2.5 条目 | 线索 |
| 22:34 | WebFetch | hunyuan.tencent.com | 首页快照，无 Hy Realtime 条目 | 线索，未存档 |
| 22:37 | opencli browser open + eval（session `d-hy`） | hunyuan.tencent.com、hunyuan.tencent.com/news | 首页（快照，关键词 Realtime/实时语音/61.01 均无命中）；`/news` **404** | 快照时点 22:37 |
| 22:38 | barking fetch（`d-list5.tsv`，3 路并发） | develata.me/news/AI_ML/2026/20261112 与 20261113 | **404** | 路径错误，已改用 `/daily/news/…` |
| 22:38 | barking fetch（同上） | aihot.news | 请求失败（协议错误） | 同时 curl 复测为 000，无法连通，记失败 |
| 22:38 | barking fetch（同上） | apnews.com（Huang 辞任） | **403**，Cloudflare 质询 | blocked，不绕过 |
| 22:38 | barking fetch（同上） | status.claude.com/incidents/nn11wk485zfn | ok，73 KB，250 行 txt | 事故状态页 |
| 22:39 | curl | develata.me/daily/news/AI_ML/2026/20261112、20261113 | 200 | 存 `d-dz-daily-1112.html`、`d-dz-daily-1113.html`（线索 L5，不引用） |
| 22:40 | curl | news.ycombinator.com（首页） | 200 | 存 `d-hn-front.html` |
| 22:41 | barking fetch（`d-list6.tsv`） | ipo.anthropic.com/us/；cnbc（IPO 价格区间） | ok 两条（51 KB；855 KB） | 用于 D4 判断 |
| 22:43 | WebFetch | openai.com/news/ | **403** | 未存档 |
| 22:43 | WebFetch | anthropic.com/news | listing；最新条目 Nov 10（roadshow）、Nov 9（Fable 5.2 / Mythos 5.2） | 线索，未存档 |
| 22:43 | WebFetch | blog.google/technology/ai/ | 无条目内容 | 线索，未存档 |
| 22:44 | WebFetch | microsoft.com/en-us/research/blog/ | listing 确认 Lazarus 文章（Nov 10，无时刻），仅导语 | 线索；窗口外，未存档全文 |
| 22:44 | WebSearch | NVIDIA GR00T 2、Meta Muse、Mistral、xAI Grok 4.6、Qwen/DeepSeek/Kimi 等 | 线索，无窗口内新发布 | 线索，未存档 |
| 22:32、22:46 | barking heat（两次有效运行，见下） | HN（Scribe v3 / Gemini Robotics 2.5 / Hy Realtime / Claude 事故 / Jensen Huang / Anthropic IPO）；Reddit（Scribe v3、Gemini Robotics 2.5 Mercedes） | HN 正常；**Reddit 两次均返回 old.reddit 登录页**，记失败，未重试 | `d-heat-run1.json`（首轮，仅 Scribe/Gemini/Hy）、`d-heat-run2.json`（第二轮）、`d-heat.json`（最终，覆盖 6 组 HN） |

## 失败与未取

- 腾讯混元公众号原文：已取到，但官网无对应条目。
- Reddit 热度：两组检索均返回登录页，未取到。
- AP（Huang 辞任）：403 质询，未取到正文，因此 D4 对该条只能按 HN 标题与 URL 判断，已在 evidence-d.md 说明。
- aihot.news：连接失败，未能打开。
- develata.me 日报页：已取到，但只作线索（L5）。
- OpenAI newsroom：403。
- MSR Lazarus 与 NVIDIA GR00T 2：窗口外，只取 listing 或次级线索，未存档全文。
