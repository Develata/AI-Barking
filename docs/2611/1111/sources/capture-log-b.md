# 1111 期 B 组抓取与截图日志

执行：Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（UTC+8，本机时区）。存档前缀 `b-`；机器可读日志：`b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。清单文件 `b-list1.tsv`–`b-list6.tsv`。截图用 `barking shot render`（headless Chrome，匿名，`BARKING_CHROME` 指向 Chrome 应用二进制），700 CSS px 宽、DPR 2（图 1400 px 宽），再用 `cropbox`/`stack` 裁切拼接；未用 opencli 浏览器，没有使用任何登录会话，没有 X 等社交平台页面，所以没有需要抹去的抓取者信息（已 grep `docs/2611/1111/sources/b-*` 与 `evidence-b.md` 确认无本机用户名/路径泄露）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 17:53 | 官方文 aboutamazon.com/news/aws/how-autonomous-ai-agents-built-aws-emulator；Vogels 博客；github.com/aws/aws-emulator；raw README、LICENSE、CONTRIBUTING（根目录）；GitHub API repo、contents | `barking fetch`（`b-list1.tsv`） | 7 ok；`gh-contributing`（根目录 CONTRIBUTING.md）404 failed（真实文件在 `.github/CONTRIBUTING.md`，见下） |
| 17:54 | GitHub API `git/trees/main?recursive=1` | curl（`--max-time 60`） | 200，但响应 16.6 MB 且 `truncated: true`（45,451 条）；**未入库**，移到草稿目录，改用非递归逐层取（见下） |
| 17:55 | GitHub API 非递归 trees：根、`issues`、`things-that-do-not-work`、`things-that-now-work`、`services`、`vendor` | curl | 200，均未截断；统计存 `b-gh-issues-summary.json` |
| 17:55 | GitHub API commits `per_page=1`（取最后一页页码）、`per_page=3`、最后一页（第 29684 页 per_page=1） | curl | 200；总数 29,684；首条 `b-gh-commit-first.json` |
| 17:56 | GitHub API commits `per_page=100` 第 1、50、100、150、200、250、296 页 | curl（Python 一次性脚本，未存脚本） | 200；700 条作者统计存 `b-gh-commits-sample.json` |
| 17:56 | raw `.github/CONTRIBUTING.md`、`AGENTS.md`、`vendor/README.md`、`vendor/NOTICE`、`.emulator/license-header` | curl / `barking fetch`（`b-list2.tsv`） | 200 |
| 17:56 | releases API、releases 页、评测套件仓库页、文档 what-is/security/telemetry/check-project、GitHub issues 页、vendor LICENSE、CONTRIBUTING（`.github/`）、AGENTS.md | `barking fetch`（`b-list2.tsv`） | 11 ok |
| 17:57 | 文档 fidelity.html、versioning.html、setting-up.html | `barking fetch`（`b-list3.tsv`） | 3 ok |
| 17:57 | v1.0.322 `scores.json.gz`（1,140,628 字节） | curl（`--max-time 120`） | 200；**只存在草稿目录，未放入仓库**（1.1 MB，超过 1 MB 的文件按规则先告知）；SHA-256 eca6d69bfa6e801d224f88dd47907ef1573002161e57d1b092de655db77e08cd；汇总存 `b-fidelity-scores-v1.0.322-summary.json`（41 KB） |
| 17:58 | `barking heat`：条目 50456631、50456740、50456319；HN 搜索 “AWS Emulator”；Reddit 搜索两条 | `barking heat`（前缀 `b`，`--since` 2026-11-11 14:53 北京） | HN 命中 5 条；**Reddit 2 条失败**：响应是 old.reddit.com 登录页，不登录、不重试；记入 `b-heat.json` errors；条目 JSON 存 `b-hn-50456631.json`（含 208 条嵌套评论，无评论得分）、`b-hn-50456740.json`、`b-hn-50456319.json` |
| 18:00 | Exa 搜索（`web_search_exa`）：英文与中文媒体报道、InfoWorld/GeekWire/SiliconANGLE/AWS 新闻博客等 | Exa MCP（WebSearch 返回无关结果，改用 Exa） | 返回摘录；仅用于找 URL 与发布时刻，引文以 `barking fetch` 存档的原页面为准（GeekWire 例外，见下） |
| 18:02 | SiliconANGLE、GeekWire、AWS 新闻博客、What’s New、ITmedia、至顶网、Unite.AI 中文版、agenticfeed、frontword、HN 帖页 | `barking fetch`（`b-list4.tsv`） | 8 ok；**GeekWire 403（Cloudflare 质询，blocked）**，不绕过，未存正文；**agenticfeed.io 协议错误（failed）**，同一来源不再试 |
| 18:03 | 评测套件 README、TAXONOMY.md、评测套件 commits API | `barking fetch`（`b-list5.tsv`） | 3 ok；评测套件仓库共 2 次提交 |
| 18:07 | GitHub API commits `?path=vendor|.github|AGENTS.md&per_page=100`；issues `state=open&per_page=100` | curl | 200；路径提交摘要存 `b-gh-path-commits.json`；开放 issue 55 条（全部外部用户），原文只在草稿目录 |
| 18:10 | InfoWorld 文章 | `barking fetch`（`b-list6.tsv`） | ok |

另：`b-gh-contributing.*`（根目录 CONTRIBUTING.md 的 404 残留，`.bin`/`.headers`）、`b-news-agenticfeed.headers`（失败残留）、`b-news-geekwire.html/.headers`（Cloudflare 质询页 6 KB）无用，列入回报的待删除清单。

## 截图（`barking shot`，日志 `b-shots-log.jsonl`；2 倍像素比；所有取自匿名 headless Chrome）

时间 17:59–18:09。中间渲染图放在系统临时目录（`barking-shot/26111111-b/`），不入库。裁切边界落在文字行之间；拼接（`stack`）的段间黑色或深色间隔条是工具生成的（间隔 24 px，背景取页面边缘色，个别页面取到近黑色），不属于原页面。

| 图 | 来源页面 | 方式 | 备注 |
|---|---|---|---|
| `13-aws-official-claim.png` | 官方文（`official2`，700 px 宽移动版式，页高 14,800 px） | `stack` 4 段 | 浅色；含作者头像（亚马逊新闻站公开头像） |
| `14-aws-kiro-prompt.png`、`15-aws-three-teams-tests.png`、`16-aws-merge-gate.png` | 同上 | `cropbox` | 15 含官方架构图与图注 |
| `17-aws-python-interpreter.png` | 同上 | `stack` 2 段 | 中间省略的是一张配图 |
| `18-vogels-spec-tests.png`、`19-vogels-engineer-job.png` | Vogels 博客（`vogels`） | `cropbox` | 浅色 |
| `20-gh-repo-files.png`、`21-gh-readme-production.png`、`22-gh-readme-telemetry.png`、`23-gh-readme-license-pr.png`、`24-gh-readme-telemetry.png` | github.com/aws/aws-emulator（`ghrepo2`，页高 15,600 px，匿名） | `cropbox`/`stack` | **22 作废**：第 2、3 段交界处切断一行，由 24 取代。GitHub 浅色主题 |
| `37-evalsuite-readme.png` | github.com/aws/aws-emulator-evaluation-suite（`evalsuite`） | `cropbox` | |
| `38-gh-things-do-not-work.png` | github.com/aws/aws-emulator/tree/main/issues/things-that-do-not-work（`dnw`） | `cropbox` | GitHub 目录截断提示；右侧列是加载占位条（静态渲染未取到提交信息） |
| `39-docs-limitations.png` | docs.aws.amazon.com/emulator/latest/userguide/what-is-service.html（`docwhat2`） | `stack` 2 段 | 页面浅色（拼接间隔条深色是工具生成的） |
| `40-hn-50456631-title.png` | news.ycombinator.com/item?id=50456631（`hn`） | `cropbox` | 18:06 取，848 分 / 235 评（`barking heat` 17:58 为 847 分） |
| `41-aws-fidelity-doc.png` | docs.aws.amazon.com/.../fidelity.html（`fid`） | `cropbox` | **作废**：底部切断一行，由 44 取代 |
| `42-news-siliconangle.png` | siliconangle.com 文章（`sa`） | `cropbox` | |
| `43-news-zhiding.png` | zhiding.cn 文章（`zd`） | `stack` 2 段（中文 OCR 定位，`--lang chi_sim`） | 页面浅色 |
| `44-aws-fidelity-doc.png` | 文档 fidelity.html（`fid`） | `cropbox` | |

没有截图的候选：服务覆盖段（官方文中 438 的句子已含在图 16；Vogels 的 400+/200+ 在图 18），releases 页（静态渲染只见修复清单，分数在附件里，见 evidence-b.md B4-3）。渲染过 releases 页（`rel`）与官方文末尾检查，但未保存裁图。

## 备注

- `barking shot lines` 在 macOS 上能用（tesseract 5，已装 `chi_sim`）。
- `barking heat` 同一前缀只写一次，本组只调用一次，无需第二前缀。
- 所有 URL 已去掉 utm 等查询参数；`aboutamazon.com` 页内带 `utm_medium=trending_module` 的“热门新闻”链接未引用。
