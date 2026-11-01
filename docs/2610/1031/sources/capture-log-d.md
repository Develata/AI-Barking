# 1031 期 D 组抓取日志

执行方：claude-haiku-5-5。时间一律北京时间（本机 EST 时间见括号）。机器可读版：`sources/d-fetch-log.jsonl`、`sources/d-heat.json`、`sources/d-hn-*.json`。

| 北京时间 | 动作 | 工具 | URL（去查询参数） | 结果 | 备注 |
|---|---|---|---|---|---|
| 11-02 02:32:00（11-01 13:32 EST） | 批量存档 4 个页面（`barking fetch docs/2610/1031 d - --jobs 4`，清单经标准输入） | barking fetch（curl） | https://karpathy.bearblog.dev/hacker-tube/ | 200 ok，14,980 B，`d-karpathy-hackertube-post.{html,txt,headers}` | 正文 33 行 |
| 同上 | 同上 | 同上 | https://nolanlawson.com/2026/10/28/the-ai-cost-of-javascript-frameworks/ | 200 ok，83,645 B，`d-nolanlawson-js-framework-cost.{html,txt,headers}` | 正文 855 行，表格与数字完整 |
| 同上 | 同上 | 同上 | https://thedataexchange.media/leo-de-moura-lean-aws | 200 ok，135,730 B，`d-dataexchange-de-moura.{html,txt,headers}` | 正文 706 行，含 "polished and edited transcript" 字样 |
| 同上 | 同上 | 同上 | https://www.wired.com/story/i-spent-30-minutes-with-apples-ceo-now-i-know-why-john-ternus-got-the-job/ | 200 ok，1,447,299 B，`d-wired-ternus.{html,txt,headers}` | 正文 326 行，正文完整；HTML 含 paywall 组件类名，文本中未见付费提示。未绕过，未做浏览器目视核验 |
| 11-02 02:32:02（11-01 13:32 EST） | HN 热度（`barking heat docs/2610/1031 d --item 50319510 --item 50323088 --item 50290652`） | barking heat（HN 条目接口） | https://hn.algolia.com/api/v1/items/50319510 | 1878 分；嵌套评论 454 条；`d-hn-50319510.json` | 0 个请求失败 |
| 同上 | 同上 | 同上 | https://hn.algolia.com/api/v1/items/50323088 | 217 分；嵌套评论 145 条；`d-hn-50323088.json` | 同上 |
| 同上 | 同上 | 同上 | https://hn.algolia.com/api/v1/items/50290652 | 208 分；嵌套评论 52 条；`d-hn-50290652.json` | 同上 |
| 11-02 02:32–02:36 | 按 advisor 复查补 D4 热度：`barking heat docs/2610/1031 d` 加 `--hn "Ternus"` 与 `--item` 重跑，被拒（“已有 d-hn-*.json，拒绝覆盖”），未写入任何文件；为此曾把 `d-heat.json` 临时改名，随即恢复，内容未变 | barking heat | — | 拒绝覆盖，未写入 | 之后不再重跑 heat，改用下两行一次性请求 |
| 11-02 02:36（11-01 13:36 EST） | HN Algolia 检索 "Ternus"（按时间，20 条，`curl --max-time 60`） | curl（一次性请求） | https://hn.algolia.com/api/v1/search_by_date?query=Ternus&tags=story&hitsPerPage=20 | ok，`d-hn-search-ternus.json`；最晚命中为北京 10-19 的 Time 文 | 无 WIRED 文章投稿 |
| 11-02 02:36（11-01 13:36 EST） | HN Algolia 按 WIRED URL 检索（`curl --max-time 60`） | curl（一次性请求） | https://hn.algolia.com/api/v1/search?query=wired.com%2Fstory%2Fi-spent-30-minutes&tags=story&hitsPerPage=10 | ok，nbHits 0，`d-hn-search-ternus-wired.json` | D4 HN 热度记为“无投稿” |
| 11-02 约 02:33（11-01 约 13:33 EST） | 只读核对：读取 `d-hn-*.json` 的字段（python3 -I，不写文件）；`rg` 在 `daily-scan.md` 与往期 `fact-check.md` 中查重（Karpathy、nolanlawson、Ternus、de Moura 等） | 只读命令 | — | daily-scan 仅 1011 期有"Karpathy 能力认知漏斗"，与 HackerTube 无关；无其他重复 | 未修改任何文件 |

## 未做的事（按派工单范围）

- 未用 opencli，未做截图（D 组不截图），未用 headless Chrome 渲染。
- 未访问 hackertube.org、HN 社区评论质疑、夸大标题实例（D 组不需要）。
- D4 的 HN 热度只做了检索（见上表两行 curl），结果为无投稿。
- 未使用 web.archive.org、镜像站或代理。
- 未触碰 X、Reddit。

## 失败与限制

- 本组 4 个抓取全部 ok，无 blocked / failed。
- WIRED 的付费墙状态未经浏览器目视核验，只能说明普通请求拿到了完整正文。
- `d-wired-ternus.html` 1.45 MB，按规则只存本地。
