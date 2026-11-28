# 1127 期 C 组抓取与热度日志

执行：Claude Haiku 5.5（`claude-haiku-5-5`）。北京时间 11-28 23:32–23:35。未截图。未绕过验证码、付费墙或登录；未使用 firecrawl；未 commit、未 push、未调用 rm。

## 一、批量存档（`barking fetch docs/2611/1127 c`）

清单源：脚本外临时清单（scratchpad），四批运行：

| 时间（北京） | 名称 | URL | 结果 | 说明 |
|---|---|---|---|---|
| 23:32:35 | c1-google-blog | https://research.google/blog/supercharging-transformer-residual-connections/ | 200 ok | Google Research 博客，日期 November 23, 2026 |
| 23:32:35 | c1-arxiv-abs | https://arxiv.org/abs/2502.06785 | 200 ok | 提交记录 v1 2025-02-10 18:58:52 UTC；v2 2025-07-23 19:32:20 UTC |
| 23:32:35 | c2-nielsen-home | https://neuralnetworksanddeeplearning.com/ | 200 ok | 原站撤下声明，署 November 2026 |
| 23:32:35 | c3-substack | https://generativehistory.substack.com/p/gpt-62-astra-solves-a-historical | 200 ok | post_date 2026-11-24 10:01:24 UTC |
| 23:32:35 | c4-deepmind-blog | https://deepmind.google/blog/interpreting-mass-spectra-with-google-antigravity/ | 200 ok | 日期 November 25, 2026 |
| 23:32:35 | c5-osomni-arxiv | https://arxiv.org/abs/2611.34232 | 200 ok | 提交 2026-11-25 11:55:12 UTC |
| 23:32:35 | c6-qbitai | https://www.qbitai.com/2026/11/513227.html | 200 ok | 量子位，2026-11-27 15:01:20（北京） |
| 23:32:35 | c7-hfe-blog | https://blog.google/innovation-and-ai/models-and-research/google-deepmind/humanitys-first-exam/ | 200 ok | 日期 Nov 24, 2026 |
| 23:32:35 | c8-reuters | https://www.reuters.com/technology/openai-countersues-apple-stealing-trade-secrets-ft-reports-2026-11-27/ | 401 blocked | 验证码页（captcha），未绕过；存档 `c-c8-reuters.html`（待删除，见报告） |
| 23:32:35 | c1-hn-algolia | https://hn.algolia.com/api/v1/items/50672565 | 200 ok | HN 条目 JSON |
| 23:32:35 | c2-hn-algolia | https://hn.algolia.com/api/v1/items/50670757 | 200 ok | HN 条目 JSON |
| 23:32:35 | c3-hn-algolia | https://hn.algolia.com/api/v1/items/50643158 | 200 ok | HN 条目 JSON |
| 23:32:35 | c4-hn-algolia | https://hn.algolia.com/api/v1/items/50647716 | 200 ok | HN 条目 JSON（3 分，0 评） |
| 23:32:35 | c8-hn-algolia | https://hn.algolia.com/api/v1/items/50673521 | 200 ok | 误标：此为 B 组 HN 条目（租客驱逐案），误用 c8 前缀，待删除，见报告 |

批量结果：ok 13，blocked 1，failed 0（用时 5 秒）。

第二批（C6 官方页，23:33:23）：

| 名称 | URL | 结果 |
|---|---|---|
| c6-robbyant-map-v2 | https://technology.robbyant.com/lingbot-map-v2 | 200 ok |
| c6-robbyant-navi | https://technology.robbyant.com/lingbot-navi | 200 ok |
| c6-hf-map-v2-collection | https://huggingface.co/collections/robbyant/lingbot-map-v2 | 200 ok（“updated 2 days ago”，相对时间） |
| c6-hf-navi | https://huggingface.co/robbyant/lingbot-navi | 200 ok（License: apache-2.0） |
| c6-hf-navi-dataset | https://huggingface.co/datasets/robbyant/LingBot-Navi | 200 ok |

第三批（C5 HTML，23:34:24）：

| 名称 | URL | 结果 |
|---|---|---|
| c5-osomni-html-v1 | https://arxiv.org/html/2611.34232v1 | 200 ok（用于核对 Macro/Micro-Avg 表注） |

第四批（C8 媒体，23:34:56）：

| 名称 | URL | 结果 | 发布时刻 |
|---|---|---|---|
| c8-cnbc | https://www.cnbc.com/2026/11/27/openai-countersues-apple-for-stealing-trade-secrets-ft-reports.html | 200 ok | 11-27 1:56 PM EST（北京 11-28 02:56）；更新 6:07 PM EST |
| c8-guardian | https://www.theguardian.com/technology/2026/nov/27/openai-countersues-apple-trade-secrets | 200 ok | 11-27 18:28 EST（北京 11-28 07:28） |
| c8-bbc | https://www.bbc.com/news/articles/cm15ekqvlezeo | 200 ok | 未取得精确时刻（页面时间未读） |

失败与受阻：`c8-reuters`（401 验证码）。其余无失败。日志 `c-fetch-log.jsonl` 记录全部条目。

## 二、HN 与热度（`barking heat docs/2611/1127 c` 与 `c-kw`）

| 时间（北京） | 命令 | 结果 |
|---|---|---|
| 23:32:55 | `heat docs/2611/1127 c --item 50672565 --item 50670757 --item 50643158 --item 50647716 --item 50673521` | 写入 `c-heat.json`；507/178、614/404、485/308、3/0、198/154（最后一条为 B 组，已在日志中） |
| 23:34:43 | `heat docs/2611/1127 c-kw --hn "OS-Omni" --hn "LingBot" --hn "Humanity's First Exam" --hn "OpenAI Apple trade secrets" --reddit "OS-Omni" --reddit "Humanity's First Exam"` | 写入 `c-kw-heat.json`；HN 关键词 OS-Omni 0 条、LingBot 0 条、HFE 2 条、OpenAI Apple 4 条；Reddit 2 次搜索被重定向到登录页，失败 |

注：`heat` 不允许覆盖同组已有 `c-heat.json`，因此关键词热度另起前缀 `c-kw`。

HN 条目（提交时刻为北京时间，热度为 23:33–23:35 读数）：

| 条目 | 标题 | 分数 | 嵌套评论 | 提交（北京） |
|---|---|---|---|---|
| 50672565 | Google's DeepCrossAttention achieves comparable model quality up to 3x faster | 507 | 178 | 11-28 07:06:29 |
| 50670757 | Michael Nielsen took down his book on neural nets | 614 | 404 | 11-28 03:31:01 |
| 50643158 | GPT 6.2 Astra Solves a Historical Mystery | 485 | 308 | 11-25 23:16:53 |
| 50647716 | Identifying Molecules with Mass Spec Skills for Antigravity | 3 | 0 | 11-26 05:21:00 |
| 50633316 | Humanity's First Exam: Testing LLM Reliability | 3 | 0 | 11-25 03:34（关键词命中） |
| 50640241 | Humanity's First Exam (HFE)（Kaggle） | 1 | 1 | 11-25 17:18（关键词命中） |
| 50672949 | OpenAI countersues Apple for stealing trade secrets（FT） | 8 | 0 | 11-28 08:06（关键词命中） |
| 50671976 | OpenAI countersues Apple for stealing trade secrets（CNBC） | 4 | 0 | 11-28 05:44（关键词命中） |
| 50678061 | OpenAI countersues Apple for allegedly stealing trade secrets（卫报） | 4 | 0 | 11-28 22:27（关键词命中） |
| 50677365 | OpenAI counter-sues Apple over alleged theft of trade secrets（BBC） | 4 | 0 | 11-28 21:24（关键词命中） |

## 三、未截图

C 组不需要截图（派工单）。未生成 `images/` 下任何文件。

## 四、一手材料的 URL 去参数

所有 URL 已去除 utm 等查询参数（抓取时即为干净 URL）。

## 五、隐私

存档中未收集用户个人信息或抓取者信息。X/Twitter 未访问。B 组 HN 文件仅作误存记录，不含个人信息以外内容。
