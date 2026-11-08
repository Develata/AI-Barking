# 1107 来源与制作状态

期号1107（上期1106+1）；线索为 Develata 回贴的 ChatGPT 选题扫描 SLOT 1109-00（北京 11-09 00:01），另由 Claude 现查 Hacker News 补速览线索（AIHOT 在内置浏览器未打开，Develata 新闻页未试）。Develata 拍板：主帖 Apiiro 与 DoubleAI，速览由 Claude 自选。

## 主帖

1. Claude Code 配置文件外泄：[Apiiro 博文](https://apiiro.com/blog/coding-agents-leaking-secrets-github-dont-ask-again)（只能经内置浏览器读取，要点转述见 `a-apiiro-browser-notes.md`，未截图）、[官方文档 settings](https://code.claude.com/docs/en/settings)、[permissions](https://code.claude.com/docs/en/permissions)、[HN 帖及 Cherny 回复](https://news.ycombinator.com/item?id=50407882)、四个 GitHub issue（`a-gh-issues-meta.json`）、[THN](https://thehackernews.com/2026/11/claude-codes-dont-ask-again-file.html)、iTWire（`a-itwire.*`）。
   - 待核：Apiiro 数字无第三方复核；博文发布日原页未显示，报道写 11/5 或 11/6；Lakera 一手页 404，正文未用。
   - Claude 与本事件当事产品同属 Anthropic，本条靠跨模型复核把关。
2. DoubleAI：[原文](https://www.doubleai.com/research/can-llms-redesign-themselves-as-they-train)、[新智元网易稿](https://www.163.com/dy/article/LB7TFPQE0511ABV6.html)、MathArena HF 数据集（题数）。
   - 待核：DeepSeek 组 token 成本、重复次数未给；第三方复现未找到（未穷尽）。热度偏弱：HN 10 分与 2 分均 0 评，网易稿评论 3 条。

## 速览

- Anthropic IPO：[彭博社](https://www.bloomberg.com/news/articles/2026-11-06/anthropic-ipo-valuation-expectations-pressured-after-ai-rout)（L4）。
- OpenAI 融资：[TechCrunch](https://techcrunch.com/2026/11/06/openai-reportedly-raises-an-additional-11-billion-at-a-1-25-trillion-valuation/)（L4）。
- 微软 AI Frontiers：[Langford 博客](https://hunch.net/?p=13764006)（L2，个人自述）。
- 图书馆：[CBC](https://www.cbc.ca/news/world/university-libraries-ai-data-scraping-bots-9.7408287)（L4）。
- Claude Code 2.1.319：[CHANGELOG](https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md)（L1，摘录 `c-changelog-2.1.319.txt`）。
- llama-modchip：[README](https://github.com/synchopate/llama-modchip)（作者自报）。
- 未收：FT 燃气机组（FT 403，未读到）；Arena 周报（榜单 403，周报未找到）；8rank Mac 分类器（无 L1–L4）；数据中心与议员调查（涉政风控）。

## 流程记录

- 三组取证：C Haiku 约 7 分钟，A Sonnet 约 18 分钟，B Sonnet 约 18 分钟（harness 时长）。
- 超过 1 MB 的 `a-changelog.txt`、`c-claude-changelog.txt`、`c-npm-claude-code.json` 经 Develata 决定不入库，以摘录代替。
- 取证遭拦截并未绕过：Apiiro（Cloudflare，自动化访问）、FT、WSJ、Arena 榜单、SEC EDGAR、web.archive.org 快照（权限分类器拒绝）。
- 扫描勘误：Apiiro“8 月 12 日已收”不实；“中文稿删了 65.4%”不实；“没有安全公告页”不实（官方 GHSA 列表存在）；DoubleAI 2.8× 属 Qwen 组；燃气机组未读到 FT 原文。
