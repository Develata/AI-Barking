# 1115 期 C 组抓取日志

执行：claude-haiku-5-5。时间为北京时间（本机时区 UTC+8）。文件前缀 `c-`。未登录任何账户，未提交表单，未发帖，未绕过付费墙或质询。`opencli doctor` 本组未用（全部静态存档成功）。

## 抓取（`barking fetch`）

| 时刻（北京） | 清单 | URL | 结果 |
|---|---|---|---|
| 23:58:52 | `c-list1.tsv` | Apple 新闻稿 `c1-apple`；ECB 演讲 立陶宛语 `c2-ecb-lt`、英文 `c2-ecb-en`；Fed Barr 演讲 `c3-barr`；HN Algolia 条目 `c6-hn-algolia` | 全部 200 ok；`c-c6-hn-algolia.json` 为 JSON，无 `.txt` |
| 23:59:20 | `c-list2.tsv` | Xena 博客原文 `c6-xena`；`c6-xena-ko`（`?lang=zh` 参数版） | 两条 200 ok（`c6-xena-ko` 未用） |
| 00:00:52（11-17） | `c-list3.tsv` | 华为云直播页 `c4-live`；华为云产品页 `c4-cloudrobo`；IT之家 `c4-ithome`；TPS `c4-tps`；BBC Korean `c5-bbc`；Yonhap `c5-yna`；etnews `c5-etnews`；Korea Herald `c5-koreaherald`、`c5-koreaherald2`；TechCrunch `c1-techcrunch`；Quartz `c3-qz`；Euronews `c2-euronews` | 11 ok；**Quartz `c3-qz` blocked（HTTP 403，Cloudflare 质询），未绕过，无 `.txt`** |

所有网页存档为 `c-<name>.html|.headers|.txt`（JSON 为 `.json`），日志在 `c-fetch-log.jsonl`。单个文件均小于 1 MB（最大 `c-c2-euronews.html` 约 306 KB）。

## 搜索（仅作线索，经 `mcp__99e1748b__web_search_exa`，ToolSearch 以 `exa web search` 关键词加载）

| 时刻（北京） | 查询 | 用途 | 结果摘要 |
|---|---|---|---|
| 00:02 左右 | 华为云 CloudRobo WM1.2 发布、官方 | C4 找发布后官方页 | 得到华为云直播页与产品页（直播页为议程，产品页含 CR-VLA-Dual 的 98.35%）；IT之家、财联社、凤凰网、veryin 转述 |
| 00:02 左右 | 韩国 税务师 AI 作弊 自首 | C5 找主管机关或 L4 原站 | 得到 BBC Korean、Yonhap、Korea Herald、머니투데이、etnews、SBS（SBS 未存档） |
| 00:03 左右 | CloudRobo-WM1.2 98.35% 核查 | C4 核实数字归属 | 98.35% 仅见于 CR-VLA-Dual 句；TPS 转述 88.98%/81.7% |
| 00:03 左右 | TechCrunch Apple（查询） | C1 找 TechCrunch | 找到 TechCrunch 原页与时间戳 |
| 00:03 左右 | Reuters ECB Vujčić（查询） | C2 找 Reuters | 找到 Reuters 经 Euronext live 转载（摘录）、Euronews、3CBlock、UOL 葡文（Reuters 转载） |
| 00:03 左右 | Reuters Barr（查询） | C3 找 Reuters | Reuters 原页未返回；得到 Quartz、Bloomberg Law、Investing.com、casrai（转述）；Fed 官方页元数据 |

搜索摘录不是存档，本组按原则未将其当作一手依据；需要引用的句子均以已存档的一手原文核对过（ECB、Fed、Apple 官方页；Xena 原文；华为云产品页与 IT之家；BBC/Korea Herald/etnews/Yonhap 存档）。

## 热度（`barking heat`）

| 时刻（北京） | 命令 | 结果 |
|---|---|---|
| 23:59:22 | `heat docs/2611/1115 c --item 50515058` | HN 50515058：162 分，嵌套评论 59 条；写入 `c-heat.json`（随后改名为 `c-heat-c6-item.json`，见下）、`c-hn-50515058.json` |
| 23:59:42 | `heat docs/2611/1115 c --hn "Apple Health AI Usage Study"` 等 5 个关键词 | 全部 0 命中（北京 11-09 23:59:42 起）；写入 `c-heat.json` |
| 23:59 | 第二次 `--item` 调用 | 因 `c-heat.json` 已存在被工具拒绝覆盖（未写入）；之后改用上一步 |

Reddit 未查（上期同类搜索被重定向登录，本期未重试）。

## 文件操作

- 将 `c-heat.json`（C6 条目快照）改名为 `c-heat-c6-item.json`，为保留 C6 数据、便于 HN 搜索命令写入新的 `c-heat.json`。**改名，未删除任何文件。**
- 未调用 `rm`。未 commit、未 push。未修改仓库中 `docs/2611/1115/` 之外的文件。

## 失败与缺口

- Quartz（`c-c3-qz`）403 Cloudflare 质询：未绕过，未存档全文。
- Reuters 原页（Barr、Vujčić）未取到一手；Reuters 经 Euronext live 的摘录作为线索。
- 华为云发布后的官方新闻稿未找到。
- 公团（HRDK）、警方官网未取到。
- SBS 报道仅搜索命中，未存档。
- Verge 仅搜索摘录，未存档全文。
- Reddit 未查。

## 待删除文件

无。

## 假设

- 所有 `.html` 为抓取时的服务器返回；未检查页面显示时区，时间戳以北京时间换算。
- 日期在工作中由 2026-11-16 变为 2026-11-17（00:00 后）；C 组时刻记录按实际抓取时刻。
