# 1126 B 组抓取日志

执行：Antigravity（Gemini 模型家族子代理）。本机 macOS，UTC+8；时间为北京时间（本机时钟，取证从 2026-11-27 21:12 起，约 21:48 写成）。批量抓取逐条结果在 `b-fetch-log.jsonl`，截图动作在 `b-shots-log.jsonl`，热度在 `b-heat.json`。设置 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"` 后用 `barking shot` 无痕渲染截图。不用 firecrawl，不读取浏览器 Cookie，未登录、未发帖、未点赞。对于 `theblockbeats.info` 等在 headless 下触发 WAF 的页面，使用已连接的 `opencli browser` 会话打开并移除弹窗/翻译浮层后截图；所有截图与文本存档已逐张用 `read_file` 目视确认并 `rg -i "develata"` 检查（命中 0），无抓取者个人信息泄露。

## 一、网页存档（barking fetch，限时并发）

| 批 | 北京时刻 | 清单 | 结果 |
|---|---|---|---|
| 1 | 21:12:51 | `b-fetch-list-1.tsv`：HN `50653477` 页、TechStackups 原文、Google Workspace Updates 11-24 博客、`quota-usage-tiers`、Vortner 博客、Suncly 研究文 | 6 条全部 ok（HTTP 200） |
| 2 | 21:13:25 | `b-fetch-list-2.tsv`：Workspace AI Tools Addendum、`configure-mcp-servers`、`configure-mcp-security`、`mcp-elicitation`、`mcp-server-tools`、`workspace-cli`、Gmail MCP 参考页（HTML 与 `.md.txt`）、Gmail 配额页、`quota-pricing`、`quota-dashboard`、Cloud OAuth 帮助页（`15549257`）、Admin 帮助页（`7281227`）、Google Cloud 博客、Workspace Updates 5 月博客、Google APIs ToS、Claude Workspace 连接器帮助页 | 17 条全部 ok（HTTP 200） |
| 3 | 21:14:09 | `b-fetch-list-3.tsv`：Gmail MCP 单工具参考页（`send_message`、`reply`、`forward`、`create_draft`、`update_draft`）、`gmail/api/guides/configure-mcp-server`、`workspace-api-user-data-developer-policy`、`workspace/release-notes`、`cli/release-notes`、Suncly GitHub `contradictions.json` | 10 条全部 ok（HTTP 200） |
| 4 | 21:17:45 | `b-fetch-list-4.tsv`：代表性媒体（Flavio Copes、动察 Beating / BlockBeats、Help Net Security、The Next Web） | 4 条全部 ok（HTTP 200） |
| 5 | 21:19:32 | `b-fetch-list-5.tsv`：Google Workspace `user_features`、`developers.google.com/workspace/terms` | 2 条全部 ok（HTTP 200） |
| 6 | 21:19:56 | `b-fetch-list-6.tsv`：代表性媒体（MLQ.ai、Jazz Security）、`workspace.google.com/terms/api-billing` | 3 条全部 ok（HTTP 200） |

全部 42 条 `barking fetch` 请求均为 `ok`，且 `.txt` 已抽查核实为页面正文（非质询页/登录页）。最大纯文本/JSON 存档为 `b-ws-release-notes.txt`（509 KB），**无单个超过 1 MB 的文本或 JSON 文件**（`b-oauth-agentic-help.html` 为 1.6 MB HTML 原件，不入库，交主执行方传网盘）。

## 二、个别请求与实测（curl / barking heat / gwscli）

- `barking heat`（`b-heat.json`，北京 21:12:25）：采样 HN 条目 `50653477`（356 分 / 307 条嵌套评论，另存 `b-hn-50653477.json`）及关键词 `"Google Workspace MCP"`（命中 4 条）、`"gmailmcp"`（命中 1 条）；Reddit 搜索被重定向到 `old.reddit.com/login` 失败，已记入 `errors`（见三）。另用内联 Python 解析输出 `b-hn-summary.txt` 辅助查阅（完整原数据在 `b-hn-50653477.json`）。
- 线上 MCP 端点实时探测（`curl` POST `tools/list`，北京 21:14 & 21:18）：
  - `https://gmailmcp.googleapis.com/mcp/v1` → `b-gmailmcp-v1-tools.json`（146 KB，30 个工具，含 `send_message`、`reply`、`forward`、`update_draft`）。
  - `https://gmailmcp.googleapis.com/mcp/v1self` → `b-gmailmcp-v1self-tools.json`（115 KB，25 个工具，不含上述 3 个发信工具与 2 个敏感标签工具）。
  - `https://drivemcp.googleapis.com/mcp/v1` → `b-drivemcp-v1-tools.json`（114 KB，15 个工具，仍无共享/改权限与永久删除工具）。
- 官方 `gwscli` 二进制实测（北京 21:18–21:19）：下载 `https://dl.google.com/workspace-cli/README.md` → `b-gwscli-readme.md`；将 macOS Apple Silicon 二进制 `https://dl.google.com/workspace-cli/macos_silicon/gwscli` 下载至 `/tmp/gwscli-check`（列待删）运行 `version`（`1.0.6.1026726559`）、`gmail --help`（确认返回 30 条含 `send_message` 的命令）、`drive --help`（15 条命令）。另取社区版 `https://raw.githubusercontent.com/googleworkspace/cli/main/README.md` → `b-github-gws-readme.md`。
- 发布时刻与历史快照核对：
  - `https://workspaceupdates.googleblog.com/feeds/posts/default?alt=json&max-results=10` → `b-ws-updates-feed.json`（确认 11-24 公告准确发布时间为 `2026-11-24T09:02:59.959-08:00` = 北京 11-25 01:02:59，更新于 `2026-11-26T14:28:37.307-08:00` = 北京 11-27 06:28:37）。
  - Wayback Machine（`web.archive.org`）可用性查询与快照抓取（仅用于核对 11-26 页面是否已更新）：`20261126211519`（`b-wayback-quotas-1126-raw.html/.txt`）与 `20261126211531`（`b-wayback-mcp-ref-1126-raw.html/.txt`），均已确认在 2026-11-26 21:15 UTC 时 Google 文档已更新为 30 工具并移除合作方脚注。
- Justin Poehnelt X 帖公开 JSON（`https://api.fxtwitter.com/status/2125529118133669982`，北京 21:18）：提取正文与时间戳保存为脱敏摘要 `b-jpoehnelt-x-excerpt.json`（原始响应 `b-jpoehnelt-raw.json` 含作者公开元数据，一并说明，无抓取者信息）。

## 三、失败与受限

| 项 | 原因 | 处理 |
|---|---|---|
| Reddit 搜索（`barking heat`） | `old.reddit.com/search.json` 重定向至登录页，返回 HTML | 不绕过登录墙，记失败于 `b-heat.json` 的 `errors` |
| `opencli` 首次试截 `workspaceupdates.googleblog.com`（`/tmp/b-shots/16-raw.png`） | `zoom='2'` 触发右侧轻微裁切且带翻译插件浮层 | 弃用，改用无痕 `barking shot render --width 700 --scale 2`，列待删 |
| headless Chrome 渲染 `developers.google.com` 默认显示中文 + 自动翻译条 | 本机系统语言环境导致跳转或本地化 | 所有待截谷歌开发者页均显式改用 `?hl=en` 与 `--chrome-arg=--lang=en-US` 重渲，取得纯正英文原页 |
| headless Chrome 渲染带 `#fragment` 锚点的 Google 文档 URL（`workspace-cli-hitl-en`、`quota-pricing-en`、`gmail-send-scopes-en`） | 带 `#` 锚点在 headless 截图时输出空白灰屏 | 改用不带 `#` 锚点的全页/加高渲染（`quota-pricing-full-en`、`quota-tiers-en`、`workspace-cli-full-en`）解决；`opencli` 试截的 `workspace-cli-hitl-opencli.png` 因右上角有个人头像且未滚到位已弃用，列待删 |
| headless Chrome 渲染 `theblockbeats.info` | 触发站点 WAF 拦截（`HTTP Error`） | 改用 `opencli browser --window foreground` 打开并移除订阅弹窗与翻译浮层后截图，再裁边输出 `26-blockbeats.png` |
| Wayback CDX 第二次查询（`mcp-server-tools`、`configure-mcp-server`） | `curl` 报 `SSL_ERROR_SYSCALL`（exit 35） | 未重试绕过，已有 `quotas` 与 `reference/mcp` 的 `202611262115` 快照及官方 `Last updated` 充分佐证 |

## 四、截图动作

全部正式出图（`docs/2611/1126/images/16-*.png` 至 `29-*.png`，宽 1200–1400 px，2 倍像素比）均已用 `read_file` 逐张打开目视确认文字清晰、上下文完整、裁切边界落在行间、无抓取者个人账号信息泄露。其中 **`23-techstackups.png` 与 `26-blockbeats.png` 为原站深色底页面**，其余均为白/浅灰底。`20-mcp-elicitation.png`（底部带入上一行半句 `"same action."`）与 `24-blockbeats.png`（底边切字+左侧残留分享按钮边缘）已被更干净的 `27-mcp-elicitation.png` 与 `26-blockbeats.png` 替代，列待删。

| 编号 | 文件名 | 来源与取法 | 备注 |
|---|---|---|---|
| 16 | `16-ws-updates-blog.png` | `barking shot render`（700 CSS px, DPR 2）+ `stack`（2 段） | Workspace Updates 11-24 公告标题、导语及 2027-02-01 MCP/CLI 强制与 2027-04-01 计费段 |
| 17 | `17-ai-tools-addendum.png` | `barking shot render` + `cropbox` | 《Workspace AI Tools Addendum》全文（Last modified: Nov 24, 2026，定义与条款 1–5） |
| 18 | `18-gmail-mcp-ref.png` | `barking shot render`（`?hl=en`）+ `stack`（2 段） | `gmailmcp.googleapis.com` 参考页，完整显示 `/mcp/v1`（30 工具，含 `send_message`/`reply`/`forward`/`update_draft`）与 `/mcp/v1self`（25 工具） |
| 19 | `19-gmail-send-message.png` | `barking shot render`（`?hl=en`）+ `cropbox` | `send_message` 官方工具页标题与完整功能说明 |
| 21 | `21-mcp-server-tools.png` | `barking shot render`（`?hl=en`）+ `stack`（2 段） | 9 个 GA MCP 服务器总览表（明确标出 Gmail 30 vs 25 工具、Drive 15 vs 12 工具等） |
| 22 | `22-quota-pricing.png` | `barking shot render`（`?hl=en`）+ `stack`（3 段） | 定价页 REST 每日免费额度表（70M/10M/1M）vs Scaled Tier MCP 零免费额度（从第 1 次请求计费）说明 |
| 23 | `23-techstackups.png` | `barking shot render` + `cropbox` | **深色底**；TechStackups 11-26 原文标题、AI 署名说明与前 3 段 |
| 25 | `25-hn-50653477.png` | `barking shot render` + `stack`（2 段，HN 浅米底 `#f6f6ef`） | HN `50653477` 标题（359 分/308 评）与 `bugthe0ry` 评论（`50663581`，指出 Google 刚更新支持发信） |
| 26 | `26-blockbeats.png` | `opencli browser`（foreground，去弹窗/去浮层）+ `cropbox` | **深色底**；动察 Beating / BlockBeats 11-26 19:06 夸张标题、时间与主体段落 |
| 27 | `27-mcp-elicitation.png` | `barking shot render`（`?hl=en`）+ `stack`（3 段） | Form elicitation 弹窗示例（`Allow Cymbal Agent to use send_message`）与 `/mcp/v1self` 降级说明 |
| 28 | `28-workspace-cli.png` | `barking shot render`（`?hl=en`）+ `stack`（4 段） | 官方 `gwscli` 概览与服务列表中的 Gmail 行（明确写出 `send, reply to, and forward emails`） |
| 29 | `29-quota-roadmap.png` | `barking shot render`（`?hl=en`）+ `stack`（2 段） | Usage tiers 里程碑全表（May 1, 2026 / Nov 24, 2026 / Feb 1, 2027 / Apr 1, 2027） |

## 五、本组自写脚本

- 无持久化独立脚本。`b-hn-summary.txt`、JSON 工具列表提取与 Wayback gzip 解压均通过一次性内联 Python（`python3 -`）完成。

## 六、本组假设

1. 将 Google 官方页面底部的 `Last updated 2026-11-26 UTC.` / `2026-11-27 UTC.` 结合 Suncly `contradictions.json`（`2026-11-26T11:03:29Z`）、Wayback 快照（`2026-11-26T21:15:19Z`）及 `b-ws-updates-feed.json` 的 `updated` 字段（`2026-11-26T14:28:37.307-08:00` = UTC `22:28:37`），共同作为判断 Google 于 11-26 晚间更新服务端与文档的依据；未向 Google 官方求证具体哪一分钟翻转线上开关。
2. `is_agentic`（`Use this client for an AI-powered agent`）对未声明代理的 REST 调用如何检测，因 Google 官方文档未披露检测方法，仅按官方文字记“未公开说明”。
3. 中文站动察 Beating / BlockBeats 的 `2026-11-26 19:06` 按北京时间（UTC+8）记，页面未附显式时区字符串。

## 七、待删除文件清单（精确路径）

仓库内废截图（已被 `27-mcp-elicitation.png` 与 `26-blockbeats.png` 替代）：
- `docs/2611/1126/images/20-mcp-elicitation.png`
- `docs/2611/1126/images/24-blockbeats.png`

仓库外临时试验文件：
- `/tmp/b-shots/`（目录整体，含 `16-raw.png`、`blockbeats-opencli.png`、`blockbeats-clean.png`、`workspace-cli-hitl-opencli.png`、`en-lines.txt`、`mcp-tools-lines.txt`、`quota-lines.txt`、`cli-lines.txt`）
- `/tmp/gwscli-check`（实测下载的官方 `gwscli` 二进制，约 11 MB）
- `/var/folders/hp/f92qcyq13vl60r97clcqkv8w0000gn/T/barking-shot/26111126-b/`（`barking shot render` 生成的中间全页 PNG，目录整体）
