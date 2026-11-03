# 1102 期取证 C 组（速览一）

执行方：claude-haiku-5-5（Haiku 5.5）。交付日期：2026-11-03（北京时间）。
范围：派工单 C1–C4 与“扫描说法勘误”。不截图（派工单要求）。不写正文、不下结论。

级别记法：L1 官方；L4 权威媒体（据报道，未回溯到法院卷宗或官方声明）；L5 聚合或检索摘要（仅线索，未核原文）。

时间记法：下表“官方未给时刻”指页面只有日期。北京时间 = 美东时间 + 13 小时（本机 EST）。

---

## 一、条目证据

### C1 Gemini 2.5 Pro / Flash 自动改路由

一句事实（照原文措辞）：Google 称，自 2026 年 12 月 1 日起，发往 gemini-2.5-pro 的 API 请求将自动路由到 gemini-3.8-flash，并按 Gemini 3.8 Flash 费率计费；gemini-2.5-flash 同样自动路由到 gemini-3.5-flash-lite。

| 项 | 内容 |
|---|---|
| 一手来源 | https://ai.google.dev/gemini-api/docs/changelog 跳转为 https://aistudio.google.com/docs/changelog（opencli 渲染，见 capture-log-c.md）；https://ai.google.dev/gemini-api/docs/deprecations 跳转为 https://aistudio.google.com/docs/deprecations |
| 存档 | `sources/c-gemini-changelog-aistudio.txt`（47,735 字节，正文渲染）；`sources/c-gemini-deprecations-aistudio.txt`（4,889 字节）；`sources/c-c1-gemini-changelog.html` 与 `c-c1-gemini-deprecations.html` 为 curl 原件，正文过短（JS 壳，已记 blocked） |
| 发布时刻 | 2026-11-02（官方未给时刻）；生效 2026-12-01（官方未给时刻与时区） |
| 级别 | L1 |
| 热度 | `barking heat`：HN 搜索“Gemini 2.5 Pro deprecation” 0 命中（时间窗自 10-27 起）；未找到社区热帖 |

原文摘句（逐字）：

- 2.5 Pro（changelog，11-02 条目）：“Effective December 1, 2026, all API requests to gemini-2.5-pro will automatically route to gemini-3.8-flash and be billed at Gemini 3.8 Flash rates.”
- 2.5 Flash（changelog，11-02 条目）：“Effective December 1, 2026, all API requests to gemini-2.5-flash will automatically route to gemini-3.5-flash-lite and be billed at Gemini 3.5 Flash-Lite rates.”
- AI Studio 不可选（deprecations）：“gemini-2.5-pro is deprecated and is no longer available to select in Google AI Studio.”
- 限定 API 访问（deprecations）：“Access through the API is limited to existing users who have actively used it in the past.”
- 2.5 Flash 同条限定（deprecations）：“Access to the 2.5 models is limited to users who have actively used them in the past. gemini-2.5-flash-lite is not deprecated and remains available through the API until further notice.”
- 不兼容项（changelog，11-02 条目，2.5 Pro 一条）：“Breaking API changes include function calling and thought_signature validation, thinking_level replacing thinking_budget, and removed sampling parameters and prefilled model turns.”
- 时间线冲突（changelog，2026-09-18 条目）：“These models are not deprecated and will continue to be served until further notice through the API.” 此条早于 11-02，属于时间线冲突，以 11-02 条目与 deprecations 页为准。

截图：无（C 组不截图）。

条件 / 口径 / 时区：计费口径指“Gemini 3.8 Flash rates”与“Gemini 3.5 Flash-Lite rates”，具体单价未取（未访问 pricing 页）。Vertex：两页均未提及 Vertex，故“仅限 Gemini API”的说法在一手页面中只能理解为“页面只写 API”，Vertex 口径未找到一手来源。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 / 口径 / 时区 | 状态 |
|---|---|---|---|---|---|---|
| 12 月 1 日起 2.5 Pro 自动转 3.8 Flash，按 Flash 计价 | L1 | aistudio.google.com/docs/changelog；…/docs/deprecations | “Effective December 1, 2026, all API requests to gemini-2.5-pro will automatically route to gemini-3.8-flash and be billed at Gemini 3.8 Flash rates.” | 无 | 官方未给生效时刻与时区；计价“Gemini 3.8 Flash rates”，单价未取 | 已找到 |
| 2.5 Flash 转 3.5 Flash-Lite | L1 | 同上 | “Effective December 1, 2026, all API requests to gemini-2.5-flash will automatically route to gemini-3.5-flash-lite …” | 无 | 同上 | 已找到 |
| AI Studio 已不可选 | L1 | deprecations 页 | “no longer available to select in Google AI Studio” | 无 | — | 已找到 |
| 部分参数不兼容 | L1 | changelog 11-02 条目 | “Breaking API changes include function calling and thought_signature validation, thinking_level replacing thinking_budget, and removed sampling parameters and prefilled model turns.” | 无 | 迁移指南为 3.8 Flash 与 3.5 Flash-Lite；此句为 2.5 Pro 条目原文 | 已找到 |
| 仅限 Gemini API（Vertex 同样？） | L1 | 同上 | 两页仅写“through the API”，未出现 Vertex | 无 | Vertex 口径未找到 | 未找到一手来源（Vertex 部分） |

### C2 纽约时报等起诉 Brave

一句事实（据路透社报道，保留限定词）：据路透社报道，News Corp（《华尔街日报》与《纽约邮报》母公司）已对 Brave Software 提起反诉，指控其以“covert scraping”方式抓取并转售版权文章给 AI 公司。

核心结论：找不到 NYT 起诉 Brave 的一手或可靠报道。检索与存档的所有相关材料中，原告为 News Corp，不是 NYT。

| 项 | 内容 |
|---|---|
| 一手来源 | 未找到一手来源（NYT 声明、法院卷宗均未找到）。替代的 L4 来源：Reuters 转载于 The Star，https://www.thestar.com.my/tech/tech-news/2026/07/22/news-corp-countersues-brave-for-allegedly-039scraping039-articles-for-ai- ；Semafor，https://semafor.com/article/07/21/2026/newscorp-accuses-search-engine-brave-of-ai-copyright-infringement ；Reuters 2025 年报道转载于 The Star，https://www.thestar.com.my/tech/tech-news/2025/03/14/news-corp-sued-by-brave-software-a-google-search-engine-rival |
| 存档 | `sources/c-c2-thestar-countersuit.html|.txt`（ok）；`sources/c-c2-semafor-newscorp.html|.txt`（ok）；`sources/c-c2-thestar-2025-brave-suit.html|.txt`（ok） |
| 失败记录 | The Verge https://www.theverge.com/tech/1024588 ：HTTP 404（`c-c2-verge-brave.html`）。CourtListener：派工单只给了“nysd 676310”，我自拟的 https://www.courtlistener.com/docket/676310/ 为 404（`c-c2-courtlistener-docket.html`），该 URL 非派工单给出，不作为来源引用 |
| 发布时刻 | 反诉：2026-07-21（周二，美国时间；Reuters/Semafor 日期），官方未给时刻；北京时间约 07-22 凌晨（按美东 07-21 计，未核具体钟点）。2025 年 Brave 起诉：Reuters 报道称“Wednesday night complaint”，报道 2025-03-14 刊出 |
| 级别 | L4（据路透社、Semafor 报道） |
| 热度 | `barking heat`：HN 搜索“Brave News Corp lawsuit” 0 命中（时间窗内）；未取 Reddit（失败，见 capture-log-c.md） |

原文摘句（逐字，来自存档文本）：

- 反诉（Reuters，The Star）：“News Corp, ⁠facing a lawsuit by search engine Brave Software, has filed a countersuit accusing it of "flagrant theft" ⁠in distributing and selling versions of articles from the Wall Street Journal and New York ‌Post to AI companies.”
- 法院（Reuters，The Star）：“In a Tuesday filing ​in the Oakland, California, federal court, News Corp said …”
- 指控核心（Reuters）：“News Corp is seeking an injunction and unspecified monetary damages, plus damages of up ​to $150,000 per infringement.”
- Brave 回应（Reuters）：“Brave and its ‌lawyers did not immediately respond to requests for comment outside business hours.”（反诉当日无回应；此后回应未找到。）
- Brave 的立场（Semafor）：“Brave AI was simply indexing website content, “which all search engine operators must do to exist.””（2025 年起诉时的主张，Semafor 转述 L4）
- 2025 年 Brave 起诉（Reuters，The Star，2025-03-14）：“In a Wednesday night complaint filed in San Francisco federal court, Brave said News Corp sent a cease-and-desist letter …”
- 时间线冲突（Semafor）：“But in February, Brave filed its own lawsuit against News Corp …”。Reuters 与 Semafor 对 Brave 起诉月份的说法不一致，Reuters 为 3 月。以 Reuters 为准记录，冲突保留。
- 修订（Reuters）：“It filed a revised ​complaint in May 2026, following what News Corp called failed negotiations …”

截图：无。

条件 / 口径 / 时区：指控只见于 News Corp 的诉状（经路透社转述）；未经核对诉状原文。“据 Reuters 报道”或“据 Semafor 报道”需照用。被告名单见下表。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 / 口径 / 时区 | 状态 |
|---|---|---|---|---|---|---|
| NYT 等起诉 Brave | — | 未找到（NYT 声明、卷宗、The Verge 均未取得） | 检索与存档中原告为 News Corp，未见 NYT 诉 Brave | 无 | 扫描称的“起诉书”与“另一组报纸同日另案”均未找到 | 与说法不符（原告为 News Corp，检索到的材料未见 NYT） |
| 指控 Brave 把索引付费提供给 AI 公司 | L4 | The Star（Reuters）、Semafor | “…distributing and selling versions of articles … to AI companies.” | 无 | 指控方为 News Corp，未经法院认定 | 部分支持（指控存在，但原告与口径不同） |
| 点名 Anthropic、Perplexity、Mistral、xAI、Cohere（非被告） | — | 未找到 | 上述三份存档均未提及这些公司作为被指控买家的表述（Perplexity 仅出现在 News Corp 2025 年另案的背景句） | 无 | — | 未找到一手来源 |
| 内置付费墙拦截 | — | 未找到 | 存档中无“paywall”字样 | 无 | — | 未找到一手来源 |
| Brave 有无回应 | L4 | Reuters（The Star） | 反诉当日：“did not immediately respond to requests for comment” | 无 | 之后的回应未找到 | 未找到（之后回应） |

### C3 Subvocal 无声输入

一句事实（照原文措辞，保留限定词）：Subvocal 团队称其眼镜式产品可把“几乎不可见的无声嘴部动作”解码为文字，自述 200+ WPM；预购页写“Estimated shipping Q1 2027”，且“Delivery subject to FCC approval”。

| 项 | 内容 |
|---|---|
| 一手来源 | https://subvocal.company/ ；https://subvocal.company/preorder ；https://subvocal.company/preorder-terms（版本 2026-10-30）；HN Show HN https://news.ycombinator.com/item?id=50340088 |
| 存档 | `sources/c-subvocal-site.txt`（首次 opencli，已被下一行取代）；`sources/c-subvocal-site-expanded.txt`（FAQ 展开后，主用）；`sources/c-subvocal-preorder-opencli.txt`；`sources/c-subvocal-preorder-terms-opencli.txt`（9,285 字节）；`sources/c-c3-hn-subvocal.html|.txt`（ok）。curl 原件 `c-c3-subvocal-site.html`、`c-c3-subvocal-preorder*.html`、`c-c3-subvocal-how-it-works.html`、`c-c3-subvocal-prototype.html` 均为 1,710 字节 JS 壳（blocked） |
| 发布时刻 | 预购条款版本生效 2026-10-30（官方未给时刻）。HN Show HN 2026-11-03 01:16 北京（UTC 11-02 17:16） |
| 级别 | L1（官网与预购条款，团队自述）；HN 为社区讨论，仅线索 |
| 热度 | HN 50340088：708 分，嵌套评论 343 条（搜索页显示 344 评），取于 2026-11-03 19:14 北京。HN 50309397 “Talk Without Talking”（subvocal.company）：2026-10-31 04:02 北京，4 分 1 评 |

原文摘句（逐字）：

- 产品形态（预购页）：“The glasses”；条款“Subvocal glasses”。
- 原理（官网首页）：“Subvocal decodes almost-invisible subtle, silent mouthing into text at 200+ WPM.”
- 嘴部动作（FAQ，展开后）：“Subvocal uses deliberate silent articulation. You mouth words as subtly as you want, without producing audible speech. Up close, someone may notice slight movement, but at normal conversational distance it’s designed to be be invisible.”（原文 “be be” 为原网页拼写）
- 不用摄像头（FAQ）：“No. Subvocal does not use or require cameras for silent speech input.”
- 速度（官网）：“Your words flow at 200+ words per minute.”（团队自述，未给测试条件）
- 校准（FAQ）：“There will be a short personal calibration process when you first set up Subvocal.”
- 准确率（FAQ）：“The current prototypes already decode open-ended silent speech in real time, and accuracy continues to improve as we collect more data and train larger models.” 未给任何数字。
- 出货（预购条款）：“Estimated shipping is Q1 2027, not a guaranteed date.” 以及“We currently estimate that the first batch will ship in Q1 2027. This is a good-faith estimate …”
- 价格与定金（预购页与条款）：“$199 today”定金，“Total Early Bird price $699 → $499”；“Delivery subject to FCC approval”；仅限美国地址发货（条款）。
- 射频（扫描称）：官网与条款中未找到“radio”“RF”“射频”字样。

截图：无。

条件 / 口径 / 时区：200+ WPM 为团队自述，无测试集、无对照。“Q1 2027”为估计，非保证。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 / 口径 / 时区 | 状态 |
|---|---|---|---|---|---|---|
| 产品为眼镜 | L1 | subvocal.company/preorder | “The glasses” | 无 | — | 已找到 |
| 闭口发音（无声嘴部动作） | L1 | 官网 FAQ | “Subvocal uses deliberate silent articulation …” | 无 | 近距离可能察觉轻微动作 | 已找到 |
| 眼镜射频探测 | — | 未找到 | 官网、条款中无射频字样 | 无 | 仅见 “Delivery subject to FCC approval” | 未找到一手来源 |
| 需个人校准 | L1 | 官网 FAQ | “There will be a short personal calibration process …” | 无 | — | 已找到 |
| 计划 2027 年一季度出货 | L1 | preorder-terms | “Estimated shipping is Q1 2027, not a guaranteed date.” | 无 | 估计，非保证；需 FCC 批准 | 已找到（计划、非保证） |
| 团队自述 200+ 词/分钟 | L1 | 官网首页 | “Subvocal decodes almost-invisible subtle, silent mouthing into text at 200+ WPM.” | 无 | 团队自述；无测试条件 | 已找到（团队自述） |
| 准确率数字 | — | 未找到 | FAQ 仅写 “accuracy continues to improve”，无数字 | 无 | — | 未找到一手来源 |

### C4 OpenAI 自研 Git 后端 Pando

一句事实（保留“内部”“中位”与“近”）：OpenAI 称，其内部 monorepo 的全部开发已由自研 Git 后端 Pando 承载，并保持 Git 协议与客户端兼容；原文称生产环境中位推送延迟“降低近 5 倍”，每天传输“近一 PB”的 Git 数据。

| 项 | 内容 |
|---|---|
| 一手来源 | https://openai.com/index/scaling-git-at-openai/ |
| 存档 | `sources/c-c4-openai-pando.html|.txt`（ok，659 行） |
| 发布时刻 | 2026-11-02（页面日期，官方未给时刻）。原文称 Pando “began serving production traffic in April of this year”（2026 年 4 月起上线） |
| 级别 | L1（官方工程博客） |
| 热度 | HN 50345924 “Scaling Git at OpenAI”：236 分，嵌套评论 151 条，2026-11-03 07:47 北京发帖（UTC 11-02 23:47），取于 19:14 北京。HN 50346696 “Pando - OpenAI's alternative to Git for source control”（指向 OpenAIDevs 推文）：3 分 0 评，2026-11-03 09:11 北京 |

原文摘句（逐字）：

- 全量使用：“Pando now powers all development in OpenAI’s monorepo.”
- 兼容性：“We met these goals while retaining compatibility with the Git protocol and client, enabling both our engineers and agents to seamlessly adopt Pando into their existing workflows.”
- 中位延迟与传输量：“In production, Pando cut median push latency by nearly 5x. It now delivers nearly a petabyte of Git data per day.”
- 开源（原文表述）：“We’re currently exploring ways to share the technology that makes Pando possible with our users.” 另有“we’ll continue to develop Pando while working with the open-source Git community to share any improvements that benefit Git users more broadly.”（指向上游 Git 的改进，不等于 Pando 开源）。
- 基线未给：全文只有上述一句给出延迟比例，未给对照系统与基线数值。

截图：无。

条件 / 口径 / 时区：“nearly 5x”为作者自报的生产环境数字，测试条件与基线未给。“内部”指 OpenAI 内部 monorepo，不涉及外部用户。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 / 口径 / 时区 | 状态 |
|---|---|---|---|---|---|---|
| 兼容 Git 协议 | L1 | openai.com/index/scaling-git-at-openai/ | “…retaining compatibility with the Git protocol and client …” | 无 | — | 已找到 |
| 内部单体仓库全部使用 | L1 | 同上 | “Pando now powers all development in OpenAI’s monorepo.” | 无 | 仅限 OpenAI 内部 monorepo | 已找到 |
| 内部 push 中位延迟降约 5 倍 | L1 | 同上 | “In production, Pando cut median push latency by nearly 5x.” | 无 | 原文为“近 5 倍”（nearly 5x）；基线与测试条件未给 | 部分支持（“约”改为“近”；基线未给） |
| 日传输近 1 PB | L1 | 同上 | “It now delivers nearly a petabyte of Git data per day.” | 无 | 为作者自报 | 已找到 |
| 是否开源 | L1 | 同上 | “We’re currently exploring ways to share the technology that makes Pando possible with our users.” | 无 | 未见开源、未见代码仓库 | 已找到（未开源；官方称在探索分享） |
| 发布日期 | L1 | 同上 | 页面日期 “November 2, 2026” | 无 | 官方未给时刻；北京时间日期随之为 11-02 或 11-03，取决于时刻 | 已找到（日期） |

---

## 二、扫描说法勘误

以下为扫描回贴（ChatGPT、WorkBuddy）与派工单“已知情况”中，与一手材料不符或未能确认的部分。

| 条目 | 扫描说法 | 核查结果 |
|---|---|---|
| C1 | 扫描给的链接为 aistudio.google.com/docs/… | 实际能打开的一手页面：ai.google.dev 的两页均跳转到 aistudio.google.com/docs/changelog 与 …/docs/deprecations，已据此存档。 |
| C1 | 12 月 1 日起 2.5 Pro 转 3.8 Flash，按 Flash 计价 | 确认（“billed at Gemini 3.8 Flash rates”）。 |
| C1 | 2.5 Flash 转 3.5 Flash-Lite | 确认。 |
| C1 | AI Studio 已不可选 | 确认。 |
| C1 | 部分参数不兼容 | 确认（原文列出 function calling 与 thought_signature、thinking_level、移除采样参数与预填模型回合）。 |
| C1 | 仅限 Gemini API | 未找到 Vertex 口径，两页只写“through the API”。 |
| C2 | NYT 等起诉 Brave，点名 Anthropic、Perplexity、Mistral、xAI、Cohere | 与说法不符。检索到的一手或 L4 材料中原告是 News Corp（WSJ 与 NY Post 母公司），未见 NYT 起诉 Brave。被点名的 AI 公司均未找到。 |
| C2 | The Verge 1024588 | 404，未取得。 |
| C2 | CourtListener nysd 676310 | 未取得（派工单只给编号；我自拟的 URL 为 404，不采用）。 |
| C2 | 内置付费墙拦截 | 未找到。 |
| C2 | 另一组报纸同日另案 | 未找到。 |
| C2 | Brave 起诉时间 | 来源冲突：Reuters（2025-03-14）称 Brave 的起诉为“Wednesday night”；Semafor（2026-07-21）称“in February”。 |
| C3 | 眼镜射频探测闭口发音动作 | 眼镜形态确认；射频未找到。 |
| C3 | 需个人校准 | 确认。 |
| C3 | 计划 2027 年一季度出货 | 确认，但为“estimated, not guaranteed”，并“Delivery subject to FCC approval”。 |
| C3 | 团队自述 200+ 词/分钟 | 确认（“200+ WPM”，团队自述）。 |
| C3 | 准确率 | 未找到数字。 |
| C4 | 内部 push 中位延迟降约 5 倍 | 原文为“nearly 5x”（近 5 倍），基线未给。 |
| C4 | 兼容 Git 协议；内部单体仓库全部使用；日传输近 1 PB | 确认。 |
| C4 | 是否开源 | 未开源。原文称“正在探索”与用户分享该技术。 |
| 热度 | HN 50340088 707 分 343 评；HN 50345924 235 分 148 评 | 当前为 708 分 343 评（嵌套）与 236 分 151 评（取于 11-03 19:14 北京）。 |

---

## 三、本组新增文件

- `docs/2611/1102/sources/evidence-c.md`（本文件）
- `docs/2611/1102/sources/capture-log-c.md`
- `docs/2611/1102/sources/c-*`（存档：`c-fetch-log.jsonl`、`c-heat.json`、`c-hn-50340088.json`、`c-hn-50345924.json`、`c-*.html|.txt|.headers`，以及 opencli 渲染文本 `c-gemini-*-aistudio.txt`、`c-subvocal-*.txt`）

## 四、待删除文件清单（精确路径）

以下为本组的废弃文件，未删除，待 Claude 收尾时统一处理：

- docs/2611/1102/sources/c-subvocal-site.txt（已被 `c-subvocal-site-expanded.txt` 取代）
- docs/2611/1102/sources/c-c3-subvocal-site.html（1,710 字节 JS 壳）
- docs/2611/1102/sources/c-c3-subvocal-preorder.html（同上）
- docs/2611/1102/sources/c-c3-subvocal-preorder-terms.html（同上）
- docs/2611/1102/sources/c-c3-subvocal-how-it-works.html（同上）
- docs/2611/1102/sources/c-c3-subvocal-prototype.html（同上）

注：`c-c2-verge-brave.html` 与 `c-c2-courtlistener-docket.html` 为 404 页原件，留作失败记录，不建议删除。
