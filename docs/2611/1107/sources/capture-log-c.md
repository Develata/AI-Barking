# 1107 期 C 组抓取与检索日志

时刻为北京时间（UTC+8）。逐条抓取的机器记录在 `sources/c-fetch-log.jsonl`，热度在 `sources/c-heat.json` 与 `sources/c-hn-<id>.json`。取证未截图（C 组不需要），`sources/c-shots-log.jsonl` 未生成。

| 时刻（北京） | 工具 | 对象 | URL | 结果 | 说明 |
|---|---|---|---|---|---|
| 11-09 00:49:38 起（批 1） | `barking fetch` | hunch | https://hunch.net/?p=13764006 | ok（200，296 行文本） | C1 博客全文 |
| 同上 | `barking fetch` | bloomberg-ipo | https://www.bloomberg.com/news/articles/2026-11-06/anthropic-ipo-valuation-expectations-pressured-after-ai-rout | ok（200，317 行） | C2 Bloomberg，正文已核 |
| 同上 | `barking fetch` | wsj-ipo | https://www.wsj.com/finance/major-investors-are-skeptical-anthropic-ipo-can-achieve-1-5-trillion-valuation-ca965d8b | **blocked**（401 Unauthorized，770 字节，无 .txt） | C2' 失败，不绕过 |
| 同上 | `barking fetch` | techcrunch-openai | https://techcrunch.com/2026/11/06/openai-reportedly-raises-an-additional-11-billion-at-a-1-25-trillion-valuation/ | ok（200，318 行） | C3 TechCrunch，正文已核 |
| 同上 | `barking fetch` | 247wallst-openai | https://247wallst.com/investing/2026/11/07/openai-just-raised-11-billion-at-a-1-25-trillion-valuation-below-the-1-4-trillion-it-first-sought/ | ok（200，332 行） | C3' 247wallst，正文已核（转述 CFO 访谈） |
| 同上 | `barking fetch` | ft-anthropic-gas | https://www.ft.com/content/41bf5c6a-9735-4f1a-a8e3-2bcccad58ba0 | **blocked**（403，Cloudflare 质询 cf-mitigated: challenge） | C4 失败，不绕过 |
| 同上 | `barking fetch` | cbc-libraries | https://www.cbc.ca/news/world/university-libraries-ai-data-scraping-bots-9.7408287 | ok（200，315 行） | C5 CBC，正文已核 |
| 同上 | `barking fetch` | 8rank | https://8rank.ai/game/apple-intelligence-personal-knowledge-graph | ok（200，5949 行） | C9 作者研究页存档 |
| 同上 | `barking fetch` | llama-modchip-readme | https://raw.githubusercontent.com/synchopate/llama-modchip/main/README.md | ok（200，246 行） | C8 README |
| 同上 | `barking fetch` | claude-changelog | https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md | ok（200，10126 行，1,176,779 字节） | C6 CHANGELOG，**超过 1 MB**，需决定入库与否 |
| 同上 | `barking fetch` | npm-claude-code | https://registry.npmjs.org/@anthropic-ai/claude-code | ok（200，1,588,865 字节，JSON） | C6 发布时刻，**超过 1 MB**，需决定入库与否 |
| 同上 | `barking fetch` | ghsa-claude-code | https://api.github.com/repos/anthropics/claude-code/security-advisories?per_page=30 | ok（200，114 KB） | C6 GHSA 列表 |
| 11-09 00:51:13 起（批 2） | `barking fetch` | arena-agent-pareto | https://arena.ai/leaderboard/agent/pareto | **blocked**（403，Cloudflare 质询） | C7 失败，不绕过 |
| 同上 | `barking fetch` | arena-agent-overall-pareto | https://arena.ai/leaderboard/agent/overall/pareto | **blocked**（403，Cloudflare 质询） | C7 失败，不绕过 |
| 同上 | `barking fetch` | arena-blog | https://arena.ai/blog | ok（200，354 行） | 索引最新至 2026-10-27，无 11-07 周报 |
| 同上 | `barking fetch` | arena-methodology | https://arena.ai/blog/agent-arena-methodology/ | ok（200，453 行） | 方法页：causal tracing，95% CI |
| 同上 | `barking fetch` | arena-cost-blog | https://arena.ai/blog/agent-categories-and-cost/ | ok（200，453 行） | 成本页（2026-08-14 发布、08-19 更新）；非 11-07 数字来源 |
| 00:51:50 | `barking heat` | 12 个 HN 条目 | 50406563、50406589、50394906、50386483、50386597、50402738、50413822、50412760、50392499、50386516、50411961、50403097 | 0 个失败 | 数字见 `evidence-c.md` 热度节 |
| 约 00:4x（批次未逐条记时） | WebSearch（检索，仅找 URL 与线索，不作证据） | C1–C5、C7、C9 | 检索词见下 | 多为二手转述 | 线索转为上方 URL 再由 barking fetch 核实 |
| 约 00:4x | curl → hn.algolia.com API | HN 条目 | https://hn.algolia.com/api/v1/search 与 /items/<id> | 得到 HN 标题、URL、分数、时刻 | 仅用于找原始 URL 与热度，不作正文证据 |
| 约 00:4x | curl → sec.gov EDGAR | Anthropic S-1 列表 | https://www.sec.gov/cgi-bin/browse-edgar?company=anthropic&type=S-1&action=getcompany&count=20 | **403**（“Undeclared Automated Tool”） | 未核 11-02 之后的修订；未伪造 UA，未重试 |

## WebSearch 检索词（仅线索）

- Anthropic IPO valuation buyers $1.1 trillion Bloomberg
- OpenAI raises $11 billion valuation $1.25 trillion TechCrunch November 2026
- Anthropic $3.4 billion 1.7 gigawatt gas turbine generators Financial Times
- CBC university library AI crawlers overwhelm website
- Microsoft Research AI Frontiers team disbanded John Langford hunch.net
- Arena Agent Arena open model leaderboard Kimi K3.1 Pro Hy4 median cost per task
- Apple Intelligence hidden family member classifier Mac knowledge graph 8rank
- Anthropic gas-fired power data centres $3.4bn deal Financial Times November 2026
- Microsoft statement AI Frontiers Langford reorganization Microsoft AI November 2026

检索结果中的 Nscale（460MW、450 亿美元）、Hut 8（70 亿美元）、Marmot、Anthropic 2 亿美元类数字都是其他交易或转述，未用于速览。

## 文件清单（C 组新增，均在 `docs/2611/1107/sources/`）

- `c-fetch-list-1.tsv`、`c-fetch-list-2.tsv`（抓取清单）
- `c-fetch-log.jsonl`（barking fetch 记录）
- 正文存档（html 原件只存本地，txt 与 headers 可入库）：`c-hunch.{html,txt,headers}`、`c-bloomberg-ipo.{html,txt,headers}`、`c-techcrunch-openai.{html,txt,headers}`、`c-247wallst-openai.{html,txt,headers}`、`c-cbc-libraries.{html,txt,headers}`、`c-8rank.{html,txt,headers}`、`c-llama-modchip-readme.{txt,headers}`（无 html）、`c-claude-changelog.{txt,headers}`（无 html）、`c-npm-claude-code.{json,headers}`、`c-ghsa-claude-code.{json,headers}`、`c-arena-blog.{html,txt,headers}`、`c-arena-methodology.{html,txt,headers}`、`c-arena-cost-blog.{html,txt,headers}`
- 失败页存档（只有 html 与 headers，无 .txt，不作证据）：`c-wsj-ipo.{html,headers}`、`c-ft-anthropic-gas.{html,headers}`、`c-arena-agent-pareto.{html,headers}`、`c-arena-agent-overall-pareto.{html,headers}`
- `c-heat.json`、`c-hn-<id>.json`（12 个）
- `evidence-c.md`、`capture-log-c.md`（本文件）

## 待删除文件

无。

## 说明

- 超过 1 MB 的文本存档须先由 Claude 决定是否入库：`c-claude-changelog.txt`（1,176,779 字节）与 `c-npm-claude-code.json`（1,588,865 字节）。
- HTML 原件只存本地，不入库（按规定）。
- 未截图，故 `c-shots-log.jsonl` 不存在。
