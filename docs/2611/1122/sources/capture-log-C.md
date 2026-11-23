# 1122 期 C 组（速览）抓取与检索日志

时间为北京时间（UTC+8）。网页存档用 `barking fetch`（日志 `c-fetch-log.jsonl`，34 行）；HN 热度用 `barking heat`（`c-heat.json`）与 HN Firebase API（`c-hn-extra.json`，因 `c-heat.json` 已存在、工具拒绝覆盖而改用 API）；渲染页面用 opencli（browser session `c-pnp`、`c-glm`、`c-aihot`）。

| 时刻（北京） | 动作 | 对象 | 工具 | 结果 |
|---|---|---|---|---|
| 00:11 | fetch 批 1 | z.ai/blog/glm-5.4 | barking fetch | blocked（正文 0 非空白字符，拦截页） |
| 00:11 | fetch 批 1 | mp.weixin.qq.com GLM-5.4 文章 | barking fetch | ok（140 行文本；页面 publish_time 换算 17:40:42） |
| 00:11 | fetch 批 1 | status.openai.com/incidents/n254wyd7nml7 | barking fetch | ok；内容为 2023 年事故（见 evidence-C C2） |
| 00:11 | fetch 批 1 | status.claude.com/ | barking fetch | ok（655 行） |
| 00:11 | fetch 批 1 | openai.com/index/perplexity-and-intelligence/ | barking fetch | ok（250 行） |
| 00:11 | fetch 批 1 | pequalnp.com/ | barking fetch | ok（184 行；静态 HTML，计数为空） |
| 00:11 | curl | hn.algolia.com 检索 GLM-5.4、Perplexity、pequalnp | curl | 得 HN 条目 ID 与检索时分数 |
| 00:11 | curl | status.openai.com/api/v2/incidents.json | curl | 25 条，2026-10-23 至 2026-11-23；存 `c-c2-openai-api-incidents.json` |
| 00:11 | curl | status.claude.com/api/v2/incidents.json | curl | 存 `c-c3-claude-api-incidents.json`（取 11-23 事故全部更新） |
| 00:12 | fetch 批 2 | pequalnp.com/feed.atom、/about、/live；aihot.news；news.ycombinator.com | barking fetch | feed、about、live、HN 首页 ok；aihot.news failed（协议错误） |
| 00:12–00:14 | opencli open + eval | pequalnp.com 首页 | opencli（session c-pnp） | 成功，取得计数文本（已摘录到 `c-c5-pnp-dashboard-0012.txt`） |
| 00:13 | opencli open + eval | aihot.news | opencli（session c-aihot） | 失败：`ERR_SSL_PROTOCOL_ERROR`，未取得精选 |
| 00:13 | opencli open + eval | pequalnp.com/about | opencli | ok，但 About 页内无 “checked” 问答全文；点击与页内链接未展开答案 |
| 00:14 | opencli open + eval | pequalnp.com 首页再次加载 | opencli | 快照停在 “Loading the latest snapshot…”，之后两次重取失败（文件 `c-c5-pnp-dashboard-opencli.json` 为加载态，待删除） |
| 00:14 | opencli open + eval | z.ai/blog/glm-5.4 | opencli（session c-glm） | 成功；正文存 `c-c1-zai-glm54.txt` |
| 00:14 | barking heat | HN 50609685、50601525、50604314 | barking heat | ok；`c-heat.json`、`c-hn-*.json` |
| 00:14 | fetch 批 3 | 6 个 AI 实验室/新闻索引页（openai、anthropic、deepmind、google ai、mistral、deepseek api-docs） | barking fetch | 5 ok；deepseek 404 |
| 00:15 | fetch 批 4 | Gemini 4 Neon、Genie 4、Gemini 4 Carbon、Gemini 4 Hydrogen、AlphaProof Code | barking fetch | 5 ok；日期核对：Neon 与 Genie 4 为 Nov 18；Carbon Nov 10；Hydrogen Nov 12；AlphaProof Code Nov 10，均在窗口外 |
| 00:15 | WebSearch | Microsoft 回应 Pachocki 困惑度文 | WebSearch | 未找到相关材料（两次检索，返回 MAI 模型转述，与本事件无关） |
| 00:15 | fetch 批 5 | microsoft.ai/news（两个 URL） | barking fetch | 内容相同；MAI Code 2 与 Code 2 Flash 为 11-17，窗口外 |
| 00:15 | fetch 批 6 | qwen.ai/blog | barking fetch | blocked（正文过短，疑似 JS 渲染；未用 opencli 重取） |
| 00:15 | fetch 批 6 | ai.meta.com/blog/ | barking fetch | failed（HTTP 400） |
| 00:15 | fetch 批 6 | x.ai/news | barking fetch | blocked（HTTP 403） |
| 00:15 | fetch 批 6 | huggingface.co/blog | barking fetch | ok；“How to build and maintain an agentic evaluation” 11-23（教程，不收） |
| 00:15 | fetch 批 6 | blogs.nvidia.com 生成式 AI 分类 | barking fetch | ok；无窗口内条目，未细读 |
| 00:15 | fetch 批 7 | news.ycombinator.com/?p=2、/newest | barking fetch | ok；`c-c6-hn-newest` 列出 11-23 16:00–16:13 UTC 的新帖，取 AI 相关条目 |
| 00:15 | fetch | 404 Media ARTEX 文章（URL 首次写错，HTTP 404） | barking fetch | 正确 URL 后 `--force` 重取 ok |
| 00:15 | fetch | Stanford AI100 2026 报告 PDF | barking fetch | ok（PDF 7.2 MB，存本地；文本 376 KB 入库） |
| 00:16 | WebSearch | 11-23 AI 公告汇总 | WebSearch | 多为第三方聚合站，无 11-23 一手；已忽略聚合站内容 |
| 00:16 | fetch | OpenAI GPT-6 付费对话事故详情（API） | curl | 返回 Next.js 错误页 HTML（存为 `c-c6-openai-gpt6-incident.json`，无用） |
| 00:17 | WebSearch | Microsoft 回应 Pachocki 文（第二次） | WebSearch | 未找到 |
| 00:18 | opencli 重试 | pequalnp 首页 | opencli | 仍为加载态，放弃 |

## 失败与未取得

- z.ai 博客：curl 被拦，已改 opencli 取得正文（ok）。
- aihot.news：`ERR_SSL_PROTOCOL_ERROR`（opencli）与协议错误（barking fetch）。未取得 11-23、11-24 精选。
- pequalnp About 的 “How do I know ‘checked’ isn’t made up?” 答案：未找到直链，未取得。
- pequalnp 首页快照：00:14 后停在加载态，未能再次取得计数。以 00:12–00:14 的摘录为准。
- Qwen 博客、Meta AI 博客（400）、xAI 新闻（403）：未取得。
- Microsoft 对 OpenAI 困惑度文的回应：未找到。
- OpenAI GPT-6 付费对话事故详情：未取得。
- 未取得 Reddit 热度（本组 HN 已足够，未另查）。

## 假设与说明

- 北京时间：本机即北京时间（UTC+8），HN 与 status JSON 的 UTC 时间已换算。
- 页面只给日期的（z.ai 博客、OpenAI perplexity 文、Stanford AI100）如实写“官方未给时刻”。
- 404 Media 页面时间 “9:16 AM” 未标时区，未换算，仅如实注明。
- C2 的 “2023 年” 由页面自身的时间戳得出，未另行核验 OpenAI 状态页的时区设定。
- 热度数值以抓取时刻为准，HN 分数与评论数随时间变化。
- 未操作任何 X、微博等平台；未登录任何账号；未存档登录态页面。

## 对 Claude 收尾的待删除清单（精确路径，本组未执行 rm）

1. `/Users/develata/gitclone/AI-Barking/docs/2611/1122/sources/c-c6-openai-gpt6-incident.json`（实为 HTML 错误页，无用）
2. `/Users/develata/gitclone/AI-Barking/docs/2611/1122/sources/c-c5-pnp-dashboard-opencli.json`（加载态空壳，已由 `c-c5-pnp-dashboard-0012.txt` 取代）
3. `/Users/develata/gitclone/AI-Barking/docs/2611/1122/sources/c-c6-ms-mai-code2.html`、`.txt`、`.headers`（与 `c-c6-ms-ai-news.*` 内容相同的重复抓取）

## 入库注意

- `c-c6-stanford-ai100.pdf`（7.2 MB）为 PDF，按规范只存本地、不入库；其余 PDF 原件同理。
- `c-c6-stanford-ai100.txt` 含报告致谢名单中的个人姓名与社交 handle（报告公开文本），入库前请 Claude 决定是否保留。
- 未发现 X 平台截图或抓取者账号信息；pequalnp 渲染文本已去掉贡献者 handle。

## 补充轮（协调者 11-24 追加）

| 时刻（北京） | 动作 | 对象 | 工具 | 结果 |
|---|---|---|---|---|
| 00:19 | opencli open + eval | https://aihot.news/ | opencli（session c-aihot） | 失败：ERR_SSL_PROTOCOL_ERROR |
| 00:20 | curl --http1.1 | https://aihot.news/ | curl | 无响应（000） |
| 00:20 | opencli open + eval | http://aihot.news/ | opencli（session c-aihot2） | 页面文本为空 |
| 00:21 | curl -D - | http://aihot.news/ | curl | 302 → `https://block.charter-prod.hosted.cujo.io/warn.html?url=http://aihot.news/`（网络过滤器拦截页）；https 请求得到 `HTTP/1.1 200 Connection established` 后无响应。判断为本机网络层拦截，未绕过 |
| 00:19 | curl | status.openai.com/incidents/01M84HKHX73RF7HTVZQ2A9GFXP | curl | ok，486 KB HTML，存 `c-c6-openai-gpt6-incident-page.html`；已从页面抽取 Investigating/Identified/Monitoring 原句与时刻（页面时间经 API 时间戳校验为美东） |
| 00:22 | 本地处理 | c-c6-stanford-ai100.txt | python | 生成 `c-c6-stanford-ai100-redacted.txt`：删去 Acknowledgements 段（约 2,300 字符），两处图片署名 @handle 替换为占位符；验证 @ 剩余 0 处 |
| 00:23 | curl | nvidia 与 google 索引文本复查 | grep | NVIDIA 最新 Nov 16；Google 索引无可读日期，未找到窗口内条目 |

### 补充轮结论

- AIHOT：本机网络过滤器拦截，无法浏览精选；按规则不绕过。需 Develata 决定换网络或授权其他路径。
- 新的窗口内 L1 条目：仅 C6b（GPT-6 付费对话报错）。
