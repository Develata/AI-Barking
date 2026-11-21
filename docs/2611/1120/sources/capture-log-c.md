# 1120 期 C 组抓取日志（速览）

时刻均为北京时间（本机即 UTC+8；本机时钟读数 18:25–约 18:50）。机器日志：`c-fetch-log.jsonl`（`barking fetch`，共 66 条）、`c-heat.json`、`c-heat-2.json`、`c-heat-3.json`（`barking heat`）。清单文件 `c-fetch-list-1.tsv` 至 `c-fetch-list-15.tsv`（`c-fetch-list-3` 起每份附于对应批次）。
工具：`barking fetch`、`barking heat`、curl（`--max-time 60`，fxtwitter JSON、GitHub API、Mastodon API、Hugging Face API、HN Algolia、arXiv API）、`pdftotext`/`pdfinfo`/`shasum`、WebSearch（只用来发现二手稿与 Google 澄清帖的存在，搜索摘要不作证据）。
未登录任何站点，未输入任何凭据，未接受 Cookie 提示；未读取浏览器 Cookie；未用 opencli（扩展未连接）；不截图。X 帖只取 fxtwitter 公开 JSON，其中只含官方账号（@ChatGPT、@ValsAI、@NewsFromGoogle）的公开推文数据，无抓取者信息。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 18:25:28 | Tao 博文、Euler 论文 PDF（cims.nyu.edu）、EulerBlowupPublic 主页与 README、navier-stokes 主页与 README（raw.githubusercontent.com）、Inkling 2 Mini 官方页与 /news/、METR 博文、Mother Jones 谷歌文、HN 50581260、HN 50577901、fxtwitter ChatGPT 帖 | barking fetch（清单 1） | 13 条全 ok：`c-tao-blog.*`、`c-euler-pdf.*`、`c-euler-gh.*`、`c-euler-gh-readme.*`、`c-ns-gh.*`、`c-ns-gh-readme.*`、`c-inkling-official.*`、`c-inkling-news.*`、`c-metr-blog.*`、`c-mj-google.*`、`c-hn-inkling.*`、`c-hn-google.*`、`c-fx-chatgpt.*` |
| 18:25:55 | NS 论文 PDF、`lean/README.md`、`lean/NOTICE`、Euler `formalization/README.md`、Buckmaster Mastodon 帖与 API、`huggingface.co/thinking-machines/Inkling-2-Mini` | barking fetch（清单 2） | 6 ok：`c-ns-pdf.*`、`c-ns-lean-readme.*`、`c-ns-lean-notice.*`、`c-euler-form-readme.*`、`c-mastodon-bm.*`、`c-mastodon-api.json`；`inkling-hf` 401 blocked（我猜错了组织名，实为 `thinkingmachines/Inkling-2-Mini`，见下一批） |
| 18:26 | GitHub API：两仓库提交列表与仓库信息 | curl | ok：`c-gh-commits-mcoiculescu_EulerBlowupPublic.json`、`c-gh-commits-tristanbuckmaster_navier-stokes.json` |
| 18:26 | arXiv API：作者 Buckmaster+Alpöge、标题含 Euler+Coiculescu | curl | ok：`c-arxiv-search-buckmaster.xml`（另一次检索未落盘）；未见两篇新稿 |
| 18:26:38 | barking heat：HN 查询 ×7（Navier-Stokes blow-up、Euler blowup Buckmaster、Inkling 2 Mini、Thinking Machines Inkling、METR capability update、Google school AI Search petition、ChatGPT Space）+ 条目 50581260、50577901 | barking heat | ok：`c-heat.json`、`c-hn-50581260.json`、`c-hn-50577901.json` |
| 18:26:52 | barking heat：HN 查询 ×4 + Reddit ×2 + 条目 50576232（前缀 `c2`，因 `c` 前缀已有 heat 文件被拒覆盖） | barking heat | HN ok：`c2-heat.json`（已复制为 `c-heat-2.json`）、`c2-hn-50576232.json`（已复制为 `c-hn-50576232.json`）；**Reddit 两次均被 old.reddit.com 登录页拦截（错误已记入 JSON），不绕过、未取** |
| 18:27:19 | thinkingmachines HF 仓库页、README、HF API ×2（Mini、varloop）、模型卡 ×2 | barking fetch（清单 3） | 6 ok：`c-inkling-hf2.*`、`c-inkling-hf-readme.*`、`c-inkling-hf-api.json`、`c-inkling-modelcard.*`、`c-varloop-hf-api.json`、`c-varloop-modelcard.*` |
| 18:27:36 | Vals 首页、`/benchmarks/cyberbench`、`/benchmarks/vals_index`、`/models/thinkingmachines_inkling-2-mini`、thinkingmachines.ai AUP | barking fetch（清单 4） | 4 ok：`c-vals-home.*`、`c-vals-index.*`、`c-vals-models-inkling.*`、`c-vals-aup.*`（文件名里的 vals 是命名失误，实为 AUP 页）；`vals-cyberbench` 404 failed（路径应为 `/benchmarks/cyber`） |
| 18:27:55 | Vals `/benchmarks/cyber`、`/models/anthropic_claude-opus-5-5-range` | barking fetch（清单 5） | 2 ok：`c-vals-cyber.*`、`c-vals-opus-cvp.*` |
| 18:28 | fxtwitter：ValsAI 帖 2123874523938497007 | curl | ok：`c-fx-vals.json` |
| 18:28:21 | 谷歌 K-12 博文、Drive 请愿书页、AWU about/our-people/首页、Mother Jones 抗议报道 | barking fetch（清单 6） | 6 ok：`c-google-k12-blog.*`、`c-petition-drive.*`（Drive 登录前壳页，仅 5 行，非请愿书正文）、`c-awu-about.*`、`c-awu-people.*`、`c-awu-home.*`、`c-mj-protest.*` |
| 18:28 | WebSearch ×2（谷歌学校政策让步；谷歌澄清帖） | WebSearch | 只用来发现 GovTech、Android Central、NewsFromGoogle 帖的存在 |
| 18:28 | GovTech、Android Central、Windows Report | barking fetch（清单 7） | `c-govtech.*`、`c-androidcentral.*` ok；`windowsreport` 403 Cloudflare 质询 blocked，未用 |
| 18:28 | fxtwitter：NewsFromGoogle 帖 2117714815477059722（从 Android Central 页内链接得到） | curl | ok：`c-fx-newsfromgoogle-2117714815477059722.json` |
| 18:29 | Drive 公开 PDF 直下 `drive.google.com/uc?export=download&id=1ZMCJhkZmIQiSOOM392hxMf0s7_NZOxyc` | curl（`-L`，`--max-time 60`） | ok：`c-petition.pdf`（1 页，匿名可取）、`c-petition.txt`（pdftotext） |
| 18:29:50 | OpenAI 发布说明（两个 URL 同内容）、help 搜索页 “space”、openai.com/news/ | barking fetch（清单 8） | 4 ok：`c-oa-relnotes.*`、`c-oa-relnotes-alt.*`（与前者同内容，重复）、`c-oa-space-search.*`、`c-oa-news.*` |
| 18:30 | OpenAI 帮助中心两篇 Space 文章 | barking fetch（清单 9） | 2 ok：`c-oa-space-start.*`、`c-oa-space-sharing.*` |
| 18:31:10 | barking heat：HN 查询 ×4（前缀 `c3`） | barking heat | ok：`c3-heat.json`（已复制为 `c-heat-3.json`） |
| 18:30 | HN Algolia 窗口列表（北京 11-20 18:00 起 24 小时，>120 分；>25 分另一份） | curl（`-G --data-urlencode`） | ok：`c-hn-window.json`、`c-hn-window2.json`（第一次因 URL 里 `>` 被 shell 当重定向，返回空，已重做） |
| 18:31 | Anthropic /news、谷歌 Innovation & AI、DeepSeek 更新日志、Meta AI 博客、xAI news、Mistral news、Qwen 博客 | barking fetch（清单 10） | `c-anth-news.*`、`c-google-ai.*`、`c-deepseek-news.*`、`c-mistral-news.*` ok；`meta-ai` 400 failed；`xai-news` 403 blocked；`qwen-blog` 页面正文过短（需 JS）blocked。均未重试 |
| 18:32 | Anthropic IPO 交割公告、OpenAI MedChemBench | barking fetch（清单 11） | 2 ok：`c-anth-ipo-close.*`、`c-oa-medchem.*` |
| 18:33 | OpenAI 失准报告索引与“email”一篇 | barking fetch（清单 12） | 2 ok：`c-oa-mis-index.*`、`c-oa-mis-email.*`（首发 Nov 18，不在窗口，未收） |
| 18:34 | Clay 规则页、问题列表页、新闻页、`/millennium/navier-stokes-equations/`（猜错路径） | barking fetch（清单 13） | `c-clay-rules.*`、`c-clay-rules2.*`、`c-clay-news.*` ok；`clay-ns` 404 failed |
| 18:35 | Clay NS 公告（11 Sept 2026）、Clay NS 问题页（正确路径 `…/navier-stokes-equation/`） | barking fetch（清单 14） | 2 ok：`c-clay-ns-announce.*`、`c-clay-ns-problem.*` |
| 18:36 | Vals Muse Spark Open 模型页；HF API Inkling-2-Small | barking fetch（清单 15）；curl | ok：`c-vals-musespark.*`、`c-inkling-small-hf-api.json` |

## 查证与计算

- SHA-256：`c-euler-pdf.pdf` `414f5af635c78e807d674a1bab49a440c3cf1173bc3bc7cd12f8686f80a077a3`（55 页）；`c-ns-pdf.pdf` `4a01929b2f8a595637bbbe1dfef9724e5de768d31283d62415ce045e2ed69901`（79 页）；`c-petition.pdf` `dfeb6ca327109e9255b4ce710cd623d97cdafddb0741f18efac7951d0d842992`（1 页）。
- 时间换算（UTC → 北京）：全部 +8 小时；Tao 博文发布 11-20 17:19 UTC = 11-21 01:19；Mastodon 11-20 13:37 UTC = 21:37；Vals X 帖 11-20 20:22 UTC = 11-21 04:22；OpenAI 官方帖 11-20 18:48:12 UTC = 11-21 02:48:12；NewsFromGoogle 11-03 20:25:31 UTC = 11-04 04:25:31；谷歌博文 11-03 14:00 UTC = 22:00；Mother Jones 发布 11-20 07:30 -05:00 = 20:30（北京）。Inkling 与 METR 页面的 `datePublished` 为当日午夜占位值，不换算，记“官方未给时刻”。
- 去重：`daily-scan.md` 文末“已发过”与“近 3 天速览已收”无 Euler/NS 爆破、Inkling 2 Mini、谷歌学校请愿、METR 时间跨度、ChatGPT Space 条目；Inkling 2 Small 在 1105 期事实清单里；1008 期主帖是 OpenAI NS/撤稿；1119 速览收了英国 DeepMind 工会与 Alpöge RH 声明；0929、0930、1016、1017 各期发布稿 `grep` 无 “Space”。
- 泄露自查：见回报（交付前对 `docs/2611/1120/` 本组文件 grep 绝对路径与用户名）。

## 失败与限制

- Reddit：两次搜索被 old.reddit.com 登录页拦截；未取、未绕过。
- `huggingface.co/thinking-machines/Inkling-2-Mini` 401（路径错误，已改用正确路径）；Windows Report 403 质询；xAI 403；Meta 400；Qwen 需 JS；`vals-cyberbench`、`clay-ns` 404（路径错误，已改用正确路径）。
- NYT 原文（Chris Phillips 引语）与 Google 发给学校的细则未取（付费墙与未公开）；谷歌澄清帖未明说能否在开启 Search 的同时关闭 AI Overviews / AI Mode。
- 未自行构建任一 Lean 项目；未读 OpenAI Euler 论文原文以确认其定义域；未查 Muse Spark Open 的参数量；“CVP”在 Vals 页面里无定义，未去外部找。
- AIHOT 精选无法访问，C6 只用 HN 窗口与官方新闻页，结果为无达标条目。

## 待删除 / 可清理文件（精确路径，由 Claude 收尾时一次删除）

重复或失败产物：
- `docs/2611/1120/sources/c2-heat.json`（已复制为 `c-heat-2.json`）
- `docs/2611/1120/sources/c2-hn-50576232.json`（已复制为 `c-hn-50576232.json`）
- `docs/2611/1120/sources/c3-heat.json`（已复制为 `c-heat-3.json`）
- `docs/2611/1120/sources/c-oa-relnotes-alt.headers`、`c-oa-relnotes-alt.html`、`c-oa-relnotes-alt.txt`（与 `c-oa-relnotes.*` 重复）
- 失败/拦截条目的无用产物：`c-inkling-hf.headers`、`c-inkling-hf.html`、`c-vals-cyberbench.headers`、`c-vals-cyberbench.html`、`c-clay-ns.headers`、`c-clay-ns.html`、`c-meta-ai.headers`、`c-meta-ai.html`、`c-xai-news.headers`、`c-xai-news.html`、`c-qwen-blog.headers`、`c-qwen-blog.html`、`c-windowsreport.headers`、`c-windowsreport.html`（均在 `docs/2611/1120/sources/` 下）
- `docs/2611/1120/sources/c-oa-space-search.headers`、`.html`、`.txt`（帮助中心搜索页，已用其中链接取得正文，可留可删）

不入库的大文件（HTML 原件、PDF）：本组 `.html`、`.pdf` 与 `.headers` 按 AGENTS.md 只存本地、传网盘；文本存档（txt/json/md）单个均未超过 1 MB。
