# 1018 期 D 组抓取日志（Haiku 5.5，北京时间 2026-10-19）

| 北京时间 | 目标 | 工具 | 结果 | 备注 |
|---|---|---|---|---|
| 17:08:59 | D1 https://www.vals.ai/blogs/impossible-refund | barking fetch（清单 `d-manifest.tsv`） | ok，txt 206 行 | 存档 html+txt+headers |
| 17:08:59 | D2 https://arxiv.org/abs/2610.22319 | barking fetch | ok，txt 245 行 | |
| 17:08:59 | D3 README raw.githubusercontent.com/google-research/harness2/main/README.md | barking fetch | ok，209 行 | |
| 17:08:59 | D3 docs/EXPERIMENTS.md raw | barking fetch | ok，122 行 | |
| 17:08:59 | D3 https://github.com/google-research/harness2 | barking fetch | ok，634 行 | 页面含 stars 等 |
| 17:08:59 | D3 https://arxiv.org/abs/2610.14224 | barking fetch | ok，249 行 | |
| 17:08:59 | D3 https://arxiv.org/html/2610.14224 | barking fetch | ok，3367 行（html 1.2 MB，仅本地） | 全文 grep |
| 17:08:59 | D4 https://blog.picheta.me/post/killswitch-lang/ | barking fetch | ok，112 行 | |
| 17:08:59 | D5 Tao 博客 2026/10/18 页 | barking fetch | ok，1208 行 | |
| 17:08:59 | D6 https://browser-use.com/posts/browser-agents | barking fetch | ok，304 行 | |
| 17:08:59 | D8 help.openai.com ChatGPT 发布说明 | barking fetch | ok，3716 行（html 822 KB，仅本地） | |
| ~17:09 | D3 GitHub API `api.github.com/repos/google-research/harness2` | curl --max-time 60 | ok（exit 0） | 取 created_at、stargazers_count；`d-d3-h2-api.json` |
| ~17:10 | D4 https://bench.killswitch-lang.org/ | curl --max-time 60 | 200，SPA 外壳 2 KB，无正文 | 存为 `d-d4-killswitch-bench.html`；试生成的 txt（17 字节）已按普通 rm 删除（自生成试验文件） |
| ~17:10 | D4 榜单正文 | opencli browser `d-kill`（open --window background，eval document.body.innerText） | ok，渲染文本 3598 字节 | 存为 `d-d4-killswitch-bench-rendered.txt`；Methodology 与 “Last updated” 已读 |
| 17:11:34 | D6 https://browser-use.com/showcase/landscape | barking fetch（stdin 清单） | ok，103 行 | 查样本量与时间段，未找到 |
| 17:12:10 | D7 https://arxiv.org/abs/2609.25804 | barking fetch | ok，246 行 | |
| 17:12:10 | D8 platform.openai.com/docs/changelog | barking fetch | ok，3579 行（html 540 KB，仅本地） | |
| ~17:12 | D7、D2 secondary：WebSearch（3 次） | WebSearch（standard） | 返回二手汇总（aiweekly、hyper.ai、digg、orcarouter 等） | 仅作线索，未入库；The Decoder 原文未找到；OpenAI 原始发布说明未由搜索找到，改用 changelog 页 |
| ~17:13 | D7 https://arxiv.org/html/2609.25804v2 | barking fetch | ok，2674 行（html 806 KB，仅本地） | 核 502、14 模型、59.7%、25% 口径 |
| 17:13:01 | 热度：HN 50140462、50141681 条目；搜索 Harness2、KillSwitch、Minecraft、Vals | barking heat（首次） | HN 5 组，Reddit 0 组，1 个请求失败 | 失败：Reddit 搜索 “KillSwitch programming language” 返回登录页（`old.reddit.com/login`，text/html，非 JSON），未重试 |
| 17:14:51 | 热度：合并重跑，加入 Taste-Bench、Browser Use、ChatGPT Slides、GPT-Live 语音 | barking heat（清空本组 heat 文件后重跑） | HN 8 组，0 个请求失败 | 首次结果副本存 scratchpad；本组热度以本次为准。Taste-Bench、ChatGPT Slides、GPT-Live 0 条；Browser Use 无相关条目 |

| ~17:19 | D2 https://arxiv.org/html/2610.22319v1 | barking fetch（d2-argon-html） | ok，610 行 | HTML 机构宏 `\thepa` 未展开，机构未从 HTML 得出 |
| ~17:19 | D2 https://arxiv.org/pdf/2610.22319 | barking fetch（d2-argon-pdf，响应为 PDF，按 180 秒限时重取） | ok，pdf 6.2 MB（仅本地）、txt 966 行 | PDF 首页 “1 Google DeepMind” |
| ~17:20 | D3 全文 grep `offline` 与 `network/internet/egress/sandbox` | 本地 grep | — | 论文未交代执行器联网；offline 指 Meta-Harness 设定 |
| ~17:20 | 交付前泄露检查：`docs/2610/1018/sources/d-*` 与 `.headers` 中搜 `develata`、邮箱、本机用户路径、`cookie`/`authorization`、个人 handle | 本地 grep（-i -I） | 0 命中（develata、邮箱、本机路径、cookie、authorization） | 仅有公开机构链接（x.com/OpenAI、x.com/ValsAI、x.com/browser_use 等）、GitHub 组织头像 u/43830688、Gmail 等产品名文字、HTML 内的公开 Sentry 前端 key（页面自带，非用户凭据，html 仅本地） |

## 失败与未取

- Reddit 搜索（KillSwitch）：返回登录页，记失败，未换其他 Reddit 接口。
- The Decoder（D2、D7 的报道原文）：WebSearch 未找到原站 URL，未抓取。
- D8 派工单 URL 未给 API 发布说明的具体地址，改用 platform.openai.com/docs/changelog（已核 Oct 16 条目）。
- D6 landscape 页：未给样本量与时间段。

## 自生成的试验文件处理

- `sources/d-d4-killswitch-bench.txt`（17 字节，html2txt 对 SPA 外壳的空结果）：已用普通 `rm` 删除，属于自己刚生成的试验文件。
- `sources/d-heat.json`、`sources/d-hn-50140462.json`、`sources/d-hn-50141681.json`：因 `barking heat` 拒绝覆盖，先将第一次生成的同名文件复制到 scratchpad，再用普通 `rm` 删除，随后由合并重跑重新生成。
- `sources/d-manifest.tsv`：本组抓取清单，保留。

## 热度摘要（HN，北京时间）

| 条目 | 分 / 评 | 时刻 |
|---|---|---|
| HN 50140462（Vals 退款实验） | 83 / 103 | 10-18 20:22 |
| HN 50141681（Tao 博客 Maggi 文） | 140 / 166 嵌套 | 10-18 23:00 |
| HN 50135762（KillSwitch 博客） | 184 / 126 | 10-18 06:26 |
| HN 50118352（Minecraft arXiv） | 4 / 0 | 10-16 21:47 |
| HN 50146503（Harness2 GitHub） | 5 / 0 | 10-19 07:19 |
| HN 50149398（Harness2 arXiv） | 1 / 0 | 10-19 15:24 |
| HN 50105503（dair.ai Harness2 页） | 2 / 0 | 10-15 23:53 |
| HN 50108422（Vals 旧帖） | 2 / 2 | 10-16 02:50 |

## 不入库与敏感信息

- GitHub API 返回的 JSON 只含仓库元数据，无用户个人信息。
- 所有 `.html` 原件仅本地保存（不入库，待 Claude 传网盘）；`.txt` 均小于 1 MB。
- 未使用任何登录态；未访问 Reddit 登录后页面；未发帖、点赞或关注。
