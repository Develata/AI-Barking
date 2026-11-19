# 1118 期 C 组抓取记录 capture-log-c

时区：北京时间（UTC+8）。执行：claude-haiku-5-5。C 组不截图，无 `barking shot` 动作。工具自动记录的时刻以 `c-fetch-log.jsonl` 与热度 JSON 为准；临时命令（curl、本地解析）未逐条记时刻，只标注所在时段（取证窗口 21:30–21:45）。

## 一、批量存档（`barking fetch docs/2611/1118 c <清单>`）

| 北京时间（开始） | 清单 | 条目（name → 结果） | 备注 |
|---|---|---|---|
| 21:30:13 | `c-fetch-list.tsv` | pricing-en、pricing-zh、news-index、ms-decision-ga、anthropic-ipo、genie-4、daisyrose-27b、kokoro-medium、kokoro-bleeping、openai-jagged、newcore-grok → 全部 200 ok（11 条，0 blocked，0 failed） | `news-index` 指向的 URL 是 `api-docs.deepseek.com/news/`，实际抓到的是 quick_start 页 “Your First API Call”，并非新闻页。本组不采用，文件列入待删除候选（见回报）。 |
| 21:30:30 | `c-fetch-list-deepseek.tsv` | news-261113、updates、guide-search、guide-search-zh、guide-web-tools → 全部 200 ok（5 条） | `updates` 为英文 changelog，`news-261113` 为 11/13 新闻页。 |
| 21:32:21 | `c-fetch-list-2.tsv` | stitch-loop、daisyrose-hf → 200 ok（2 条） | HF 卡为 `huggingface.co/RadixArk/DaisyRose-27B`。 |
| 21:32:53 | `c-fetch-list-3.tsv` | news-261017-en、news-261017-zh → 200 ok（2 条） | 10/17 新闻页，含 “Billing starts Oct 31, 2026, at 16:00 UTC” 与中文 “11 月 1 日 00:00” 公告。 |
| 21:33:43 | `c-fetch-list-4.tsv` | ars-grok-bot → 200 ok（1 条） | Ars Technica 报道，C8 备用。 |
| 21:34:53 | `c-fetch-list-leads.tsv` | lead-deepmind-union、lead-llm-reverse、lead-ruby2shell → 200 ok（3 条） | 仅作 HN 线索的一手页面，未进正文；ruby2shell 存档文本仅 3.6 KB，正文未取到。 |

总计：22 条 URL，0 blocked，0 failed。日志位于 `docs/2611/1118/sources/c-fetch-log.jsonl`。

## 二、热度

| 北京时间 | 命令 | 结果 |
|---|---|---|
| 21:32:05 | `barking heat docs/2611/1118 c --item 50553843 --item 50550415 --item 50551046 --item 50548832 --item 50554374 --hn "Anthropic prices initial public offering" --hn "DeepSeek search billing" --hn "Kokoro Hugging Face account" --hn "NewCore Grok Bot" --hn "jagged frontier OpenAI research acceleration" --hn "DaisyRose" --hn "Genie 4" --hn "Stitch Loop" --hn "Microsoft Decision 1.1"` | 成功，写入 `c-heat.json`，0 个请求失败；写入 `c-hn-50553843.json`、`c-hn-50550415.json`、`c-hn-50551046.json`、`c-hn-50548832.json`、`c-hn-50554374.json`。**误取**：`50554374`（Gemini 4 Neon）是 B 组条目，不属于 C 组，`c-hn-50554374.json` 列入待删除候选；`c-heat.json` 的 items 列表里也含该 id。 |
| 21:32–21:43（时刻未逐条记录） | 同一命令，加入更多 `--hn` 与 `--reddit` 查询，并追加 `--item 50535763 --item 50555832 --item 50556229 --item 50537087` | **拒绝执行**：报错“docs/2611/1118/sources/ 下已有 c-heat.json、c-hn-…json，拒绝覆盖”。未删除、未覆盖任何文件。 |
| 21:43:19 | `barking heat docs/2611/1118 c-reddit --reddit "Kokoro Hugging Face" --reddit "Grok Bot" --reddit "Genie 4" --reddit "DaisyRose" --reddit "Anthropic IPO" --reddit "Stitch Loop" --reddit "DeepSeek search"`（前缀改为 `c-reddit`，避开覆盖拒绝） | **7 个请求全部失败**：Reddit 搜索 `old.reddit.com/search.json` 被重定向到登录页（`reason=lor2`，返回 HTML 而非 JSON）。失败原因写入 `c-reddit-heat.json` 的 errors。Reddit 热度本组无数据。 |

## 三、补充的 HN 查询（curl，未经 `barking heat`）

| 时段 | 动作 | URL（去查询参数） | 结果 | 备注 |
|---|---|---|---|---|
| 21:33–21:40 之间（未逐条记录） | Algolia `search_by_date`，7 天内 story，查询词：DeepSeek、Grok Bot、NewCore、Decision 1.1、Stitch Loop、Jagged Frontier | https://hn.algolia.com/api/v1/search_by_date | 打印输出，未落盘。DeepSeek 相关 8 条多为无关内容；NewCore 无命中；Grok Bot 命中 Ars 报道 HN 50532423。 | 第一次打印时 `fromtimestamp(ts+8*3600)` 与本机时区重复加了 8 小时。已用 `barking heat` 的时刻校正，本表时刻均为校正后。 |
| 21:33–21:40 之间（未逐条记录） | Algolia `items/50532423` | https://hn.algolia.com/api/v1/items/50532423 | 写入 `c-hn-50532423.json`；标题 “Grok Bot took unapproved actions, ignored orders to stop, and then denied it”，9 分，2 条顶层评论（HN 时刻 11-17 23:09）。 | C8 备用的 HN 热度与质疑来源。 |
| 21:40–21:44 之间（未逐条记录） | Algolia `items/50557885`、`50553225`、`50556287` | 同上 | 仅打印（标题、分数、URL、时刻），用于线索清单。 | 线索未核。 |

## 四、元数据与本地解析（不落盘）

| 时段 | 动作 | 结果 |
|---|---|---|
| 21:36–21:44 之间 | 检查各 `c-*.html` 的 `datePublished` / `article:published_time` / `dateModified` / `published` 类字段 | 得到：Microsoft GA `datePublished` 2026-11-17 18:10Z；Medium `datePublished` 2026-11-16 12:02Z，另一元数据 2026-11-17 17:24Z；BleepingComputer 2026-11-17 13:21 -05:00；Ars 2026-11-17 14:00Z；Genie 4 2026-11-18 18:00Z；DaisyRose 2026-11-18 12:00Z；Anthropic 2026-11-18 00:00Z（占位，按“官方未给时刻”记）；Stitch Loop 2026-11-18 17:00Z；OpenAI 2026-11-18 00:00 -06:00（占位，同上）；NewCore、DeepSeek 页无元数据。 |
| 21:38–21:42 之间 | `python3 -I` 解码 `c-ms-decision-ga.txt` 内的 base64 嵌入内容（6 段），保留 script，检索 79x、50x、GPT-Sol、cheaper、footnote、cost 等 | Ignite 嵌入（L181，1.49 MB 解码）无“79x / 50x / GPT-Sol”文本；命中项为 base64 随机串或 CSS 类名。L149、L155、L157、L163、L211 无口径脚注。 |
| 21:38–21:42 之间 | 解码 `docs/2611/1117/sources/b-ignite.txt` 第 261 行 base64 块 | 可见文本仅 “The right model for every job”，无 79x。 |
| 21:40–21:43 之间 | `c-openai-jagged` 全文检索 estimate / duration / judge / grader 等词 | 仅见第 106、184、188 行的结论性描述；“View methods” 的具体方法未在存档中出现。 |
| 21:44 | `python3 -I` 核对 `evidence-c.md` 中全部直引（91 个，按“…”切分后逐段比对） | 未命中 12 段：6 段原文在该处带链接标记（如 `[/guides/web_tools]`）；1 段来自解码块（b-ignite 第 261 行）；4 段来自 HN 评论（存于 `c-hn-50532423.json`，不在 txt 语料中）；1 段为表格行的分隔写法；1 段是查询词，不是引文。其余逐字命中。 |

## 五、未做 / 失败

- Reddit：7 个查询全部被登录页拦截（见第二节），本组无 Reddit 热度。
- Nasdaq 实际开盘页、Artificial Analysis 页：本组未取。C3 的交易状态按“预期”记；按时钟推算取证时美东约 08:44，尚未开盘（推算，非核实）。
- xAI 原支持回复：只见 NewCore 转述，未单独取证。
- 有 blocked / failed：批量存档 0 条；Reddit 热度 7 条失败（已记录）。
- 没有使用登录态，没有绕过付费墙或验证。
- 没有 X 帖子、没有截图，因此无头像、昵称、handle 需要替换。
