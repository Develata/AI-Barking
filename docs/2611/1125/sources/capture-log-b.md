# 1125 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，UTC+8；时间为北京时间（本机时钟，取证从 21:38 起，约 22:00 写成）。批量抓取逐条结果在 `b-fetch-log.jsonl`，截图动作在 `b-shots-log.jsonl`，热度在 `b-heat.json`。`export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"` 后用 `barking shot`。不用 firecrawl，不读取浏览器 Cookie，未登录、未发帖、未点赞。**opencli 浏览器的会话可能登着维护者账号**：本组只用它读公开页并截图（rh-shrink、Zulip 匿名 web-public 页、头条文章），截图与存档已逐张自查，无登录栏、头像、昵称；`grep -i develata` 对 `b-*` 文件命中 0。

## 一、网页存档（barking fetch，限时并发）

| 批 | 北京时刻 | 清单 | 结果 |
|---|---|---|---|
| 1 | 21:39:11 | grh-lean 仓库页与根 README raw、rh-shrink 首页与仓库、rh-kernel-backbone、rh-trim、rh-proof-trimmed 仓库页、riemann.verifytask.cn、HN 50648050、alpo.ge/remark.pdf | 10 条全部 ok（HTTP 200）。VerifyTask 的 `.txt` 只有“Unable to load live board data”壳，数据需另取 API（见二）；rh-shrink 首页数字由 JS 渲染，另取 `data.json` 等（见二） |
| 2 | 21:40:27 | 社区仓库 README（main、`alpoge`、`original`）、社区仓库页、MIT Sutherland 原版页、Lean 博客声明页、lean-fro.org、mathlib-initiative.org、alpo.ge 首页、`alpoge` 分支 `src/Statements.lean` | 9 ok；`original/README.md` 404（该分支无 README 或分支名不同，不影响） |
| 3 | 21:40:54 | MIT `shortgrh` 页、rh-kernel-backbone 站点（GitHub Pages） | 2 ok |
| 4 | 21:44:49 | 中文稿：IT之家（量子位稿）、腾讯新闻量子位 11-26 稿、头条 7712294137042780698、极客公园、36氪（量子位 11-24 稿）、腾讯新闻“楼宇风运录”240 万稿、中华网、腾讯研究院 anyGeometry 澄清稿、知乎问题页 | 7 ok；头条 `blocked`（正文需 JS 渲染）、知乎 `blocked`（403）。头条文章改用 opencli 浏览器渲染后取正文（见二） |
| 5 | 21:45:53 | 腾讯新闻夕小瑶稿、什么值得买稿、头条 7712368986859635261（“被压缩 99.5% 这一说法”）、OfficeChai | 3 ok；头条 `blocked`，改 opencli 取（见二） |
| 6 | 21:54:28 | PhilArchive 质疑论文页、dev.to Frank Chu 稿、台湾自由时报 AI 页 | 2 ok；**PhilArchive 403（Cloudflare 质询）：blocked，未绕过**，质疑原文只取到 X 帖（见二） |
| 7 | 21:58 | anthropic.com/news、anthropic.com/research、en.wikipedia.org Alpöge 页 | 3 ok；两个 Anthropic 页 grep 无 riemann / lean / grh / sernau |

`.txt` 已抽查为正文（非质询页）：rh-shrink 首页 `.txt` 只含说明段，数字不在其中（JS 渲染），见二；VerifyTask `.txt` 只有壳页。

## 二、个别请求（curl / gh / opencli / Python）

- GitHub API（`gh api`，只读公开元数据）：`lsernau/grh-lean`、`leanprover-community/riemann-hypothesis`（仓库、提交列表、分支列表、提交 47fcde57 / b7a0adab / 686902ae / 9211277d / 4bd7145d）、rh-shrink / rh-kernel-backbone / rh-trim / rh-proof-trimmed 仓库元数据与 README（raw）、`users/lsernau`（只看，未存）；北京 21:38–21:40。API 响应存档为 `b-gh-*.json` / `b-gh-*-README.md`；其中的 URL 含账号名为公开项目账号，抓取者身份不在其中（已 grep）。
- GitHub 代码搜索 `gh api search/code`：`anyGeometry repo:homeowmorphism/riemann-hypothesis-original`（23 个文件），存 `b-gh-code-anygeometry-orig.json`（带 `text_matches`）。该查询用了本机已登录的 gh 令牌，响应不含令牌或账号信息。
- `git clone --depth 50 https://github.com/lsernau/grh-lean.git`（北京 21:38），随后 `mv` 到 scratchpad（见“待删除”）；实测命令与输出见 `b-grh-lean-measure.txt`（`find`、`wc -l`、`cloc --quiet`）。克隆内 git 作者邮箱为公开提交元数据，**未写入任何存档**。
- rh-shrink：`curl -sS --max-time 60 https://rh-shrink.github.io/{data,earlier,polled}.json` → `b-rh-shrink-data.json`（北京 21:39；55 条）、`b-rh-shrink-earlier.json`（206 条）、`b-rh-shrink-polled.json`（polled 2026-11-26T13:30:44Z）。
- VerifyTask：`curl -sS --max-time 60 https://riemann.verifytask.cn/api/board` → `b-verifytask-board.json`（北京 21:41，etag 8fce4b5f91b353a1，1,148,179 字节，**超 1 MB**）；rh-kernel-backbone 的看板快照 `data/verifytask_board.json`（raw）→ `b-backbone-verifytask_board.json`（793,241 字节）；rh-kernel-backbone `SPEC.md`（raw）→ `b-backbone-SPEC.md`；原版镜像 `Statements.lean`（raw，`homeowmorphism/riemann-hypothesis-original`）→ `b-statements-original-mirror.lean.txt`。
- Lean Zulip：匿名 web-public 接口分页取话题 “New proof in 300k lines”（频道 658556）全文 403 条，脚本 `b-zulip-fetch.py`（改自 `docs/2611/1120/sources/a-zulip-fetch.py`，只改频道号与话题名），北京 21:50 前后；完整 JSON 留 scratchpad，**不入库**；交付只留被引用的 8 条 `b-zulip-excerpts.md`（含 Sernau 6 条、Kim Morrison 2 条与被问者的提问引文，不含邮箱与头像字段）。
- opencli（`opencli browser --session b-shot`）：①读 rh-shrink 渲染后数字与截图（隐藏浏览器翻译插件浮层 `#immersive-translate-popup`，仅 CSS `display:none`，未改页面内容）；②读 Zulip 匿名页并截图；③读头条文章正文：`b-cn-toutiao-correction.txt`（北京 21:5x）；④用 `opencli yahoo search` 搜中文稿与 Luke Sernau 公开信息（结果只当线索）。**opencli 浏览器与 headless Chrome 均无法访问 `riemann.verifytask.cn`**：浏览器报 `ERR_CONNECTION_CLOSED`，headless 渲染出无样式页面；curl 可通。**按规则不绕过、不用代理或镜像**。会话已 `close`。
- fxtwitter 公开 JSON：`https://api.fxtwitter.com/status/2124984931063673166`（Nielsen 的 anyGeometry 帖，北京 21:5x）；只存帖文与时刻 `b-nielsen-x-excerpt.txt`，不存头像、简介等。完整 JSON 留 `/tmp/fx-nielsen.json`（待删除）。
- `barking heat`（`b-heat.json`，北京 21:42:10）：HN 条目 50648050 / 50654144 与 `grh-lean` 关键词；Reddit 两组搜索被重定向到 `old.reddit.com/login`，**失败，未登录**，记入 `b-heat.json` 的 errors。

## 三、失败与受限

| 项 | 原因 | 处理 |
|---|---|---|
| Reddit 热度 | 搜索 JSON 被登录墙拦 | 未取，如实写“未取” |
| PhilArchive 质疑论文页 | Cloudflare 质询 403 | 未绕过；只用 X 帖与 dev.to / 腾讯二手稿，身份“堪萨斯大学博士生”未核 |
| 知乎问题页 | 403 | 未取 |
| 头条两篇 | `barking fetch` 判 blocked（需 JS） | 改 opencli 渲染后取正文，一篇存档，另一篇（7712294137042780698）只用于标题检索，未存正文 |
| VerifyTask 页面渲染与截图 | 浏览器 `ERR_CONNECTION_CLOSED`；headless 无样式 | 数据改用 `/api/board` 与 rh-kernel-backbone 的快照；17 号位原计划的榜单截图**未取** |
| `lc-rh original/README.md` | 404 | 无影响 |
| 澎湃、今日头条“三个数字”11-25 14:46 稿、“8 天”原句、“24.1 万重复定理”出处 | 搜索与已存稿均未命中 | 记“未找到” |
| 本机构建 grh-lean | 工具链与 Mathlib 缓存需数小时 | 未做；证据用作者自述 + MIT 页 |

## 四、截图动作

`b-shots-log.jsonl` 记 `barking shot` 的 render / stack / cropbox 动作；opencli 与 Zulip 的两张原图在 scratchpad（`shrink-top2.png`、`zulip2.png`），裁切同样用 `barking shot cropbox|stack --from`，记录在同一日志。

| 号 | 取法 | 备注 |
|---|---|---|
| 13 | headless Chrome 渲染 grh-lean 仓库页（1000 CSS px，DPR 2，深色主题）；`stack`：About 块 + 文件列表与 README | 裁掉仓库所有者头像与提交栏；About 块被裁窄 |
| 14、15 | headless Chrome 渲染 `…/blob/main/grh/README.md`；`cropbox` | 深色底 |
| 16 | opencli 浏览器，zoom 1.5（实际约 DPR 1.5，**不是 2**），`--width 1400`；`cropbox --from`；已隐藏翻译插件浮层 | 页首数字卡 + “Relative to the original” 曲线；底部白色“Lines”图未载入，已裁掉 |
| 17–19 | headless Chrome（1000 或 1400 CSS px，DPR 2）渲染仓库页；`cropbox` | 17 本为 VerifyTask 号位，改放 rh-proof-trimmed 阶段表 |
| 20、21 | headless Chrome 700 CSS px；21 用 6500 高窗口避免悬浮按钮压字，`stack` 三段 | 腾讯新闻页自带横幅广告，已裁掉；21 暗底 |
| 22 | headless Chrome 渲染 HN 两页（主帖、评论 50648590 的永久链接页），`stack` 三段 | HN 评论页只回显 bananaflag 一行与 gregdeon 评论（页面如此） |
| 23 | headless Chrome 渲染 rh-shrink（JS 图表未载入，只取“How it's measured”说明段） | 深色底 |
| 24 | opencli 浏览器，zoom 2，Zulip 匿名 web-public 页；`stack` 取话题条与消息正文 | 已裁去头像；页面在“Log in”匿名态 |

## 五、本组自写脚本

- `docs/2611/1125/sources/b-zulip-fetch.py`：匿名取 Lean Zulip 话题全文（见上）。
- 抽取与统计（rh-shrink 时间序列、VerifyTask 时间序列、cloc 汇总）为一次性内联 Python（`python3 -I -`），未存文件；结果与口径已写入 `evidence-b.md`。

## 六、本组假设

1. “原版”按 rh-shrink 的原始点 `40e56bab`（社区镜像首个提交）与 Lean FRO 声明的 “17 million” 理解；不独立核对镜像与 alpo.ge 原包逐字相同（MIT 原版页 B2-5 称镜像 “byte-identical … apart from added Comparator files” 为 dev.to 引文，L6）。
2. 把 `leanprover-community/riemann-hypothesis` `main` 换版（`47fcde57`）视为 rh-shrink 曲线的断点。
3. “有效代码”沿用 rh-shrink 的 code* 定义（去空行、纯注释行与文件头 import）；cloc 的 code 不去 import，二者差约 1,742 行（可读版 299,902 − 298,160）。
4. 作者身份只记其公开发言与仓库署名；未收个人信息。

## 七、待删除文件（精确路径）

- `/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/10a43d57-e80a-4eba-b25f-59c6c3f0fa87/scratchpad/grh-lean`（grh-lean 浅克隆，约 40 MB，含 `.git`）
- `/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/10a43d57-e80a-4eba-b25f-59c6c3f0fa87/scratchpad/zulip-cleaning.json`（空结果）、`zulip-300k.json`（Zulip 全文，含发言者邮箱等字段，不入库）、`shrink-top.png`、`shrink-top2.png`、`zulip1.png`、`zulip2.png`
- `/var/folders/hp/f92qcyq13vl60r97clcqkv8w0000gn/T/barking-shot/26111125-b/`（barking 的中间全页图，目录整体）
- `/tmp/fx-nielsen.json`、`/tmp/bing.html`
- 仓库内：无（本组所有 `b-*` 文件与 `images/13–24` 均是交付物；`b-fetch-list-1…7.tsv` 为清单，保留）
