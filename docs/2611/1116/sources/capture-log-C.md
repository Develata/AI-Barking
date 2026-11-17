# 1116 期 C 组（速览）抓取日志

时间一律为北京时间（UTC+8，本机时区）。工具：`barking fetch`（清单经 scratchpad 的 `c-list-*.tsv`，未入库）、`barking heat`、WebSearch（搜索摘要，未存档）、WebFetch（单次）。机器可读记录见 `docs/2611/1116/sources/c-fetch-log.jsonl` 与 `c-heat.json`。

## 一、barking fetch（网页与 PDF 存档）

| 时刻（北京） | 名称 | URL | 结果 | 备注 |
|---|---|---|---|---|
| 19:24:39–19:24:43 | amd-newsroom-complete | `https://newsroom.amd.com/news/amd-completes-acquisition-world-labs/` | ok（200，100942 B，245 行 txt） | C1 完成稿 |
| 19:24:39 | amd-newsroom-sign | `https://newsroom.amd.com/news/amd-acquire-world-labs.html` | ok（200，104489 B，241 行） | C1 签约稿 |
| 19:24:39 | hf-openrouter | `https://huggingface.co/blog/openrouter` | ok（200，154649 B，324 行） | C2 |
| 19:24:39 | caag-press | `https://oag.ca.gov/news/press-releases/it%E2%80%99s-golden-state-not-wild-west-attorney-general-bonta-sues-openai-over` | ok（200，53547 B，402 行） | C3 新闻稿 |
| 19:24:39 | databricks-aiindex | `https://www.databricks.com/blog/introducing-databricks-ai-index-state-enterprise-ai-tokenomics` | ok（200，758064 B，857 行） | C4 |
| 19:24:43 | hn-50526537（JSON） | `https://hacker-news.firebaseio.com/v0/item/50526537.json` | ok（395 B） | C7 热度与 HN 元数据 |
| 19:24:43 | hn-50520857（JSON） | `https://hacker-news.firebaseio.com/v0/item/50520857.json` | ok（237 B） | 非 C 组（B 组 HN 条目），误抓，见待删除清单 |
| 19:24:43 | hn-50524736（JSON） | `https://hacker-news.firebaseio.com/v0/item/50524736.json` | ok（881 B） | 非 C 组（A 组 HN 条目），误抓，见待删除清单 |
| 19:25:04 | caag-complaint-pdf | `https://oag.ca.gov/system/files/attachments/press-docs/complaint-oai-cyber.pdf` | ok（200，495536 B，PDF，1710 行 txt） | C3 诉状；PDF 文本层可用，未 OCR |
| 19:25:15 | mem-shutdown | `https://get.mem.ai/blog/mem-is-shutting-down` | ok（200，41597 B，219 行） | C7 官方公告 |
| 19:26:20 | sinoxiv-one2n | `https://sinoxiv.napstic.cn/article/24569721` | ok（200，19171 B，54 行） | 搜索线索，内容与 C8 无关（2025 年 FourCastNet 文献列表），见待删除清单 |

Fetch 汇总：本组 9 条请求 ok，0 blocked，0 failed（以上不含 WebFetch 与搜索）。PDF 与 HTML 原件只存本地（不入库）；`.txt` 文本存档可入库。

## 二、WebFetch（单次）

| 时刻（北京，约） | URL | 用途 | 结果 |
|---|---|---|---|
| 19:27 | `https://blogs.nvidia.com/blog/vera-rubin-nvl72-mlperf-inference` | C5 相关背景，核对是否为所谓 SC26 博客 | 页面日期 2026-09-16；为 MLPerf Inference v6.1 预览，含 “upcoming MLPerf Endpoints benchmark”，不含 “per watt”“every benchmark”“TTFT”“SC26” 等字样。不是 C5 的原博客。 |

## 三、WebSearch（搜索摘要，未存档原文）

| 时刻（北京，约） | 查询 | 目的 | 结果 |
|---|---|---|---|
| 19:26–19:29 | NVIDIA Vera Rubin MLPerf Endpoints first submission 10x performance per watt SC26 | C5 原博客 | 未找到 SC26 博客；命中 2026-09 的 MLPerf Inference v6.1 预览文章 |
| 19:28 | NVIDIA blog November 2026 Vera Rubin MLPerf Endpoints results 10x tokens per watt MLCommons pending verification | C5 复查 | 未找到 11 月博客或 Endpoints 成绩 |
| 19:28 | MLPerf Endpoints results MLCommons Vera Rubin 1024 concurrency GB300 48 TTFT P90 | C5 成绩条目 | 未找到含 TTFT P90 / 并发数的成绩 |
| 19:27 | TensorWave OpenAI 457MW AMD compute announcement | C6 | 未找到 457 MW 与 OpenAI 的新闻稿；命中 2026-06-10 融资、2026-07-24 Helios 采用、Fermi 650MW 租约（未存档） |
| 19:28 | TensorWave OpenAI 457 MW AMD Instinct data center 2027 press release | C6 复查 | 同上，未找到 |
| 19:27 | 中国科学院青藏高原研究所 万拓 One2N 全球天气模型 0.07度 | C8 | 未找到成果页；命中 ECMWF AI Weather Quest 的 ONE2N 团队页（负责人田向军，未存档） |
| 19:28 | One2N 全球天气预报 0.07° 青藏高原研究所 单卡 四周 arXiv 2026 | C8 arXiv | 未找到 2026 arXiv 预印本；命中 OneForecast 2502.00338（不同项目，未关联） |
| 19:29 | 中国科学院青藏高原研究所 成果 2026年11月 全球天气模型 0.07 度 单张 GPU 四周 预报 技术 | C8 复查 | 未找到；命中无关的中科院大气所论文与曙光 2025 年成果 |

## 四、barking heat（热度）

| 时刻（北京） | 命令要点 | 结果 |
|---|---|---|
| 19:26:27 | `heat docs/2611/1116 c --item 50526537 --item 50520857 --item 50524736 …` | 被拒：`docs/2611/1116/sources/` 下已有 `c-hn-*.json`（由 fetch 生成），拒绝覆盖。未改动这些文件。 |
| 19:26:27 | `heat docs/2611/1116 c --hn "AMD World Labs" --hn "Hugging Face OpenRouter" --hn "Attorney General OpenAI lawsuit" --hn "Databricks AI Index" --hn "Vera Rubin MLPerf" --hn "TensorWave" --hn "One2N weather" --reddit "Mem shutting down"` | 写入 `c-heat.json`。HN：C2 命中 50520212（4 分/0 评，11-17 01:29）；C4 命中 50520896（2 分/1 评，11-17 02:19）；其余 HN 查询 0 条相关。Reddit：1 个请求失败（返回 old.reddit.com 登录页重定向，非 JSON），记入 errors，未重试。 |

HN 官方 JSON（C7 热度，取自 `c-hn-50526537.json`）：50526537 “Mem Is Shutting Down”，64 分 / 102 评，提交 2026-11-17 10:13（北京时间）。

## 五、失败与未完成

- C5：一手原文（SC26 NVIDIA 博客、mlcommons.org 成绩条目）未找到，未存档。
- C6：TensorWave 一手新闻稿未找到，未存档。
- C8：成果页、arXiv 预印本未找到，未存档。
- C3：OpenAI 回应与 AP、Reuters 报道未检索（派工单要求的媒体回应未完成）。
- Reddit：仅 C7 一次查询，失败；其余条目未查 Reddit。
- 未使用付费墙、镜像或代理域名；未登录；未用 firecrawl。

## 六、待删除文件（由 Claude 收尾时一次删除，本组未调用 rm）

误抓或与 C 组无关的文件（均在 `docs/2611/1116/sources/` 下）：

- `docs/2611/1116/sources/c-hn-50520857.json`、`docs/2611/1116/sources/c-hn-50520857.headers`（B 组 HN 条目，误抓）
- `docs/2611/1116/sources/c-hn-50524736.json`、`docs/2611/1116/sources/c-hn-50524736.headers`（A 组 HN 条目，误抓）
- `docs/2611/1116/sources/c-sinoxiv-one2n.html`、`docs/2611/1116/sources/c-sinoxiv-one2n.headers`、`docs/2611/1116/sources/c-sinoxiv-one2n.txt`（搜索线索，内容与 C8 无关）

注：`c-fetch-log.jsonl` 中上述三组文件的抓取记录保留（追加式日志，不改）；删文件后请一并注明。

## 七、本组新增文件（`docs/2611/1116/sources/`）

- 文本存档（可入库）：`c-amd-newsroom-complete.txt`、`c-amd-newsroom-sign.txt`、`c-hf-openrouter.txt`、`c-caag-press.txt`、`c-databricks-aiindex.txt`、`c-caag-complaint-pdf.txt`、`c-mem-shutdown.txt`、`c-hn-50526537.json`、`c-heat.json`
- 原件（只存本地，不入库）：`c-amd-newsroom-complete.html`、`c-amd-newsroom-sign.html`、`c-hf-openrouter.html`、`c-caag-press.html`、`c-databricks-aiindex.html`、`c-caag-complaint-pdf.pdf`、`c-mem-shutdown.html`，以及各自的 `.headers`
- 日志与成品：`c-fetch-log.jsonl`（追加）、`evidence-C.md`、`capture-log-C.md`

大小（`wc -c` 实测）：所有 `.txt` 文本存档均远小于 1 MB，最大的 `c-caag-complaint-pdf.txt` 为 80863 字节，其次 `c-databricks-aiindex.txt` 为 29411 字节。原件 `c-databricks-aiindex.html` 约 758 KB、`c-caag-complaint-pdf.pdf` 约 496 KB，只存本地。

## 八、假设

- 搜索摘要中出现的 URL（TensorWave 2026-06 融资、Helios、Fermi 租约、ECMWF ONE2N 团队页等）只作线索，未存档、未据以定稿。
