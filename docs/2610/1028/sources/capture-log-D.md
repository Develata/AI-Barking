# 1028 期 取证 D 组：抓取与热度记录（capture-log-D）

时间为北京时间（UTC+8，2026-10-29）。工具：`barking fetch`（网页/文本）、`barking heat`（HN/Reddit）、curl（GitHub API、Stack Exchange API）。截图：不需要，未做。

## 网页存档（barking fetch，清单见 d-fetch-log.jsonl）

| 名称 | URL | 工具 | 结果 | 大小（字节）| 北京时间 |
|---|---|---|---|---|---|
| d1-copilot-changelog | https://github.blog/changelog/2026-10-28-github-intelligence-puts-your-code-context-and-history-to-work/ | barking fetch | 200 ok | 117883 | 19:40 |
| d2-openrouter-routing | https://openrouter.ai/blog/announcements/how-we-rebuilt-provider-routing/ | barking fetch | 200 ok | 75374 | 19:40 |
| d3-particle-1 | https://particle.ai/blog/introducing-particle-1 | barking fetch | 200 ok | 253050 | 19:40 |
| d3-vercel-particle | https://vercel.com/changelog/particle-1-is-now-available-on-ai-gateway | barking fetch | 200 ok | 512866 | 19:40 |
| d4-octoverse | https://github.blog/news-insights/octoverse/octoverse-commits-pushes-reach-new-highs-in-2026-as-developers-build-with-agents/ | barking fetch | 200 ok | 271047 | 19:40 |
| d5-google-connected-apps | https://blog.google/innovation-and-ai/technology/developers-tools/connected-apps-one-place/ | barking fetch | 200 ok | 989185 | 19:40 |
| d7-nemotron-transcribe | https://huggingface.co/blog/nvidia/nemotron-transcribe-granary | barking fetch | 200 ok | 270212 | 19:40 |
| d6-microzig-coc-1103 | https://raw.githubusercontent.com/ZigEmbeddedGroup/microzig/ec880d3005645d96e7fcb323b140e5f46437e383/CODE_OF_CONDUCT.md | barking fetch | 200 ok | 3741 | 19:41 |
| d6-microzig-coc-1104 | https://raw.githubusercontent.com/ZigEmbeddedGroup/microzig/ebe5c77e4e8e08e5d288ca04eff9c3f6ab0478d9/CODE_OF_CONDUCT.md | barking fetch | 200 ok | 3670 | 19:41 |
| d6-microzig-coc-main | https://github.com/ZigEmbeddedGroup/microzig/blob/main/CODE_OF_CONDUCT.md | barking fetch | 200 ok | 204750 | 19:41 |
| d8-se-dataexplorer | https://data.stackexchange.com/stackoverflow/query/1926661 | barking fetch | 403 blocked（Cloudflare 质询，cf-mitigated: challenge）；无 .txt | 5782 | 19:44 |

备注：d6-main 为 GitHub blob 页面（HTML），主要用于确认 main 分支仍含同一条款（“This policy takes effect October 28th, 2026.”）。d8 被拦截后未换其他手段绕过。

## Stack Exchange API 计数（curl，filter=total）

| 文件 | 查询 URL（站点与窗口） | 结果 | 北京时间（查询） |
|---|---|---|---|
| d-se-count-2025-09.json | https://api.stackexchange.com/2.3/questions?site=stackoverflow&fromdate=1756684800&todate=1759276800&pagesize=1&filter=total | 5072 | 约 19:40 |
| d-se-count-2026-06.json | …fromdate=1780272000&todate=1782864000… | 1701 | 约 19:40 |
| d-se-count-2026-07.json | …fromdate=1782864000&todate=1785542400… | 1175 | 约 19:5x（重跑，见下） |
| d-se-count-2026-08.json | …fromdate=1785542400&todate=1788220800… | 1019 | 约 19:5x（重跑，见下） |
| d-se-count-2026-09.json | …fromdate=1788220800&todate=1790812800… | 969 | 约 19:40 |
| d-se-count-2026-10.json | …fromdate=1790812800&todate=1793274061… （1793274061 = 2026-10-29 11:41:01 UTC） | 963 | 19:41 |

**更正记录**：首次查询 2026-07 与 2026-08 时，我把窗口起止写错（用了 1783814400–1786492800 与 1786492800–1788220800，实为 07-12 至 08-11 与 08-11 至 09-01），得到 1075 与 677。已用 `date -u -d @<ts>` 核对时间戳后重跑，并以上表的正确结果覆盖了同名文件。1075 与 677 不得引用。

## HN / Reddit 热度（barking heat）

| 文件 | 查询 | 结果 | 说明 |
|---|---|---|---|
| d-heat-run1.json | 第一次运行：`--hn "GitHub Copilot Memory Dreaming"` 等 6 组 + `--item 50282865 --item 50282454` | 写入 HN 结果：Connected Apps 与 Nemotron 命中；Copilot、OpenRouter、Particle 等无关键命中 | **已被取代，待删除** |
| d-heat-run2.json | `--hn "Copilot memory"` 等 7 组 | 与 run3 重复的部分结果 | **已被取代，待删除** |
| d-heat.json | `--hn "OpenRouter" "Copilot Memory" "Copilot Dreaming" "GitHub Copilot" "Connected Apps Platform" "Particle-1" "Octoverse" "Nemotron 3.5"` | 有效结果：Particle-1 16/6；Octoverse 1/0 与 2/0；Nemotron 124/21；OpenRouter 10 条无关；Copilot Memory／Dreaming 0 条；Connected Apps Platform 2 条无关 | 这是本组采用的 HN 热度文件（最后一次成功写入的完整 HN 结果） |
| d-heat-reddit-run.json | `--hn "rebuilt provider routing"`、`--hn "Octoverse"` 与 8 条 `--reddit` | HN 仅 Octoverse 2 条；Reddit 8 条全部失败 | Reddit 失败记录保留作证据 |
| d-hn-50282865.json | HN item 50282865（`--item`，经 heat 工具） | 51 分；created_at 2026-10-29T00:30:37Z（北京 08:30:37） | 该文件由第一次运行写入，保留 |
| d-hn-50282454.json | HN item 50282454 | 519 分；created_at 2026-10-28T23:35:02Z（北京 10-29 07:35:02） | 同上 |

Reddit 失败详情：8 条查询（Particle-1、Nemotron 3.5 Transcribe、MicroZig、Copilot memory dreaming、OpenRouter routing、Stack Overflow fewer than 1000 questions、Connected Apps Platform、Octoverse agents pull requests）均请求 old.reddit.com 的 search.json，返回登录页（reason=lor2）而非 JSON。未登录，未绕过。

## GitHub API（curl）

| 文件 | URL | 结果 | 北京时间 |
|---|---|---|---|
| d-microzig-coc-commits.json | https://api.github.com/repos/ZigEmbeddedGroup/microzig/commits?path=CODE_OF_CONDUCT.md&per_page=10 | 提交 ebe5c77e（Update CODE_OF_CONDUCT.md (#1104)，2026-10-29T00:37:14Z = 08:37:14）与 ec880d30（Code of Conduct including LLM Policy (#1103)，2026-10-29T00:13:46Z = 08:13:46） | 19:4x |

## 待删除文件（Claude 收尾时一次 rm）

- docs/2610/1028/sources/d-heat-run1.json
- docs/2610/1028/sources/d-heat-run2.json
- docs/2610/1028/sources/d-d8-se-dataexplorer.html 与 d-d8-se-dataexplorer.headers（Cloudflare 质询页，无正文；失败记录已在 d-fetch-log.jsonl 中，若需保留可不删）

## 本组新增文件

- docs/2610/1028/sources/evidence-D.md
- docs/2610/1028/sources/capture-log-D.md
- docs/2610/1028/sources/d-d1-copilot-changelog.{html,txt,headers}
- docs/2610/1028/sources/d-d2-openrouter-routing.{html,txt,headers}
- docs/2610/1028/sources/d-d3-particle-1.{html,txt,headers}
- docs/2610/1028/sources/d-d3-vercel-particle.{html,txt,headers}
- docs/2610/1028/sources/d-d4-octoverse.{html,txt,headers}
- docs/2610/1028/sources/d-d5-google-connected-apps.{html,txt,headers}（html 约 0.99 MB，未超过 1 MB）
- docs/2610/1028/sources/d-d7-nemotron-transcribe.{html,txt,headers}
- docs/2610/1028/sources/d-d6-microzig-coc-1103.{txt,headers}、d-d6-microzig-coc-1104.{txt,headers}、d-d6-microzig-coc-main.{html,txt,headers}
- docs/2610/1028/sources/d-d8-se-dataexplorer.{html,headers}（blocked）
- docs/2610/1028/sources/d-se-count-2025-09.json、d-se-count-2026-06.json、d-se-count-2026-07.json、d-se-count-2026-08.json、d-se-count-2026-09.json、d-se-count-2026-10.json
- docs/2610/1028/sources/d-microzig-coc-commits.json
- docs/2610/1028/sources/d-hn-50282865.json、d-hn-50282454.json
- docs/2610/1028/sources/d-heat.json、d-heat-reddit-run.json
- docs/2610/1028/sources/d-fetch-log.jsonl

## 环境异常（未处理，供 Claude 知悉）

- 运行 `cargo run ... barking heat` 时编译失败：`tools/barking/src/card.rs` 的 `include_str!("../../../templates/cards/event.html")` 找不到文件。该文件在工作区中处于别人未完成的改动状态（git status 显示 `M tools/barking/src/card.rs`），与本组无关。本组改用既有的 `tools/barking/target/release/barking.exe`（2026-10-28 09:28 构建）完成 fetch 之后的热度与存档。我没有修改 card.rs 或 templates。
- 第一次热度运行后 `d-heat.json` 已存在，`barking heat` 拒绝覆盖，因此多次改名保留（见上表）。
- `barking fetch` 的 `.txt`：d-d6-* 为纯文本源（raw），无 .html，属正常。
