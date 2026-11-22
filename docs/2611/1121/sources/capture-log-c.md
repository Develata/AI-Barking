# 1121 期 C 组（速览）抓取与采样日志

时间一律北京时间（UTC+8）。全部为 headless 或直接 HTTP 抓取，无截图（C 组不需要）。各网页的 `fetch` 结果同时逐条记在 `c-fetch-log.jsonl`（本表摘要）。

## 一、`barking fetch`（清单 `c-fetch-list-1.tsv` 至 `c-fetch-list-9.tsv`）

| 北京时间 | 名称 | 请求 URL（最终 URL 相同，除注明） | HTTP | 结果 | 备注 |
|---|---|---|---|---|---|
| 11-22 22:59:59 | c-sutherland-grh | https://math.mit.edu/~drew/grh/ | 200 | ok（html + txt） | 页面 last updated 2026-11-22 14:10 UTC。早先 curl 直接请求返回 000，本次 fetch 正常 |
| 11-22 22:59:59 | c-bdcp-byop | https://betterdatacenterproject.com/2026/11/16/how-ai-is-powered-a-breakdown-of-byop-and-grid-power/ | 200 | ok | |
| 11-22 23:00:00 | c-google-duplex | https://research.google/blog/how-google-duplex-evolved-to-also-call-businesses-for-you-in-gemini/ | 200 | ok | 页面日期 November 20, 2026 |
| 11-22 23:00:00 | c-openai-medchembench | https://openai.com/index/introducing-medchembench/ | 200 | ok | 页面日期 November 20, 2026 |
| 11-22 23:00:00 | c-anthropic-ipo | https://www.anthropic.com/news/anthropic-announces-closing-of-initial-public-offering | 404 | failed | 清单里的 URL 有误。正确地址见下一行 |
| 11-22 23:00:00 | c-browser-use-1 | https://browser-use.com/posts/browser-use-1-0 | 200 | ok | 页面 Published November 20, 2026 |
| 11-22 23:00:59 | c-hunyuan-hy4 | https://hunyuan.tencent.com/ | 200 | blocked（正文过短） | 腾讯混元站点需渲染，未存 txt |
| 11-22 23:01:00 | c-anthropic-news | https://www.anthropic.com/news | 200 | ok | 从列表找到 IPO 交割页 `/news/ipo-close` |
| 11-22 23:01:00 | c-maynard-anthropic-riemann | https://www.anthropic.com/research/riemann-zeta | 200 | ok | 2026-08 Anthropic 黎曼 ζ 研究页；未引用，仅核对 Maynard 原话不在该页内 |
| 11-22 23:01:00 | c-hunyuan-github | https://github.com/Tencent-Hunyuan | 200 | ok | 组织概览首屏，只见 Hy4-preview 仓库（未翻页） |
| 11-22 23:01:01 | c-runway-blog | https://runwayml.com/research | 200 | ok | 无相关内容；未引用 |
| 11-22 23:01:11 | c-anthropic-ipo-close | https://www.anthropic.com/news/ipo-close | 200 | ok | Published Nov 20, 2026 |
| 11-22 23:01:18 | c-hy4-preview-repo | https://github.com/Tencent-Hunyuan/Hy4-preview | 200 | ok | 预览版仓库，未引用为 Hy4 正式版 |
| 11-22 23:01:42 | c-maynard-livescience | https://www.livescience.com/technology/artificial-intelligence/this-is-what-happened-with-claude-and-me-ais-failed-attempt-to-crack-the-riemann-hypothesis-led-mathematician-to-a-breakthrough | 200 | ok | html 约 1.9 MB（本地），txt 约 45 KB。Published 1 October 2026 |
| 11-22 23:01:43 | c-maynard-sciam | https://www.scientificamerican.com/article/no-ai-didnt-just-solve-the-thorniest-problem-in-math/ | 200 | ok | 页面 August 12, 2026 |
| 11-22 23:02:01 | c-hy4-hf-card | https://huggingface.co/tencent/Hy4/raw/main/README.md | 200 | ok | HF 模型卡原始 Markdown |
| 11-22 23:02:12 | c-hy4-license | https://huggingface.co/tencent/Hy4/raw/main/LICENSE | 200 | ok | Tencent Hy License 1.0 |
| 11-22 23:02:12 | c-gemini-tasks | https://blog.google/innovation-and-ai/products/gemini-app/tasks-in-gemini-free-new-features/ | 200 | ok | 页面 Nov 17, 2026 |
| 11-22 23:03:19 | c-hy4-official | https://hy.tencent.ai/research/hy4 | 200 | blocked（正文过短） | 官方页需渲染；HN 50365780 指向此页。未存 txt |
| 11-22 23:04:18 | c-minimax-news | https://www.minimax.io/news | 200 | ok | 页面中未见 MIM |
| 11-22 23:04:18 | c-runway-software-factory | https://runway.com/news/engineering/building-a-software-factory | 200 | ok | 页面 November 20, 2026；C4 主来源 |
| 11-22 23:04:19 | c-minimax-github | https://github.com/MiniMax-AI | 200 | ok | 组织页中未见 MIM |

本组另有 `barking fetch` 以外的一次性 HTTP 请求：

| 北京时间（约） | 工具 | 请求 | 结果 | 存档 |
|---|---|---|---|---|
| 23:00 | curl `--max-time 60` | https://api.fxtwitter.com/status/2124222447855743019 | 第一次 exit 35（TLS 失败），第二次返回 200 | `c-fx-tesla-2124222447855743019.json` |
| 23:04 | curl `--max-time 60` | https://api.fxtwitter.com/status/2123875791566168423 | 200 | `c-fx-runway-2123875791566168423.json` |
| 23:02 | curl `--max-time 60` | https://huggingface.co/api/models?author=tencent&limit=100&sort=lastModified&direction=-1 | 200 | `c-hf-api-tencent-models.json`（HF 列表，含 `tencent/Hy4`、`Hy4-base`、`Hy4-FP8`、`Hy4-preview` 的 `private`、`createdAt`、`lastModified`、license tag） |

X 帖子只用 fxtwitter 公开 JSON 取文本与时间，未访问 X 页面，JSON 内无抓取者信息。

## 二、热度采样

| 北京时间 | 工具 | 参数 | 结果 |
|---|---|---|---|
| 23:02:24 | `barking heat docs/2611/1121 c` | `--hn "FSD v15"`、`"Anthropic closing of initial public offering"`、`"MedChemBench"`、`"browser-use 1.0"`、`"Google Duplex Tasks in Gemini"`、`"Better Data Center Project Anthropic"`、`"Tencent Hy4 open weights"`、`"Riemann Anthropic Maynard"` | 写入 `c-heat.json`：FSD 命中 1 条（50594937，28 分 / 8 评，11-22 08:14）；IPO 命中 1 条（50574068，8 分 / 0 评，11-20 20:57）；其余 6 个查询 0 命中。 |
| 23:04 | `barking heat docs/2611/1121 c --hn "MedChemBench" ...`（第二轮） | 另 10 个查询 | 被拒绝：`docs/2611/1121/sources/ 下已有 c-heat.json，拒绝覆盖`。未写入，未重跑 |
| 23:06 | Algolia 公共 API `https://hn.algolia.com/api/v1/search`（curl，tags=story，hitsPerPage=3） | 查询 "Better Data Center Project"、"MedChemBench"、"browser-use 1.0"、"Tasks in Gemini Duplex"、"Tencent Hy4"、"Anthropic Claude Riemann Alpöge" | 替代 `barking heat` 第二轮。结果：BDCP 50597612（330 分 / 440 评，11-22 16:57）；Hy4 50365780（294 / 142，11-04 22:11，即 11-04 首发）；Hy4 Flash 50538184（175 / 27，11-18 05:06）；Riemann Alpöge 两帖（50581076 4 分，50580069 2 分，11-21）；MedChemBench、browser-use 1.0、Duplex 无相关 story |
| — | Reddit | — | 未查 |

## 三、网络搜索（WebSearch，标准模式，除注明）

搜索 14 次。失败 5 次是工具名写错（误用 `mcp__Claude_Code__WebSearch`），以下只列有效调用：

1. 腾讯混元 Hy4 开源 770B 49B 1M 上下文 System Card 模型权重 Hugging Face GitHub：无官方链接，只有媒体转载（新华网、凤凰网等，L4/L5）。
2. Maynard Alpöge Claude Riemann hypothesis proof interview：找到 SciAm 与 Live Science 的 Maynard 原话（2026-08 事件）；未找到 Alpöge 2026-11-20 声明的 Maynard 评论。
3. Runway agents approved 38% of merged pull requests October review median wait rollback rate：无相关结果。
4. MiniMax MIM cryptanalysis NIST candidate first practical attack technical report PoC：无相关结果。
5. Anthropic closing of initial public offering 1,127,000,000 shares over-allotment：未直接找到交割页；交割页由 anthropic.com/news 列表找到（`/news/ipo-close`）。
6. Maynard 黎曼猜想 Alpöge Claude 证明 采访 正确 理解 认可：无 Maynard 对 Alpöge 证明的原话。
7. MiniMax MIM 密码分析 NIST 候选 首个实用攻击 技术报告 开源：无相关结果。
8. Runway 智能体 38% 合并 PR 审批 中位数 回滚率 千分之一：无相关结果。
9. Runway agents 38% of merged pull requests approved by agents October 2026：结果含 Runway 的 X 帖（2123875791566168423），由此找到工程文章。
10. IT之家 Maynard Alpöge Claude 黎曼猜想 证明 相信正确 需要被理解：未找到 IT 之家原稿；返回的是其他转载与 Anthropic 2026-08 研究页。
11. Runway agent-authored pull requests approval share 38% blog post 2026（extended）：无工程文章，只有 X 帖。
12. MiniMax 研究模型 密码分析 MIM 攻击 NIST 后量子 签名 2026年11月（extended）：找到 10jqka、FX168、17173、新浪微博（二手快讯，L4–L6）。
13. MiniMax MIM internal model cryptanalysis NIST candidates Waterfall-256 LOCKER SBC-VOLE technical report open source：无官方材料。
14. MiniMax official blog MIM model cryptanalysis announcement November 2026：无 MiniMax 官方博客结果。

Anthropic IPO 的 WebSearch（第 5 条）返回的 "980 million base shares plus 147 million option shares" 等数字是搜索摘要的推算，未用于正文；本组以 `anthropic.com/news/ipo-close` 原文为准。

## 四、本组未完成与失败项

- C3：Maynard 对 Alpöge 声明的评论原稿（IT 之家）未找到。
- C4：工程文章已找到（`runway.com/news/engineering/building-a-software-factory`）；X 帖与文章均为 Runway 自报。
- C9：MiniMax 官方博客、技术报告、PoC 仓库均未找到；MiniMax 新闻页与 GitHub 组织页中未见 MIM。
- C1：`hunyuan.tencent.com` 与 `hy.tencent.ai/research/hy4` 为 JS 渲染（blocked）；System Card PDF 未存档（仅在模型卡中见链接）。Hy4 GitHub 正式版仓库未找到（只看组织概览首屏，未翻页）。
- C7：清单中的 URL 404，已用 `/news/ipo-close` 替代。
- Reddit 热度未查。
- 未使用 opencli（doctor 显示 Browser Bridge 未连接，按派工单直接走抓取）。

## 五、时间与设备

- 时间戳取自 `date` 与 fetch 日志，北京时间。
- 本组无截图，无 `barking shot` 调用。
- 未登录、未访问需登录页面、未使用 Cookie。

## 六、待删除文件（待 Claude 收尾时一次删除）

以下为失败页面的抓取副产品，内容只是 404 或 JS 壳，不作为证据使用（记录已在本日志与 `c-fetch-log.jsonl` 中）：

- `docs/2611/1121/sources/c-c-anthropic-ipo.html`
- `docs/2611/1121/sources/c-c-anthropic-ipo.headers`
- `docs/2611/1121/sources/c-c-hunyuan-hy4.html`
- `docs/2611/1121/sources/c-c-hunyuan-hy4.headers`
- `docs/2611/1121/sources/c-c-hy4-official.html`
- `docs/2611/1121/sources/c-c-hy4-official.headers`

其余 `c-*` 文件均为本组证据，保留。
