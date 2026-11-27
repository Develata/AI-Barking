# 1126 期 A 组取证抓取日志

- **期次目录**：`docs/2611/1126/`
- **抓取日期**：2026-11-27（北京时间 CST / UTC+8）
- **工具环境**：`barking`（`tools/barking/target/release/barking`）、`opencli 2.0.3`（Chrome 扩展已连接，`--session a-shot`）、`api.fxtwitter.com`

---

## 一、社区热度采样（`barking heat` 与浏览器补充）

| 时间（CST） | 工具 | 目标与参数 | 结果 | 本地存档 |
|---|---|---|---|---|
| 2026-11-27 21:12:52 | `barking heat` | `--hn "Claude sabotaging" --reddit "Claude sabotage" --hn-item 50664325` | HN 成功：主帖 `50664325`（148 分 / 97 评，`2026-11-27 16:28:08 CST`），搜索另命中 `50644109`（`@skirano`，2 分 / 0 评，`2026-11-26 00:24:37 CST`）。Reddit 搜索**失败**：跳至 `old.reddit.com/login/?reason=lor2` 返回 HTML（343,085 字节），JSON 解析失败。 | `a-heat.json`、`a-hn-50664325.json` |
| 2026-11-27（21:20–21:25 区间） | `opencli browser`（Chrome 公开页面提取） | `https://www.reddit.com/r/ClaudeAI/comments/1y65dvn/`、`https://www.reddit.com/r/ClaudeCode/comments/1y6s6ze/` | 成功提取脱敏公开帖与评论 JSON（`1y65dvn`：133 分 / 60 评，`upvote_ratio` 0.76；`1y6s6ze`：49 分 / 20 评），已自查不含抓取者账号字段。 | `a-reddit-sweeney-1y65dvn.json`、`a-reddit-claudecode-1y6s6ze.json` |

---

## 二、网页与 PDF 批量抓取（`barking fetch` + `opencli` 兜底）

共执行 `barking fetch` 36 条 URL（详见 `a-fetch-log.jsonl`），其中 **31 条成功（`ok`）**、**5 条被拦截/需 JS 渲染（`blocked`）**：

### 2.1 `barking fetch` 成功清单（31 条）

1. `21:12:51` `anthropic-fable-mythos` — `https://www.anthropic.com/claude-fable-and-mythos-5-2`（HTTP 200，411,992 B → `.html`/`.txt`/`.headers`）
2. `21:12:51` `hn-50664325` — `https://news.ycombinator.com/item?id=50664325`（HTTP 200，152,272 B）
3. `21:12:52` `aa-models` — `https://artificialanalysis.ai/leaderboards/models`（HTTP 200，1,464,437 B）
4. `21:12:52` `aa-methodology` — `https://artificialanalysis.ai/methodology/intelligence-benchmarking`（HTTP 200，1,045,621 B）
5. `21:13:42` `support-covered-models` — `https://support.claude.com/en/articles/15425695-covered-models`（HTTP 200，224,263 B）
6. `21:13:42` `preserved-thinking` — `https://platform.claude.com/docs/en/build-with-claude/preserved-thinking`（HTTP 200，633,530 B）
7. `21:13:43` `aa-fable-5-2` — `https://artificialanalysis.ai/models/claude-fable-5-2`（HTTP 200，1,206,984 B）
8. `21:13:43` `distillation-defenses` — `https://www.anthropic.com/research/distillation-defenses` → `https://www.anthropic.com/news/distillation-defenses`（HTTP 200，178,795 B）
9. `21:13:45` `threat-intel-sep26` — `https://www.anthropic.com/threat-intelligence-report-september-2026`（HTTP 200，1,349,951 B）
10. `21:13:51` `system-card-doc` — `https://www.anthropic.com/document/claude-fable-5-2-mythos-5-2-system-card` → `https://assets.claude.com/27d60918d218125e7504bc7354c4b4626ec56b11.pdf`（HTTP 200，PDF 23,887,562 B，按 180 秒限时重取成功，264 页转 txt 8,559 行）
11. `21:14:25` `support-switched-models` — `https://support.claude.com/en/articles/15363606-why-claude-switched-models-in-your-conversation-with-fable-models`（HTTP 200，235,014 B）
12. `21:14:25` `support-data-retention` — `https://support.claude.com/en/articles/15425996-data-retention-practices-for-covered-models`（HTTP 200，223,999 B）
13. `21:14:26` `platform-stop-reasons` — `https://platform.claude.com/docs/en/build-with-claude/handling-stop-reasons`（HTTP 200，1,996,946 B）
14. `21:14:58` `platform-fallback-credit` — `https://platform.claude.com/docs/en/build-with-claude/fallback-credit`（HTTP 200，857,340 B）
15. `21:14:58` `platform-refusals-fallback` — `https://platform.claude.com/docs/en/build-with-claude/refusals-and-fallback`（HTTP 200，1,143,885 B）
16. `21:14:58` `support-preserved-thinking` — `https://support.claude.com/en/articles/16761192-preserved-thinking-changing-how-the-messages-api-handles-thinking-blocks-to-protect-against-distillation`（HTTP 200，231,966 B）
17. `21:17:58` `support-how-to-get-support` — `https://support.claude.com/en/articles/9015913-how-to-get-support`（HTTP 200，241,175 B）
18. `21:17:58` `aa-fable-providers` — `https://artificialanalysis.ai/models/claude-fable-5-2/providers`（HTTP 200，616,818 B）
19. `21:17:58` `anthropic-redeploying-fable-5` — `https://www.anthropic.com/news/redeploying-fable-5`（HTTP 200，196,825 B）
20. `21:18:49` `aa-creators-anthropic` — `https://artificialanalysis.ai/models/creators/anthropic`（HTTP 200，862,867 B）
21. `21:18:50` `cn-ithome` — `https://www.ithome.com/1/032/822.htm`（HTTP 200，25,194 B）
22. `21:18:51` `aa-providers-anthropic` — `https://artificialanalysis.ai/providers/anthropic`（HTTP 200，1,369,333 B）
23. `21:19:08` `platform-sysprompt-fable-5-2` — `https://platform.claude.com/docs/en/release-notes/system-prompts/claude-fable-5-2`（HTTP 200，472,977 B）
24. `21:19:09` `support-appeals` — `https://support.claude.com/en/articles/8241253-safeguards-warnings-and-appeals`（HTTP 200，207,860 B）
25. `21:19:16` `fable-5-system-card` — `https://assets.claude.com/d00db56fa754a1b115b6dd7cb2e3c342ee809620.pdf`（HTTP 200，PDF 27,001,265 B，转 txt 10,609 行）
26. `21:20:16` `en-wccftech` — `https://wccftech.com/epic-games-ceo-tim-sweeney-calls-claude-garbage/`（HTTP 200，148,707 B）
27. `21:23:43` `cn-qq` — `https://news.qq.com/rain/a/20261126A0AYVO00`（HTTP 200，231,553 B）
28. `21:23:44` `cn-blocktempo` — `https://www.blocktempo.com/claude-accused-sabotaging-ml-research-anthropic-documents-frontier-llm-limits/`（HTTP 200，166,354 B）
29. `21:23:44` `cn-technews` — `https://technews.tw/2026/11/26/epic-ceo-slams-claude-anti-distillation-mechanism/`（HTTP 200，136,380 B）
30. `21:23:45` `en-decrypt-jun11` — `https://decrypt.co/370831/anthropic-apologizes-claude-fable-5-secret-censorship`（HTTP 200，1,103,741 B）
31. `21:25:04` `aa-methodology-root` — `https://artificialanalysis.ai/methodology`（HTTP 200，321,760 B）

### 2.2 `barking fetch` 失败/拦截条目及后续处置（5 条）

| 时间（CST） | 条目 | URL | `barking fetch` 拦截原因 | 后续处置与结果 |
|---|---|---|---|---|
| 2026-11-27 21:18:53 | `cn-51cto` | `https://www.51cto.com/article/862320.html` | 需 JS 渲染（HTML 987 B，正文 0 字） | 改用 `opencli browser` 打开并提取正文存为 `a-cn-51cto.txt`（376 行） |
| 2026-11-27 21:19:08 | `en-tweaktown` | `https://www.tweaktown.com/news/114891/...` | HTTP 403 Cloudflare 质询（`cf-mitigated: challenge`） | 遵守规范不绕过质询，记失败跳过，改用同事件 Wccftech |
| 2026-11-27 21:19:10 | `cn-36kr` | `https://36kr.com/p/3848134330356999` | 需 JS 渲染（正文 54 字） | 用 `opencli browser` 检查确认为 2026-06-11 初版 Fable 5 旧文，非 11 月事件，未作当期报道正文存档 |
| 2026-11-27 21:20:16 | `cn-techflow` | `https://www.techflowpost.com/en-US/newsletter/147029` | HTTP 403 Cloudflare 质询（`cf-mitigated: challenge`） | 遵守规范不绕过质询，记失败跳过 |
| 2026-11-27 21:24:22 | `cn-10jqka` | `https://m.10jqka.com.cn/20261126/c682245300.shtml` | 需 JS 渲染（正文 41 字） | 改用 `opencli browser` 提取全文存为 `a-cn-10jqka.txt`（235 行，系转载 IT之家） |

---

## 三、X 平台接口取证（`api.fxtwitter.com`）

于 `2026-11-27 21:20–21:25 CST` 拉取 13 条公开推文及作者资料（写入 `a-x-tweets.json`，246 行，已校验无抓取者信息）：

- `@RealAstropulse`：`2126119408997466389`（主帖）、`2126119412470022460`（thread 拆解）、`2126192618766049313`（后续引流贴）
- `@skirano`：`2125371115979641188`（`DoNotSabotage` 原帖）
- `@edwinarbus`（Anthropic 员工）：`2125757350040711397`、`2125799392481571135`（两次公开回复 `@skirano`）
- `@jacobeverly`：`2125007879220658193`
- `@TimSweeneyEpic`：`2125513144353411115`（首帖）、`2125661964617871830`（回复 Wccftech 澄清）、`2125699782979326128`（非良基集合论与 Verse 补充）
- `@plotarmordev`：`2125498274698457507`（105 次防蒸馏标记组织封号帖）
- `@hmbakhsh`：`2125383877908717643`（指出 6 月 vs 11 月系统卡差异）
- `@wccftech`：`2125619384035192960`

---

## 四、截图取证与裁切拼接（`barking shot` & `opencli`）

所有终稿截图均在 `docs/2611/1126/images/`（编号 `01`–`13`，全部宽 ≤ 1400 px），并逐张用 `read_file` 目视确认清晰、文字完整、无浏览器扩展浮动图标、无登录态头像/账号/回复框泄露：

1. `01-anthropic-safeguards.png`（1160×1761）：`opencli` 截取 Anthropic 发布文 `AI research and development` 与 `Anti-distillation` 段落（2x DPR）后经 `barking shot stack` 纵向拼接。
2. `02-platform-refusal-fallback.png`（1400×1954）：`barking shot render` + `stack`，截取 `platform.claude.com/docs/en/build-with-claude/refusals-and-fallback` 中拒绝类别表（`"frontier_llm"`、`"reasoning_extraction"`）及无输出计费规则。
3. `03-astropulse-post.png`（1310×1572）：`opencli` 截取 `@RealAstropulse` 主帖卡片后 `cropbox` 裁除底部 `Relevant v`、回复框头像与右侧浮动按钮。
4. `04-astropulse-breakdown.png`（1196×1691）：`opencli` 截取 `@RealAstropulse` 拆解 thread `2126119412470022460` 上下两段后 `stack` 拼接并裁除侧栏及回复框。
5. `05-sweeney-posts-v2.png`（1305×886）：`opencli` 截取 Tim Sweeney 首帖与澄清帖后 `stack` 拼接（替代首版未切净 `Relevant v` 的 `05-sweeney-posts.png`）。
6. `06-aa-default-fallback-v2.png`（1400×2210）：`barking shot render`（`1000 CSS px, DPR 1.4`）+ `cropbox`，完整展示榜单头部与前 7 行含 `Max, Default Fallback` 条目，底部精准切在 `Claude Sonnet 5.5` 行底线（替代底行被半裁的 `06-aa-default-fallback.png`）。
7. `07-cn-blocktempo-coverage.png`（1400×946）：`barking shot render`（`700 CSS px, DPR 2`）+ `stack`，截取动区动趋 11-27 17:02 报道标题、署名及“Anthropic 尚未回應”导语（此前试截 `51cto` 固定 1200 px 宽在 700 px 视口溢出右裁，`m.51cto.com` 无法访问，故改取响应式且包含关键对比点的动区报道，51CTO 夸大全文另存于 `a-cn-51cto.txt`）。
8. `08-hn-50664325.png`（1400×1545）：`barking shot render`（`700 CSS px, DPR 2`）+ `cropbox`，截取 HN `50664325` 主帖与首批引用 6 月系统卡的讨论。
9. `09-edwinarbus-reply-v2.png`（1305×1116）：`opencli` 截取 Edwin Arbus 两条公开回复后 `stack` 拼接，保留两条各自完整互动栏（替代首版漏截首条底栏且顶留细线的 `09-edwinarbus-reply.png`）。
10. `10-support-switched-models.png`（1400×2804）：`barking shot render`（`700 CSS px, DPR 2`）+ `stack`，截取 Help Center 自动切换条件、提示行为及关闭开关路径。
11. `11-system-card-1-5.png`（1224×918）：`barking shot pdf`（DPI 144）+ `stack`，提取 `a-system-card-doc.pdf` 第 12–13 页 `1.5 Safeguards` 全文。
12. `12-skirano-donotsabotage.png`（1305×760）：`opencli` 截取 `@skirano` 原帖卡片后 `cropbox` 裁除底部交互区与右侧浮动按钮。
13. `13-cn-ithome-sweeney.png`（1400×1670）：`barking shot render`（`700 CSS px, DPR 2`）+ `cropbox`，截取 IT之家 11-26 15:21 报道标题、导语及正文（含机器翻译错误示例）。

### 本组废弃截图（未调用 `rm`，留待主执行方清理）

- `docs/2611/1126/images/05-sweeney-posts.png`（已被 `05-sweeney-posts-v2.png` 替代）
- `docs/2611/1126/images/06-aa-default-fallback.png`（已被 `06-aa-default-fallback-v2.png` 替代）
- `docs/2611/1126/images/09-edwinarbus-reply.png`（已被 `09-edwinarbus-reply-v2.png` 替代）
