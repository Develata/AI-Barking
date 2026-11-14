# 1113 期取证 C 组：抓取与操作日志

执行方：Haiku 5.5（claude-haiku-5-5）。时间为北京时间（UTC+8，本机时区）。工具：`barking fetch`（匿名 GET，桌面 Chrome UA，无 Cookie）、`barking heat`（HN Algolia 与 Reddit）、WebSearch（仅作线索，结果未作为一手依据）。未使用 opencli、curl、headless Chrome，未截图。

每次 `barking fetch` 的逐条结果也记在 `sources/c-fetch-log.jsonl`（北京时间、清单 URL、最终 URL、状态、字节、sha256）。

## 一、官方页面抓取（barking fetch）

| 时刻（北京） | 批次 | name | URL | 结果 | 备注 |
|---|---|---|---|---|---|
| 21:49:25 | 第一批 | c1-wallet | https://openai.com/index/introducing-chatgpt-wallet | ok 200 | 378,693 B，274 行 txt |
| 21:49:25 | 第一批 | c2-langchain | https://www.langchain.com/blog/series-c | ok 200 | 164,954 B，358 行 txt |
| 21:49:25 | 第一批 | c3-devin | https://devin.ai/blog/devin-teams-plan | ok 200 | 293,472 B，285 行 txt |
| 21:49:25 | 第一批 | c4-projects-docs | https://code.claude.com/docs/en/projects | failed 404 | 扫描给的路径不存在；正确路径见下 |
| 21:49:25 | 第一批 | c5-antigravity | https://antigravity.google/docs/dynamic-workflows | ok 200 | 176,012 B，796 行 txt |
| 21:49:25 | 第一批 | c6-poppy | https://sierra.ai/blog/releasing-poppy | ok 200 | 173,830 B，220 行 txt |
| 21:49:25 | 第一批 | c7-citizen | https://www.citizen.org/news/coalition-of-nonprofits-files-landmark-lawsuit-to-stop-development-of-advanced-ai-systems-at-anthropic-google-meta-openai-and-spacexsi-that-proliferate-bioweapons-and-cyberattacks/ | ok 200 | 80,083 B，321 行 txt |
| 21:49:25 | 第一批 | c8-fortune | https://fortune.com/2026/11/13/openai-disbands-robotics-team-as-its-general-models-start-running-robots/ | ok 200 | 444,628 B，269 行 txt |
| 21:50:23 | 第二批 | c4-claude-projects | https://claude.com/resources/articles/projects-redesigned | ok 200 | 565,605 B，748 行 txt |
| 21:50:23 | 第二批 | c1-reuters | https://www.reuters.com/technology/openai-launches-chatgpt-wallet-letting-ai-agents-pay-purchases-2026-11-13/ | blocked 401 | 未绕过，未存正文 |
| 21:50:23 | 第二批 | c5-workflows-to-skills | https://antigravity.google/docs/migration/workflows-to-skills/ | ok 200 | 162,553 B，650 行 txt |
| 21:50:23 | 第二批 | c6-poppy-spec | https://personalagentprotocol.org/ | ok 200 | 84,082 B，226 行 txt |
| 21:50:23 | 第二批 | c5-antigravity-blog | https://antigravity.google/blog | ok 200 | 177,785 B，417 行 txt（仅用于核对，无 Dynamic Workflows 条目） |
| 21:52:38 | 第三批 | c4-cc-changelog | https://code.claude.com/docs/en/changelog | ok 200 | 4,622,498 B，22,552 行 txt（超过 1 MB，见下） |
| 21:52:38 | 第三批 | c4-cc-whatsnew | https://code.claude.com/docs/en/whats-new | ok 200 | 294,049 B，374 行 txt（仅导航，无 Projects 条目） |
| 21:52:38 | 第三批 | c5-antigravity-releases | https://antigravity.google/releases | ok 200 | 163,256 B，170 行 txt（无版本条目） |
| 21:52:38 | 第三批 | c6-poppy-path-to-1 | https://personalagentprotocol.org/docs/path-to-1 | ok 200 | 112,139 B，329 行 txt |
| 21:52:45 | 补抓 | c4-cc-projects-docs | https://code.claude.com/docs/en/claude-projects | ok 200 | 446,833 B，918 行 txt（修正后的 Projects 文档路径） |
| 21:53:00 | 补抓 | c5-antigravity-changelog | https://antigravity.google/docs/changelog | ok 200 | 1,354,593 B，9,473 行 txt（超过 1 MB，见下） |

第一批的 `c4-projects-docs` 失败（404）为扫描给的路径错误，已由正确路径 `claude-projects` 补抓。

## 二、热度（barking heat，HN Algolia 与 Reddit）

取得时刻 2026-11-14 21:50:06（北京）；HN 搜索窗口自 2026-11-07 21:50 起。

| 查询 | 命中 | 结果摘要（HN item，分/评，北京时间） |
|---|---|---|
| ChatGPT Wallet | 2 | 50484640，13/4，11-13 23:07（官方页）；50487510，9/1，11-14 02:17（Reuters 链接） |
| LangChain Series C | 1 | 50486088，4/0，11-14 00:44 |
| Devin Teams | 0 | 无 |
| Claude Code Projects | 15 | 50492141，92/86，11-14 08:28（claude.com 官方文章）；其余为无关 Show HN |
| Antigravity Dynamic Workflows | 0 | 无 |
| Poppy Personal Agent Protocol | 0 | 无 |
| Sierra Poppy | 0 | 无 |
| Public Citizen lawsuit AI | 0 | 无 |
| OpenAI robotics team | 1 | 50487184，172/72，11-14 01:55（Fortune） |
| Reddit: OpenAI robotics team | 0 | 失败：返回登录页（非 JSON），记入 `c-heat.json` errors |

热度结果已写入 `sources/c-heat.json`（HN 9 组，条目 0 个，1 个 Reddit 请求失败）。

注：`c-heat.json` 的 `hn` 字段按查询分组保存上表各条（points、comments、time_bj、url），与上表一致。“条目 0 个”指 `items` 字段（仅由 `--item` 指定的 HN 条目生成），本组未使用 `--item`，故为 0。

## 三、WebSearch（仅线索，未作为一手依据）

| 时刻（北京） | 查询 | 结果 | 处理 |
|---|---|---|---|
| 21:5x | Just Security Public Citizen Heat Initiative Alliance for Secure AI lawsuit … Platkin | 无 Just Security 与 Platkin 相关命中；找到另一起联邦反垄断诉讼（四名消费者，2026-09-18 立案，Pacing the Frontier），与 C7 不是同一案件 | 未采用为 C7 依据，已在 evidence-C.md 说明 |
| 21:5x | Claude Code /btw /fork projects coordinator read-only share thread limit changelog November 2026 | 第三方汇总为主，无 Projects 与 /btw、/fork 的关联 | 以官方文档与 changelog 为准 |
| 21:5x | LangChain Series C $300M CapitalG headcount 435 850 employees 2027 | 只找到 2025 年 B 轮与二手数据站；无 435/850 数字 | C2 员工数记为未找到一手来源 |
| 21:5x | Google Antigravity Dynamic Workflows run-workflow launch announcement | 找到 I/O 2026 的 Antigravity 2.0 二手报道；无 Dynamic Workflows 条目 | 改用官方变更日志确认 v2.32.0，2026-11-13 |

## 四、大文件与隐私检查

- 单个文件超过 1 MB：`c-c4-cc-changelog.txt`（3.39 MB，22,552 行）与 `c-c4-cc-changelog.html`（4.62 MB）、`c-c5-antigravity-changelog.html`（1.35 MB）。`c-c5-antigravity-changelog.txt` 为 578,807 B，未超过。两份 changelog 是整站变更日志，只用来核对 Projects 与 Dynamic Workflows 的条目。**是否入库由 Claude 决定**（派工单要求超过 1 MB 的先告知）。HTML 原件按规则只存本地，不入库。
- 存档写盘前的身份检查：`.headers` 文件仅记录响应状态、最终 URL 与少数重写字段，未存抓取地点或标识；存档正文未含头像、昵称或 handle。已抽查 `c-c4-claude-projects.headers` 与 `c-c1-wallet.headers`，无抓取者信息。
- 未使用 Cookie，未登录，未接受非必要 Cookie。未提交任何表单，未发帖、点赞或关注。

## 五、未完成项与受限

- Reuters（C1 旁证）：HTTP 401，blocked，未绕过。
- WSJ（C2）：派工单未给 URL，未查。
- 官方 X 账号（C4 “11-13 22:51 UTC” 来源）：派工单未给 URL，未查。
- Platkin 诉状页（C7）：未抓取，未绕反爬。
- Reddit：`barking heat --reddit` 请求失败（登录页），未重试。
- 同一来源失败两次的，只记失败。本组无重试。

## 六、临时文件

- 清单文件存于 scratchpad `/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/273b0324-4dd3-40a6-b8cf-b3e373b66abf/scratchpad/c-manifest.tsv`，不在仓库内，不需删除。
- 仓库内本组新增文件：`sources/evidence-C.md`、`sources/capture-log-C.md`、`sources/c-*`（存档与日志）、`sources/c-heat.json`。未创建脚本，未调用 `rm`。
- 待删除文件：无。
