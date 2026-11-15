# 1114 期 C 组（速览）抓取日志

时区：北京时间（UTC+8）。工具：`barking fetch`（自动记入 `c-fetch-log.jsonl`）、`barking heat`（记入 `c-heat.json` 与 `c-hn-<id>.json`）、curl（个别请求）、WebSearch / WebFetch（只作线索，不存档）。

| 时刻（北京） | 动作 | 对象 | 结果 |
|---|---|---|---|
| 20:54:20–20:54:22 | barking fetch（清单 6 条） | C1 Google 研究博客；C2 Project Zero；C3 Google 博客（Wallet）；C4 DeepSeek 更新日志；C5 EFF；C6 Ars Technica | 6 条 200 ok，blocked 0，failed 0。存档 `c-c1-…` 至 `c-c6-…`（html、headers、txt） |
| 20:55:55 | barking heat（`--item 50496811 --item 50496051 --item 50496602`） | HN 条目 C1、C2、C5 | 3 条写入 `c-heat.json`、`c-hn-50496811.json`、`c-hn-50496051.json`、`c-hn-50496602.json`。errors 空 |
| 20:55 | barking heat（`--item 50496...`，首次） | — | 报错：`--item` 须为数字 id。未写入任何文件（用户侧输入错误，之后未重复） |
| 20:56 | barking heat（`--hn` 查询 C3、C4、C6） | HN 检索 | 拒绝覆盖：`c-heat.json` 已存在（见 evidence-c.md 第四节） |
| 20:56:25–20:56:29 | barking fetch（清单 2 条） | hintonlectures.com 首页与 2026 页 | 两条都 200 ok。两份存档内容相同，最终跳转到 aisfoundation.ai/hinton-lectures-2026 |
| 20:56:45 | barking fetch（zh-cn 更新日志） | api-docs.deepseek.com/zh-cn/updates | 200 ok，`c-c4-deepseek-updates-zh.*` |
| 约 20:57 | barking fetch（清单 1 条） | openai.com 任职公告 | 200 ok，`c-c6-openai-board-announcement.*`（公告日期 September 9, 2026） |
| 约 20:58 | curl 调 hn.algolia.com（`search`，3 个查询） | C3 “Google Wallet Gemini”、C4 “DeepSeek API web search”、C6 “Christiano AI accident” | 无与三条速览对应的 HN 故事；只见无关结果。结果存于 `/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/7a5ecbf8-2ee0-4e64-9d2a-f18d3fdb536d/scratchpad/hn-search.txt`，未入库 |
| 约 21:00 | curl 调 hn.algolia.com（`search_by_date`，第二轮 5 个查询） | 补充 C3、C4、C6 关键词 | 同上，无对应故事。第二轮结果同样存于 `/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/7a5ecbf8-2ee0-4e64-9d2a-f18d3fdb536d/scratchpad/hn-search.txt`，未入库 |
| 约 20:58–21:05 | WebSearch（4 次） | Christiano 讲座与头衔；“1:1 复刻 DeepSeek App”稿件 | 线索：Hinton Lectures 官方站、Webull/Macau Business 转发稿、BetaKit 二手报道（称周二、第二场）、OpenAI 公告。澎湃稿：未找到 2026-11 相关报道，只找到 10 月相关稿。搜索结果不入库，只作线索 |
| 约 21:05 | WebFetch（PR Newswire 镜像） | streetinsider.com 上的 Hinton Lectures 新闻稿 | HTTP 403，未取到正文。未存档，未重试 |
| 约 21:06 | WebFetch（Webull 转发稿） | webull.com 新闻稿 | 取到：发布日期 2026-10-05，列 Christiano 为 ARC 创办人与执行董事，并称 NIST CAISI 高级技术顾问。仅作线索，未存档；与官方公告的差异见 evidence-c.md |

## 失败与未完成

- C3、C4、C6 的 HN 热度：未写入 `barking heat`（工具拒绝覆盖），curl 检索未找到对应故事。结论为未找到，不是零热度。
- C6 讲座录像（YouTube `-Ef0ocEoBCs`）未抓取；Ars 引语未在录像或字幕中核对。
- 澎湃稿（C4 “1:1 复刻”）未找到。
- PR Newswire 镜像 403，未取到。
- 未做截图（C 组不需要）。
- 未做 Reddit 热度（派工单只要求 `barking heat` 的 HN 数据，C 组不写“可能的吠点”）。

## 隐私与存档检查

- `c-*` 文件 grep：仅命中官网公开联系邮箱（`name@example.com` 类占位、`api-service@deepseek.com`、`adinquiries@condenast.com`），无抓取者账号、昵称或头像。
- HN JSON（`c-hn-*.json`）含 HN 发帖人 id（`acossta`、`hacb`、`gslin`），属第三方公开作者，不是抓取者账号。
- 所有 `c-*` 文件均小于 1 MB（最大为 `c-c6-openai-board-announcement.html`，约 363 KB），无需先问是否入库。
