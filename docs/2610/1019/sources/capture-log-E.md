# 1019 期 E 组抓取与热度日志

执行方：claude-haiku-5-5。时间一律北京时间（UTC+8）。日志机器记录见 `e-fetch-log.jsonl`、`e-heat.json`，本表为人工汇总。

| 北京时间 | 动作 | 工具与命令 | 目标 | 结果 |
|---|---|---|---|---|
| 19:13:06–19:13:10 | 批量存档（清单 `e-list.tsv`） | `barking fetch docs/2610/1019 e …` | 路透社、Browser Use、GitHub changelog、OpenAI 研究页 | 路透社 401 blocked（`e-reuters-openai.html` 774 字节，无 txt）；其余三条 200 ok，已生成 html/txt/headers |
| 19:14 | 路透社改用浏览器 | `opencli profile list` 显示两个已连接配置（5df8dbx7、87qqzcav），首次未指定报错；设 `OPENCLI_PROFILE=5df8dbx7`（任选，未核哪个是用户主用的配置） | 路透社原文 | 成功，见下 |
| 19:14 | 打开页面 | `opencli browser e-reuters open <url> --window background`；`get title`、`get url` | 同上 | 标题与 URL 正确；未点击、未输入、未登录 |
| 19:14–19:15 | 摘取正文与 HTML | `opencli browser e-reuters extract`（输出 JSON）、`get html` | 同上 | `e-reuters-extract.json`（13573 字节）、`e-reuters.html`（756548 字节） |
| 19:14–19:15 | 转文本 | `barking html2txt docs/2610/1019/sources/e-reuters.html docs/2610/1019/sources/e-reuters.txt` | 同上 | 625 行 |
| 19:15 | 脱敏（写盘后处理） | `sed` 删除 `e-reuters.txt` 中记者个人邮箱、LinkedIn、X、Instagram 链接行；`e-reuters.html`、`e-reuters-extract.json` 中个人邮箱与社交链接改为 `REDACTED`，`chrome-extension://…` 扩展 ID 改为 `REDACTED` | 同上 | 保留正文、记者公开职业简介与公司账号 |
| 19:15:57 | 热度第一轮（已弃用） | `barking heat docs/2610/1019 e --hn … --reddit …`（4 个 HN、4 个 Reddit 查询） | HN、Reddit | HN 有结果；Reddit 4/4 重定向到 `old.reddit.com/login`，记失败。随后删除自己生成的 `e-heat.json`（普通 rm，精确路径），重跑 |
| 19:16:34 | 热度第二轮（已弃用） | 同上，9 个 HN 查询，无 Reddit | HN | 顺带查到 HN 50155245 “Mercury 3: Diffusion at scale”（249 分 / 89 评，10/20 01:27 北京）；这是 C 组的主题，未存入 E 组热度，告知 Claude 备用。随后删除 `e-heat.json` |
| 19:17:01 | 存档 MultiLangBench 仓库页 | `barking fetch` 清单 `e-list2.tsv` | https://github.com/openai/MultiLangBench | 200 ok（262950 字节，414 行）。未登录；页面有 “Sign in” 按钮，无用户信息 |
| 19:17:47 | 热度第三轮 | `barking heat docs/2610/1019 e`：8 个 HN 查询 + 4 个 Reddit 查询 | `e-heat.json` | HN 8 组；Reddit 4/4 失败（同上登录页重定向） |
| 约 19:20 | 热度第四轮（终版） | 同上，删除第三轮的 `e-heat.json` 后重跑，HN 增加 `browser-use.com` 查询 | `e-heat.json` | HN 9 组，`browser-use.com` 与 E2 查询均 0 条；Reddit 4/4 失败。`e-heat.json` 为本轮结果 |
| 约 19:21 | 登录态核查 | `grep -o -i` 检索 `e-reuters.html` 的 “sign in / register / subscribe / my account / isLoggedIn” 等标记 | 同上 | 页头为 “Sign In” 与 “Subscribe”，无账户或订阅者标识，判为匿名态 |
| 19:13–19:18 | 文件改名 | 同一批 fetch 因清单名已带 `e-`，产物成了 `e-e-*`；改为 `e-*`，并用 `sed` 修正 `e-fetch-log.jsonl` 内的路径 | `docs/2610/1019/sources/e-*` | 完成。日志中的 `name` 字段仍为原清单名（带 `e-`）；下期清单名不要再加前缀 |

## 失败与未完成

- 路透社 curl 直取 401：记失败。打开来源是 opencli 浏览器，不是转载站。
- Reddit：4 次搜索（终版与第一轮各 4 次）全部被重定向到登录页，不是 JSON，未取得热度。
- HN 无命中：E2 “Browser Use Bench”（只命中无关帖）、E3 10/19 当日公告（只有 10/15 的 11 月公告）、E4 “OpenAI COBOL” 与 “MultiLangBench”（0 条）。
- 截图：派工单规定 D/E 组不截图，未做。

## 本组写入的文件（`docs/2610/1019/sources/`）

- 清单：`e-list.tsv`、`e-list2.tsv`
- 存档：`e-reuters.html`、`e-reuters.txt`、`e-reuters-extract.json`、`e-reuters-openai.html`、`e-reuters-openai.headers`；`e-browseruse-agents.{html,txt,headers}`；`e-copilot-changelog.{html,txt,headers}`；`e-openai-lowres.{html,txt,headers}`；`e-multilangbench-repo.{html,txt,headers}`
- 日志与热度：`e-fetch-log.jsonl`、`e-heat.json`
- 本组文档：`evidence-E.md`、`capture-log-E.md`

## 不入库与本地文件

- 入库（txt/json/md、清单、日志）：`e-reuters.txt`、`e-browseruse-agents.txt`、`e-copilot-changelog.txt`、`e-openai-lowres.txt`、`e-multilangbench-repo.txt`、`e-reuters-extract.json`（约 13 KB）、清单、日志、热度 json、两份 md。
- 只存本地（html 原件、headers）：按派工规定不入库，由 Claude 事后传网盘。单个文件均未超过 1 MB 的文本入库要求：最大 txt 为 `e-reuters.txt`（约 26 KB）。
- 个人信息：`e-reuters.html`、`e-reuters-extract.json` 已脱敏（见上）；其余 html 为公司页面，未见登录态信息。`e-multilangbench-repo.html` 中的 `copilot_*` 字样是 GitHub 功能开关名，不是泄露，已排除。
- 体积：`e-openai-lowres.html` 为 1.13 MB，html 原件本地保存、不入库，传网盘时需注意（它本身不超过 1 MB 的文本入库限制，入库的是 `e-openai-lowres.txt`）。
