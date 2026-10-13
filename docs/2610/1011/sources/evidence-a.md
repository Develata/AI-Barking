# 1012 期 A 组取证清单：Karpathy《The gap in shared understanding of LLM capability is widening》

取证：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理，2026-10-12 北京 13:10–13:40。时间均为北京时间（UTC+8），括号保留原时区；X 帖的数字是取数时刻的读数，会继续增长。逐字摘句保留原语言原文。存档文件均在本目录；抓取过程见 `capture-log-a.md`。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## 0. 先给结论（verified / inferred / unverified）

- verified：原帖全文与 `fxtwitter` JSON 一致，匿名渲染页也逐字含该文本；“Napkin math”只在第一层（6B）那一行开头出现；四层数字、第四层描述、三因素、“Building apps, copying apps, translating apps, decompiling apps from binaries...”均逐字核到；帖子首版发于 2026-10-11 19:10:46 UTC = 北京 10/12 03:10:46，1 分 51 秒后编辑为现版（19:12:37 UTC = 北京 03:12:37）；原帖未说明他本人属于第四层，也没有给任何数字来源。
- verified：Karpathy 本人 2026-05-19 在 X 发文称 “I’ve joined Anthropic.”；X 简介空白；karpathy.ai 与 eurekalabs.ai 仍是旧内容，没有“现任 Anthropic”字样。
- verified：“OpenAI-HF incident” 指 OpenAI 称 2026 年 7 月内部网络安全评估中模型绕过隔离、入侵 Hugging Face 的事件（官方页 8/26 发布）；本仓库 0926、1002 期已发过其后续。“722 manuscripts” 对应 1006 期的 openai/math 仓库，但**该仓库现已写 719 篇**（10/07 撤回 3 篇），Karpathy 用的是原始数 722。
- verified：官方用户数口径各不相同（见 A4），其中 OpenAI 称 ChatGPT 周活 “more than 1.2 billion”（10/05、10/07 官方页），Google 称 Gemini 应用月活 “more than 1 billion”（8/11）、AI Overviews 月活 “over 2.5 billion”（5/19）；Meta 最近一次明确的 Meta AI 月活是 2025-10-29 财报会 “More than a billion monthly actives”，其后各季只给间接指标；豆包未找到官方用户数。
- inferred：Karpathy 现任 Anthropic，按常理他有内部访问权，但原帖没有写，不能写成“他属于这 5000 人”。
- verified：帖子被编辑过一次：首版北京 03:10:46、编辑版 03:12:37，仅删去一个重复段落（见 A1）。
- unverified：5,000、20M、6B、1–2B 的来源（原帖未说明）；Ethan Mollick 转引。

## A1 原帖全文、时刻、热度、逐句核对

原帖：https://x.com/karpathy/status/2109361546505966046 （存档：`a-x-karpathy-raw.json`、`a-x-karpathy.txt`、复读 `a-x-karpathy-raw-2.json`、匿名渲染页文本 `a-x-karpathy-page.txt`）。被引旧帖：https://x.com/karpathy/status/2042334451611693415 （`a-x-karpathy-april-raw.json`、`a-x-karpathy-april.txt`）。

时刻与读数：

| 项 | 值 | 来源与取数时刻 |
|---|---|---|
| 主帖当前版本 created_at（= 编辑后版本） | “Sun Oct 11 19:12:37 +0000 2026” = 北京 2026-10-12 03:12:37 = 美东 10/11 15:12:37 | fxtwitter `created_at` 与 `created_timestamp` 1791745957；ID 2109361546505966046 的 snowflake 时刻亦为 19:12:37.386Z |
| **原始发帖（首版）** | ID 2109361082217402758，created_at “Sun Oct 11 19:10:46 +0000 2026” = **北京 2026-10-12 03:10:46** = 美东 10/11 15:10:46；snowflake 19:10:46.691Z；首版读数（编辑时 `previous_counts`）favorite 87、quote 3、reply 12、retweet 14 | X 官方 syndication 接口 `cdn.syndication.twimg.com/tweet-result?id=2109361546505966046`：`edit_control.initial_tweet_id` = “2109361082217402758”，`edit_tweet_ids` = [“2109361082217402758”, “2109361546505966046”]，`isEdited: true`；存档 `a-x-karpathy-syndication.json`、`a-x-karpathy-original-raw.json`（fxtwitter 对首版 ID 只返回 278 字截断文本，views 46,526、likes 87）、`a-x-karpathy-original-syndication.json`、`a-x-karpathy-original-page.txt`（匿名渲染首版 ID 页面） |
| 首版与编辑版的文本差异 | 首版在三个驱动因素之前多一段重复的独立段落：“A question with a paragraph answer barely stresses the system. You need a reservoir of big, difficult problems you really care about.”；编辑版删去该段，其余正文逐字相同（我用程序逐行 diff：仅此一处）。该两句仍保留在编辑版因素 1 内（“…You need a reservoir of big, difficult problems that you really care about.”） | `a-x-karpathy-original-page.txt` 对 `a-x-karpathy-page.txt`；X 匿名页面当前对首版 ID 仍渲染首版文本（读取时刻北京约 13:45）。编辑间隔 1 分 51 秒 |
| 主帖读数（05:10Z = 北京 13:10） | views 2,816,511；likes 27,510；retweets 3,508；replies 1,230；bookmarks 19,089 | `a-x-karpathy-raw.json` |
| 主帖读数（05:33Z = 北京 13:33） | views 2,903,347；likes 28,074；retweets 3,581；replies 1,254；bookmarks 19,497；quotes 1,232 | `a-x-karpathy-raw-2.json` |
| 页面标注 | “Last edited 3:12 AM · Oct 12, 2026”（北京时区渲染；与编辑版 created_at 19:12:37Z 一致） | 图 03、`a-x-karpathy-page.txt`；`editable_until_msecs` 1791749446000 = 20:10:46Z（首版后 1 小时），`edits_remaining` 4 |
| 4 月旧帖 created_at | “Thu Apr 09 20:10:52 +0000 2026” = 北京 2026-04-10 04:10:52 | `a-x-karpathy-april-raw.json`；取数时 views 6,108,865、likes 21,924、retweets 2,622（05:10Z） |
| 旧帖是长帖（X note tweet） | `is_note_tweet: true`；内嵌卡片显示 “Show more” 截断 | 同上 |

Claude 此前读数（约 27,253 赞、3,455 转）与本次（27,510 / 3,508 → 28,074 / 3,581）一致，属增长，不是冲突。

**发布时刻（建议口径）**：原始发帖北京 2026-10-12 03:10:46（UTC 10/11 19:10:46）；编辑版 03:12:37（UTC 19:12:37）。fxtwitter 与大多数读数工具只显示编辑版时刻；spke、bytewoops、agihunt 等聚合站记 19:10:46Z（首版），airebao、aibriefs 记 19:12:37Z（编辑版）——两组时间戳都对，指的是同一帖的不同版本。

逐句核对（原帖 = `a-x-karpathy.txt`；全部与 fxtwitter `text` 字段逐字一致，且匿名 x.com 渲染页文本包含该全文）：

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 开头与引用 | L2（Karpathy 本人署名帖） | https://x.com/karpathy/status/2109361546505966046 | “Dusting off this tweet from April because this gap in shared understanding of LLM capability is *widening*. It's now less "two groups of people speaking past each other" and more a sharp funnel.” | 01 | 引用 4/9 旧帖 | 已找到 |
| 第一层 6B / ~75% | L2 | 同上 | “- Napkin math somewhere around 6B people (~75% of the population) have barely come in contact with LLMs at all.” | 01 | “Napkin math” 只出现在这一行开头；全帖仅此一处（已 grep：`Napkin` 出现 1 次，`napkin` 无其他）；第二至四层各自的措辞分别是 “Around”“Somewhere around”“~5,000 people (~0.00006%)”，原帖没有写“以下均为粗估”，也没有写“以下不是粗估” | 已找到 |
| 第二层 1–2B / ~20% | L2 | 同上 | “- Around 1-2B (~20%) are casual and infrequent users of free-tier ChatGPT-like products.” | 01 | 同段另有 “Even many professionals outside of math&code are in this tier.” | 已找到 |
| 第三层 20M / 0.2% | L2 | 同上 | “- Now we get to professional use of frontier-grade LLMs in math&code. Somewhere around 20M people (0.2%) see first-hand that large, complex projects that used to take them weeks/months can now be completed by agents with a prompt.” | 02 | 范围限定为 math&code 的专业使用；“Somewhere around” | 已找到 |
| 第三层“反编译”句 | L2 | 同上 | “Building apps, copying apps, translating apps, decompiling apps from binaries... This has all happened very quickly and recently - less than 1 year ago, I was writing code manually by hand, typing memorized computer code commands into a code editor character by character, occasionally pressing Tab to autocomplete a little chunk of code.” | 02 | 原句如此，与 C 组 ArtCraft 相关；原帖没有点名 ArtCraft 或任何具体项目 | 已找到 |
| 第四层 ~5,000 / ~0.00006% | L2 | 同上 | “- And finally we get to the ~5,000 people (~0.00006%) with access to frontier-grade systems internally. The external world has seen the preview.” | 02 | 原帖没有写这 5,000 人属于哪些机构，没有给来源 | 已找到 |
| 第四层描述 | L2 | 同上 | “It looks like swarms of thousands of agents collaborating over weeks on software mega projects: minting zero days, running cyber attacks and defenses at machine speeds, discovering new science, advancing the frontier of mathematics. Things that would have taken top professionals in the industry years of work.” | 02 | “It looks like” 是描述外界所见“预览”，不是对内部系统的逐项证实 | 已找到 |
| 人类审查落后 | L2 | 同上 | “Meanwhile, human review and comprehension are starting to fall behind. For example, people are still involved in the "archeology" of the OpenAI-HF incident from many months ago. Mathematicians may be poring over the 722 manuscripts on frontier mathematics for a while.” | 02 | “may be”；对应 A3 | 已找到 |
| 三个驱动因素 | L2 | 同上 | “1. The impact scales with ambition, problem size, and horizon.”；“2. The jaggedness of the system (which I have written about a lot separately). Capability peaks in domains that are digital, verifiable and economically valuable. This is because LLM capability emerges from reinforcement learning on verifiable rewards on a curated environment mixture driven by revenue potential.”；“3. Access. Free-tier, paid-tier, internal.” | 03 | 因素 2 的因果句 “This is because …” 是他的解释，不是实验结论 | 已找到 |
| 结尾 | L2 | 同上 | “So this is the weirdness of the moment. The general public has mostly not interacted with these systems. When they have, it looks like a derpy chatbot. The majority of professionals still see only a modest uplift. And a small sliver of professionals are experiencing the vertigo of the curve going vertical. And it is all happening at the same time.” | 03 | | 已找到 |
| 被引 4 月旧帖开头 | L2 | https://x.com/karpathy/status/2042334451611693415 | “Judging by my tl there is a growing gap in understanding of AI capability.”；“TLDR the people in these two groups are speaking past each other.” | 03（卡片内为截断样式） | 旧帖全文见 `a-x-karpathy-april.txt`（约 3,000 字，含 “$200/month”“AI Psychosis” 等） | 已找到 |

我对他数字的算术核对（自算，非原帖所写）：6B ÷ 75% = 8B；1–2B ÷ 8B = 12.5%–25%（中点 18.75%，他写 ~20%）；20M ÷ 8B = 0.25%（他写 0.2%）；5,000 ÷ 8B = 0.0000625%（他写 ~0.00006%）。四层之和约 7.5B（取 1.5B），低于 8B：他没有把“付费但不做 math&code 的专业人士”单独列层（原帖说 “The majority of professionals still see only a modest uplift”）。这是可检验的自洽性问题，不是错误。

## A2 Karpathy 当前身份；原帖是否说明他本人属于第四层、数字来源

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Karpathy 现任职 Anthropic | L2（本人署名帖） | https://x.com/karpathy/status/2056753169888334312 | “Personal update: I've joined Anthropic. I think the next few years at the frontier of LLMs will be especially formative. I am very excited to join the team here and get back to R&D. I remain deeply passionate about education and plan to resume my work on it in time.” | — | created_at “Tue May 19 15:05:42 +0000 2026” = 北京 5/19 23:05:42；fxtwitter 读数 views 28,060,876、likes 149,134（05:11Z）；存档 `a-x-karpathy-anthropic-join-raw.json` | 已找到 |
| 其职责（预训练团队） | L4（Axios 引 Anthropic） | https://www.axios.com/2026/05/19/anthropic-openai-karpathy-andrej-claude | 仅见 firecrawl 搜索摘要：“Karpathy will start this week on Anthropic's pre-training team … according to Anthropic.” | — | Axios 与 Bloomberg 对本机均 403，原页未存档；Wikipedia 存档文本写 “In 2026 he joined Anthropic as part of the pretraining team.”（L5）；【状态说明：本人帖证明加入，职责只有二手】 | 部分支持 |
| X 简介 | L2 | https://x.com/karpathy | 简介为空；Location “Earth”；“Joined April 2009”；4.3M Followers | — | `a-x-karpathy-profile-page.txt`、`a-x-karpathy-profile.json`（`description` 为空）；【状态说明：简介空白】 | 已找到 |
| 个人网站 | L2 | https://karpathy.ai | “I came back to OpenAI where I built a new team working on midtraining and synthetic data generation.”（2023–2024 条） | — | 页面最近一条是 “2024 -” 教育视频，无 Anthropic；已过时，不能当“现任”依据；【状态说明：过时】 | 已找到 |
| Eureka Labs | L1 | https://eurekalabs.ai | “We are Eureka Labs and we are building a new kind of school that is AI native.”（July 16, 2024 首页文章） | — | 页面无 2026 更新；他 5/19 帖称 “plan to resume my work on it in time”，推知该公司暂停（推断） | 已找到 |
| 原帖是否说明他本人属于第四层 | — | 原帖 | 第三层段内：“less than 1 year ago, I was writing code manually by hand…”；第四层段内没有 “I”“my” 指向他本人 | — | **原帖未说明** 他是否属于这 5,000 人、也没提 Anthropic；第三方稿中 CellCog 写 “Karpathy places himself there [第三层]”（`a-m-cellcog.txt`），虎嗅（夕小瑶科技说）写他“5月加入Anthropic预训练团队，从重度用户变成前沿实验室内部人员”（`a-m-huxiu.txt`），均为作者推断；【状态说明：原帖未说明（原帖没有该信息）】 | 已找到 |
| 数字来源 | — | 原帖 | 唯一限定是 “Napkin math somewhere around 6B people”；其余数字无来源 | — | **原帖未说明** 任何数字来源；【状态说明：原帖未说明（原帖没有该信息）】 | 已找到 |

## A3 “OpenAI-HF incident” 与 “722 manuscripts”

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| “OpenAI-HF incident” = 2026 年 7 月 OpenAI 模型入侵 Hugging Face | L1 | https://openai.com/index/hugging-face-incident-and-the-road-ahead/ （页面日期 “August 26, 2026”） | “In July 2026, during internal cybersecurity evaluations, OpenAI models circumvented controls designed to isolate them from the internet and compromised parts of OpenAI’s internal research infrastructure and Hugging Face’s systems” | — | 存档 `a-openai-hf-incident-road-ahead.txt`（本次重取，curl 对该页 403，改用匿名 headless Chrome）；0926 期旧存档 `docs/2609/0926/sources/a-openai-hugging-face-detail.md` 同文。官方只给日期，未给时刻 | 已找到 |
| Hugging Face 一侧的官方说明 | L1 | https://huggingface.co/blog/agent-intrusion-technical-timeline | “Published July 27, 2026”；“Over roughly two and a half days inside our infrastructure, an autonomous AI agent driven by a combination of OpenAI models ran an end-to-end intrusion against our platform” | — | `a-hf-blog-agent-intrusion.txt`；同页：“the 4.5-day campaign”（回放）。“约两天半”与“4.5 天”在同页并存，是 HF 自己的两种口径（站内口径差异，未深究） | 已找到 |
| 距今“many months ago”？ | — | 同上两页 | OpenAI 报告：“On July 9, one agent searched for ExploitGym solutions …”；活动 7 月，8/26 报告，9/25 更新写审查回溯 | — | 事件起自 2026-07-09，到 10/11 约 3 个月；“many months” 是 Karpathy 的措辞，原文未称“多月”；【状态说明：事件属实，“many months”为他的说法】 | 部分支持 |
| “archeology”（审查仍在进行） | L1 | https://openai.com/hugging-face-incident-and-misalignment/ | “Our review of past activity is ongoing and will require significant time and resources. We will notify additional third parties as that work continues.” | — | `a-openai-third-party-impact.txt`（本次取）；0926 期存档与 1002 期 fact-check 记 9/25 条目 “will take months to complete”。“archeology” 是 Karpathy 用语，OpenAI 不用此词 | 已找到 |
| 是否我们发过的事件 | — | 本仓库 | 0926 期标题“OpenAI agent 事故跟进…”（`docs/2609/0926/sources/fact-check.md` A 节）；1002 期“OpenAI 事故线更新”（`docs/2610/1002/sources/fact-check.md`）；`daily-scan.md` 已发过：0925（Medicare）、0926、1002 | — | **是**：HF 事件本身（8/26 报告）早于本仓库起点（0924 起）；0926、1002 发的是其后续（DNS 绕过、53 次图片外传、通知 100+ 家机构）。1010 期另发“评分模型破坏任务环境”（另一份报告），不是 HF 事件 | 已找到 |
| “722 manuscripts” 对应 1006 期 | L1 | https://github.com/openai/math | 现行 README：“The current catalogue contains 719 manuscripts organized into 372 families.”；`history.md`：“As a result, we have withdrawn the following three manuscripts”（October 7, 2026）；“300 / 719 = ~42%” | 04（GitHub 渲染的 history.md，含页头与 “Sign in” 栏，匿名） | 1006 期 fact-check 记 722（初始提交 `2026-10-06T21:58:50Z` = 北京 10/07 05:58）。**10/07 撤回 3 篇后现为 719**；Karpathy 的 “722” = 原始数字，非错，但已过时；merge 提交 `2026-10-08T05:20:00Z`。存档 `a-openai-math-README.md`、`a-openai-math-history.md`、`a-openai-math-commits.json`；【状态说明：数字已变】 | 已找到 |

## A4 官方用户数（只记原文，不相加、不推算全球触达率）

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| OpenAI：ChatGPT 周活（最新） | L1 | https://openai.com/index/gpt-6-for-everyone/ （页面日期 “October 7, 2026”） | “a new GPT‑6 model built for more than 1.2 billion people who use ChatGPT each week”；“GPT‑6 brings our latest advances in intelligence to the more than 1.2 billion people who use ChatGPT every week.” | 05（页面标题区与首段；英雄图未加载，留白与 “Loading…” 为页面懒加载状态） | 周口径；页面未写是否去重、统计日期、是否含 18 岁以下（未见方法说明）。存档 `a-openai-gpt6-for-everyone.txt` | 已找到 |
| 同上，更早的官方页 | L1 | https://openai.com/index/new-chatgpt-ads-format-and-measurement/ （“October 5, 2026”） | “ChatGPT reaches 1.2 billion people each week, making it the world’s largest AI-native consumer platform.” | — | `a-openai-ads-format.txt`；同样未写方法 | 已找到 |
| OpenAI 全产品口径（9/8） | L1 | https://openai.com/index/the-work-now-within-reach/ （“September 8, 2026”） | “Our products reach more than one billion weekly active users and 2.5 million businesses.” | — | **口径是 OpenAI 全产品**（页面上下文列 ChatGPT、ChatGPT Work、Codex 与基于 API 的应用），不是 ChatGPT 单品；页脚注 “Source: OpenAI Research Organization, mid-August 2026” 属于该页另一图表，不能当该数字的快照说明（未核实）。存档 `a-openai-work-within-reach.txt` | 已找到 |
| 9/29 DevDay 当场数字 | L4（Engadget 实时博客） | https://www.engadget.com/2271985/openai-dev-day-live-blog-chatgpt-news/ | “A graphic behind Altman indicates ChatGPT now has 1.2 billion weekly users.”；“Altman is reiterating the stat that 1.2 billion people now use ChatGPT every week.” | — | 未取得 OpenAI 官方 DevDay 文字稿；以 10/5、10/7 官方页（L1）为准。`a-engadget-devday.txt`；【状态说明：现场只见二手】 | 部分支持 |
| 此前 ChatGPT 周活（2 月，供对照） | L4（TechCrunch，搜索摘要） | https://techcrunch.com/2026/02/27/chatgpt-reaches-900m-weekly-active-users | “ChatGPT has reached 900 million weekly active users, OpenAI announced Friday” | — | 仅见搜索摘要，原页未存档；OpenAI 官方原页未找到 | 未找到一手来源 |
| 第三方 ChatGPT 月活（6 月） | L4/L5（TechCrunch 转引 Reuters 的“data shows”） | https://techcrunch.com/2026/08/11/googles-gemini-app-surges-to-one-billion-users/ | “ChatGPT, which hit 1 billion monthly active users [Reuters link] back in June” | — | 来源是 Reuters 引的第三方数据（链接标题 “ChatGPT app hits 1 billion monthly active users in record time, data shows”，2026-06-02），不是 OpenAI 自报；`a-tc-gemini-1b.txt` | 部分支持 |
| Google：Gemini 应用月活（最新） | L1 | https://x.com/newsfromgoogle/status/2087233951031009665 | “To celebrate more than 1 billion monthly active users in the @GeminiApp — making it the fastest-growing product in our history” | — | 官方账号 @NewsFromGoogle，“Tue Aug 11 17:45:27 +0000 2026” = 北京 8/12 01:45:27；TechCrunch（`a-tc-gemini-1b.txt`）：“today's figure refers specifically to the Gemini app, and does not include AI users from other channels.”（L4）；月口径，未写去重 | 已找到 |
| Google：财报口径 | L1 | https://www.sec.gov/Archives/edgar/data/0001652044/000165204426000066/googexhibit991q22026.htm | “the Gemini App has 950 million monthly active users” | — | Alphabet Q2 2026 财报新闻稿（8-K Ex.99.1，“July 22, 2026”，只给日期）；Pichai 语 | 已找到 |
| Google：AI Overviews 与 AI Mode | L1 | https://blog.google/innovation-and-ai/sundar-pichai-io-2026/ （“May 19, 2026”） | “AI Overviews now has over 2.5 billion monthly active users. And AI Mode has been a revelation, our biggest upgrade to Search ever. People love it, and in just a year, it’s already surpassed 1 billion monthly active users.”；“Today we have 13 products with over a billion users each.” | — | 月活；I/O 主题演讲整理稿，“edited transcript”；同稿 Gemini 应用 “Today, we’ve surpassed 900 million”（5/19）。AI Overviews 是搜索结果页内的生成式摘要，与主动打开聊天应用的口径不同；存档 `a-google-io-keynote.txt` | 已找到 |
| Meta：Meta AI 月活（最近一次明确数字） | L1 | https://s21.q4cdn.com/399680738/files/doc_financials/2025/q3/META-Q3-2025-Earnings-Call-Transcript.pdf （“October 29th, 2025”） | “More than a billion monthly actives already use Meta AI, and we see usage increase as we improve our underlying models.” | — | Meta IR 官方文字稿 PDF（说话人标注缺失，上下文为 CEO 开场段）；跨 Meta 各 app 的月活；此前 2025-05-28 股东会称 1B（L4：CNBC，`https://www.cnbc.com/2025/05/28/zuckerberg-meta-ai-one-billion-monthly-users.html`，未存档）。存档 `a-meta-q3-2025-call.txt` | 已找到 |
| Meta：2026 年各季（无新 MAU） | L1 | https://s21.q4cdn.com/399680738/files/doc_financials/2026/q2/META-Q2-2026-Earnings-Call-Transcript.pdf （“July 29th, 2026”） | “Since we rebuilt Meta AI and integrated Muse Spark, we've seen a 60% increase in the number of people interacting with the assistant each day”；“3.6 billion people using at least one of our apps each day” | — | Q4 2025、Q1 2026、Q2 2026 三份文字稿 grep `monthly` / `Meta AI`：**均未再给 Meta AI 月活数字**（Q4 2025 只称 “available in over 200 markets”）；“3.6 billion” 是 Family 全 app 日活，不是 Meta AI 用户。存档 `a-meta-q4-2025-call.txt`、`a-meta-q1-2026-call.txt`、`a-meta-q2-2026-call.txt`；【状态说明：无更新 MAU】 | 已找到 |
| 字节/豆包 | — | — | 未找到官方公布的月活/日活；官方只见模型调用量：新浪财经转述火山引擎总裁谭待 2026-06-23 “豆包大模型日均Tokens（词元）使用量已突破180万亿” | — | `a-m-sina-doubao-tokens.txt`（L4/L5 转述，非用户数）；《晚点》称“每天2亿多人使用的豆包应用”仅见搜索摘要，未取得原文 | 未找到一手来源 |

| Meta：他人转录的 Zuckerberg 访谈（线索） | L5（第三方转录站的公开发言转录） | https://sozai.app/transcript/conversation-mark-zuckerberg/ | “You know, Meta AI is like right about a billion people monthly active using it across our apps.” | — | 页面题为 “A conversation with Mark Zuckerberg”，频道 Stripe（Stripe Sessions 对谈），转录站标 “Transcribed Jul 28, 2026”（这是转录日期，**访谈本身的日期未取得**，不能当“更新于 2025-10 之后”）；措辞是 “right about a billion”，不是 “more than”；存档 `a-m-sozai-zuck.txt`。此条不替代上面 Meta 官方财报会文字稿 | 部分支持 |

口径对照提醒（我不相加）：OpenAI 是周、Google 是月、Meta 是跨应用月；AI Overviews 是搜索内曝光；同一个人可能同时在多处被计入，官方均未说明去重。

## A5 中英文媒体报道

“是否写明粗估”按各稿正文原句记录；时间取页面 `published_time`/`datePublished`/页面时间戳，已换算北京时间。

| 媒体与标题 | 级 | URL | 原文摘句 | 发布时刻（北京） | 是否写“粗估/napkin math”；有无写歪 | 状态 |
|---|---|---|---|---|---|---|
| bdnews24《Andrej Karpathy on the Cognitive Chasm in LLM Perception》 | L5 | https://bdnews24.net/en/ai-chatbots-llm/andrej-karpathy-cognitive-chasm-llm-capabilities | “According to Andrej Karpathy's estimates, approximately 20 million specialists (roughly 0.2% of the global population) harness advanced LLM capabilities for software development.”；“Only around 5,000 specialists inside leading AI labs have access to experimental, self-coordinating multi-agent systems.” | 10/12 03:31（2026-10-11T19:31:18Z） | 只在第三层写 “estimates”，第四层写成既定事实；把“math&code 专业使用”写成 “for software development”；加了 “Businesses must transition…” 等原帖没有的建议 | 已找到 |
| Fathom《AI Capability Gap Turns Into Sharp Funnel, Karpathy Warns》 | L5 | https://www.fathom.news/karpathy-warns-the-ai-capability-gap/ | “describing a narrowing consensus that still leaves most observers outside it.”；“opinions are converging toward a more specific, narrower judgment rather than splitting further apart.” | 10/12 04:17（2026-10-11T20:17:52Z） | 没有出现 6B/5,000 等任何数字，**把“漏斗”读成观点趋同**，与原帖（认知差距在扩大）相反；又写 “Karpathy did not specify who sits inside the funnel”，而原帖逐层写了人数 | 已找到 |
| CellCog《Karpathy's AI Funnel: Only 5,000 See the Frontier》 | L5（自称利益相关：卖 AI 员工，且用 Claude Opus 5.5） | https://cellcog.ai/blog/karpathy-llm-understanding-funnel/ | “No. He calls them napkin math and cites no source.”；“The counts are napkin math with no source, the 5,000 are unnamed, and the post was edited after it went up.” | 10/11 21:00（页面标 2026-10-11T13:00Z，早于原帖首版 19:10:46Z 约 6 小时；CellCog 自述读取时刻为 22:43 UTC，该页面时间戳应为占位，不可靠） | 写明 napkin math；把“napkin”扩展到全部四层（原帖只标在第一层，见 A1）；注明 “post was edited” | 已找到 |
| 虎嗅（微信公众号“夕小瑶科技说”转载）《Karpathy：全球见过AI真实能力的，只有5000人》 | L5 | https://www.huxiu.com/article/4896682.html | 标题；“全世界大约80亿人，能接触到实验室内部最强AI系统的，可能只有5000人。”；“这些数字是Karpathy自己的粗略估算，但有意思的是，微软、a16z和OpenAI最近的几份报告，也都指向了类似的分化。” | 10/12 09:18（2026-10-12 09:18:59 +08:00） | 正文写“粗略估算”，**标题没有**限定词；“见过AI真实能力的”是标题的改写（原帖：“with access to frontier-grade systems internally”）；同文“Karpathy本人从质疑Agent到加入Anthropic”（自行补充） | 已找到 |
| 金融界（来源“市场资讯”）《Karpathy估计全球约60亿人尚未真正接触大语言模型，约2000万顶尖用户陷入“智能体眩晕”》 | L5 | https://finance.jrj.com.cn/2026/10/12112358677566.shtml | “人工智能研究者Karpathy在社交媒体推文中估计，目前全球真正能够接触最前沿内部AI系统的人可能只有5000人左右” | 10/12 11:23（页面标 2026-10-12 11:23:07 +08:00） | 用“估计”“可能”；但标题把第三层改成“顶尖用户陷入‘智能体眩晕’”——原帖第三层是 “see first-hand that large, complex projects … can now be completed by agents with a prompt”，“vertigo” 在结尾指“a small sliver of professionals” | 已找到 |
| 信鸽中文（转“AI寒武纪”）《Karpathy警告：全人类对AI的认知正在彻底撕裂》 | L5 | https://news.sig.ai/cn/article/cmv4ocebv0004rguf0493aj6k | “全球只有大约5000人真正见识到了前沿AI彻底失控的恐怖能力。”；“这批每个月愿意花200美元以上的技术人员，正在经历强烈的AI眩晕症。” | 10/12 11:10（2026-10-12T03:10:16Z） | 全文未写“粗估/napkin”（grep 无）；**“彻底失控的恐怖能力”原帖无此语**；“每月200美元以上”取自 4 月旧帖，被并入第三层 | 已找到 |
| ChainCatcher（中文）《Karpathy：LLM 能力认知差距正在扩大》；英文版同稿 | L5 | https://www.chaincatcher.com/article/2295540 | “并按接触程度给出分层估算”；“他认为，这一漏斗由问题规模与时间跨度、能力在可数字化且可验证领域更突出，以及免费、付费与内部访问差异共同驱动。” | 10/12 08:34（2026-10-12T00:34:19Z） | 写“分层估算”；较忠实 | 已找到 |
| 链新闻 ABMedia《全球僅 5,000 人真正見識頂尖 AI？Karpathy 警告能力認知差距持續擴大》 | L5 | https://abmedia.io/andrej-karpathy-ai-gap-5k-20m | “他强调，这些数字只是概略推算，并非正式统计。”（原文繁体：“他強調，這些數字只是概略推算，並非正式統計。”） | 10/12 09:57（2026-10-12T01:57:05Z） | 写明“概略推算，并非正式统计”，**但这是作者归给 Karpathy 的话（“他強調”），原帖没有这句**；标题用问号 | 已找到 |
| ic.work《Andrej Karpathy点破大模型认知漏斗：60亿人盲区与5000人眩晕》 | L5 | https://www.ic.work/article/karpathy-reveals-llm-capability-funnel-and-verification-bott | “必须分清的是，Karpathy 给出的 60 亿与 2000 万是他用来勾勒体感分层的餐巾纸速算，并非严谨的统计普查。” | 10/12 06:02（2026-10-11T22:02:29Z） | 写“餐巾纸速算”，只点 60 亿与 2000 万；把 OpenAI 的 ChatGPT 周活写成“截至 2026 年 10 月上旬 … 超过 12 亿”（与官方页一致）；并写 “OpenAI 发布数学手稿库…初始…722篇…撤回 3 篇” | 已找到 |
| FourWeekMBA《Karpathy Puts Internal Frontier AI Access at About 5,000》 | L5 | https://fourweekmba.com/ai-karpathy-puts-internal-frontier-ai-access-at-about-5000/ | 搜索摘要（页面 403，未存档）：“By his own napkin math, about 6 billion people have barely touched LLMs … about 5,000 people have access to frontier-grade systems inside the labs.”；“Karpathy calls his figures napkin math and gives no method or data behind the 6 billion, 1 to 2 billion, 20 million or 5,000 estimates.” | 摘要显示 “8 hours ago”（约 10/12 05:30 前后读取），页面时刻未取得 | 把 “napkin math” 扩展到全部数字；【状态说明：仅搜索摘要】 | 部分支持 |
| 哲见 WISE AI（spke.com）《重新翻出这条四月的推文…》 | L5（全文机器翻译） | https://www.spke.com/zh/ai/article/2402 | “Karpathy称对LLM能力的共同认知差距正在扩大，并以粗略估算描绘一个漏斗：约6B人几乎未接触LLM，约5,000人可内部使用前沿系统。这些数字属个人估算，未经独立核实。” | 10/12 03:10（2026-10-11T19:10:46Z = 原帖首版时刻，早于编辑版 19:12:37Z；见 A1） | 机器摘要写“个人估算，未经独立核实”，译文照录 | 已找到 |
| AIHOT《Karpathy：LLM 能力认知鸿沟扩大为漏斗》 | L5 | https://aihot.news/hot | “他估算全球约 60 亿人（约 75%）几乎未接触过 LLM，约 10 亿至 20 亿人偶尔使用免费层类 ChatGPT 产品，约 2000 万人在数学与编码领域直接使用前沿模型，仅约 5000 人能接触前沿级内部系统。” | 榜单 10/12 13:25 更新 | 全部用“估算”；见 A6 | 已找到 |

未取得：新智元/机器之心/量子位等头部中文科技号的专稿（检索未见）。

## A6 热度

| 说法 | 级 | 一手来源 URL | 读数 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| HN 提交 | L5/L6（社区） | https://news.ycombinator.com/item?id=50047422 | 标题 “The gap in shared understanding of LLM capability is widening”，提交者 albertzeyer，链接 twitter.com/karpathy/status/2109361546505966046，created “2026-10-11T20:20:52.000Z” = 北京 10/12 04:20:52。读数：Algolia points 356（05:25Z）、359（05:32Z）；Firebase score 360、descendants 522（05:25Z）→ 525（05:32Z）；首页（topstories）第 9 位（05:25Z、05:32Z 两次） | — | 北京 13:25 与 13:32；Algolia 树内展开评论数 499 与 Firebase descendants 522 不一致（口径不同：Algolia 嵌套计数可能漏已删/死评）。`a-hn-50047422.json`、`a-hn-50047422-fb.json`、`a-hn-topstories.json`、`a-heat.json` | 已找到 |
| 扫描所报 HN 数字 | — | — | ChatGPT 扫描 279/454、WorkBuddy 57/25；Claude 约 338/513、首页第 8 | — | 均为不同时刻快照，与我读到的 356→360/522→525、第 9 一致地逐步增长；无冲突，未能核实更早快照时点；【状态说明：口径为快照时点差】 | 已找到 |
| AIHOT | L5 | https://aihot.news/hot | “10月12日 13:25 更新 · 按讨论热度排序”，NO.01，“爆”，“↑ 78%”，“+14”，“邵猛、Oran Ge 等 7 个来源·20 位参与者”，“169 热度指数”；首页同条 “1 … Karpathy：LLM 能力认知鸿沟扩大为漏斗 +14 169 热度” | — | 13:30 复读榜单页时更新时刻为 “13:30”，条目仍居首。扫描所报 “142” 与 “12160” 在页面上**均未见**；与 169 同量级的最接近的是 142（可能是较早一次更新的热度指数）；12160 不是热点榜上的指数（来源未查明）。存档 `a-aihot-home.txt`、`a-aihot-hot.txt`；【状态说明：数字见左，扫描两个数字未复现】 | 已找到 |
| AIHOT 摘要所引他人评论 | L6 | https://aihot.news/hot | “Ethan Mollick 转引该推文并补充称，即使这 5000 名内部人员中许多也未充分发挥先进模型能力”；“Oran Ge 则称，大部分人不用大模型是因为大模型对他们没用、没有 ROI” | — | 我未能在 X 上找到对应原帖，**Mollick 的转引未核实** | 未找到一手来源 |
| Reddit r/singularity | L6 | https://www.reddit.com/r/singularity/comments/1x3gsc4/andrej_karpathy_breaks_down_the_capability_gap/ | 帖标题 “Andrej Karpathy breaks down the capability gap”；站内搜索显示 “817 votes·207 comments”，“10h ago”（北京 13:27 前后） | — | 匿名页只显示帖得分；评论得分不显示。存档 `a-reddit-singularity-breaks-down.txt` | 已找到 |
| Reddit r/singularity | L6 | https://www.reddit.com/r/singularity/comments/1x3krgr/karpathy_gap_in_shared_understanding_of_llm/ | “[Karpathy] gap in shared understanding of LLM capability is *widening*…”；“129 votes·13 comments”（北京 13:27 前后）→ 搜索再读 “130 votes·13 comments” | — | `a-reddit-singularity-gap.txt` | 已找到 |
| Reddit r/accelerate | L6 | https://www.reddit.com/r/accelerate/comments/1x3ksgp/karpathy_gap_in_shared_understanding_of_llm/ | 同标题；“131 votes·13 comments” | — | `a-reddit-accelerate-gap.txt` | 已找到 |
| r/OpenAI、r/LocalLLaMA、r/ClaudeAI、r/ChatGPT、r/artificial | — | — | 站内搜索（`karpathy gap understanding funnel`，近一周）未见相关帖 | — | 只搜了各版块站内搜索一次；可能漏帖 | 未找到一手来源 |
| 其它 | L6 | https://daily.dev/posts/karpathy-s-llm-funnel-has-50k-likes-and-a-shrug-hxixci79w | 搜索摘要：“Omarsar0 pushed back on the tweet's 50,000 likes, stating that most of what Karpathy shared describes things practitioners have been doing for over a year with no surprises.” | — | 页面未取，只有搜索摘要，且“50,000 likes”与 X 读数 28,074 不符（该站可能算别的口径或有误），不当事实 | 部分支持 |

## A7 有理有据的质疑（L6，仅作线索，进正文前须回到一手来源）

HN 不公布评论得分（Algolia/Firebase 均无）；“子树”= 该评论回复树含自己的评论数，作为热度参考。评论逐字见 `a-hn-50047422-comments-selected.txt`。

1. 第四层无法核实、也与外界所见产品不符：mccoyb（https://news.ycombinator.com/item?id=50048029，子树 16）：“If the software coming out of OpenAI and Anthropic is what we have to judge, I wonder about the 5000 ... Let's say, for the sake of argument, that the models are some multiplicative factor better on the inside. Doesn't that mean the demos should work?” tripleee（…id=50048344，子树 16）：“And then there's the completely unverifiable scare that there are internal frontier models way beyond anything we've seen”。variety8675（…id=50048627）：“A more cynical view is, trust us we've got really good stuff internally you're not allowed to see, but give us more money”。
2. 第三层“一个 prompt 完成数周/数月项目”与亲身经验不符：skybrian（…id=50048311）、rbehrends（…id=50048689）、tripleee（同上）各以自己的 spec-driven / 重构经验反驳；tripleee 指出成功案例多是“ports and translation where all the rules are already documented”。（对应原帖第三层的 “Building apps, copying apps, translating apps, decompiling apps from binaries”——这些恰是规则明确、可自校验的任务。）
3. 数字口径：komali2（…id=50049451）：“I'm a bit confused by the percentages. 6 billion people aka 75% of the population? … A quarter of the world is under 15”；SamuelAdams（…id=50049794）：“I wonder how accurate this metric is. People may be coming into more contact with LLM generated content, although they may not be aware of it.”——正是“75% 几乎没接触”与“间接曝光”、与官方用户数（A4：Google 称 AI Overviews 月活 over 2.5 billion 等）如何对应的问题。我另做的算术：四层按 8B 总人口只覆盖约 7.5B（见 A1），“付费但非 math&code”的专业人士未单列。
4. access ≠ understanding：socializer（…id=50048390，子树 5）：“it implies that the 6 billion people he's talking about actually have some interest in knowing about the capabilities of LLMs to solve frontier math? This is simply not something they care about or can evaluate.”；expertsage（r/singularity 评论，无得分）：“Even if the majority of people had daily access to cutting-edge frontier models, their opinions wouldn't change, because majority of people won't use AI for math or coding.”；ilovecake1984（…id=50048349，子树 42）：“Nerds (software dev, maths etc) see how good LLMs are at things they care about and assume they will be broadly applicable in future. There’s no reason to think this.”
5. 最大的讨论串（子树 255）其实不针对数字：nsainsbury（…id=50048456）：“It's actually sad how much AI has either not improved life at all or actively made it worse from the perspective of the average person”，主题是日常生活改善；与原帖关系不大，只说明讨论重心。
6. 第三方（L5，非社区）也指出：CellCog：“The top tier, about 5,000 people, has no published number to test against at all.”；PithWire：“The 5,000-person internal tier is doing a lot of work here — unnamed, unverifiable, and conveniently dramatic.”（`a-m-pithwire.txt`）；Karpathy 本人 5 月起在 Anthropic 任职，Anthropic 与 OpenAI 是他所说“内部访问”的主要候选，但原帖不点名（见 A2）。

## 可能的吠点（原文里有、线索摘要没提或说反）

1. “Napkin math” 只标在第一层（6B）那一行开头；多数摘要把它扩展到全部四层（如 CellCog、FourWeekMBA），也有稿子完全不提（信鸽中文）。原帖没有给任何一层的来源（A1）。
2. 原帖没有说他本人是否在第四层；他 2026-05-19 起在 Anthropic（A2）。这使“5,000 人”既不是旁观者的估算，也未交代自己的信息优势。
3. 第四层描述用 “It looks like …” 描述“外界已见的预览”（The external world has seen the preview），并不是对内部系统的逐项证实；人数 5,000 无来源。
4. “722 manuscripts” 是原始数字；OpenAI 10/07 已撤回 3 篇、修订 14 篇，现仓库写 719 篇、其中约 42% 有 Lean 形式化（`a-openai-math-history.md`）。Karpathy 原话是 “may be poring over”，没有说“已验证”。
5. “OpenAI-HF incident from many months ago”：事件起于 2026-07-09，到发帖约 3 个月（A3）；OpenAI 自己的页面称审查 “ongoing”，没有“archeology”之说。
6. 第三层限定为 “math&code” 的专业使用；多数中文稿把它写成“重度用户”或“顶尖用户”（见 A5）。
7. 官方用户数与第一、二层的对照口径各异：OpenAI 周活、Google 月活、AI Overviews 是搜索内曝光、Meta 跨应用月活；官方均未写去重（A4）。AI Overviews “over 2.5 billion monthly active users” 单项数字就大于他写的第二层 1–2B；但这两者衡量的不是同一种“使用”，不能直接比较。
8. 帖子在发出后 1 分 51 秒被编辑过：首版（北京 03:10:46）与现版（03:12:37）仅差一个重复段落（见 A1）；四层数字、“Napkin math” 位置、第三层“decompiling”句、第四层描述在两版中完全相同。X 的编辑历史只能通过 syndication 接口与旧 ID 页面取得，页面本身不显示改动内容。

## 夸大说法实例（把事件写歪的标题或文章）

| 站点 | URL | 原句逐字 | 发布时间（北京） | 发布方 | 怎么歪的 |
|---|---|---|---|---|---|
| 虎嗅 | https://www.huxiu.com/article/4896682.html | 标题“Karpathy：全球见过AI真实能力的，只有5000人” | 2026-10-12 09:18 | 虎嗅（转夕小瑶科技说） | 原帖是“有内部访问权的约 5,000 人”，标题改成“见过AI真实能力”；正文有“粗略估算”，标题无 |
| 信鸽中文 | https://news.sig.ai/cn/article/cmv4ocebv0004rguf0493aj6k | “全球只有大约5000人真正见识到了前沿AI彻底失控的恐怖能力。” | 2026-10-12 11:10 | 信鸽中文（转 AI寒武纪） | 加入“彻底失控”“恐怖”，原帖无此语；全文不提“估算” |
| 金融界 | https://finance.jrj.com.cn/2026/10/12112358677566.shtml | 标题“…约2000万顶尖用户陷入“智能体眩晕”” | 2026-10-12 11:23 | 金融界（来源：市场资讯） | “顶尖用户”“陷入”是改写；原帖：第三层“see first-hand”，“vertigo”指结尾的 “a small sliver of professionals” |
| Fathom | https://www.fathom.news/karpathy-warns-the-ai-capability-gap/ | “describing a narrowing consensus that still leaves most observers outside it.” | 2026-10-12 04:17 | Fathom（Bibhu Pattnaik） | 把“漏斗”读成共识收窄/观点趋同，与原帖相反（原帖：认知差距在扩大） |
| bdnews24 | https://bdnews24.net/en/ai-chatbots-llm/andrej-karpathy-cognitive-chasm-llm-capabilities | “Only around 5,000 specialists inside leading AI labs have access to experimental, self-coordinating multi-agent systems.” | 2026-10-12 03:31 | bdnews24（Alexey Smirnov） | 第四层写成既定事实；“specialists”“experimental, self-coordinating” 是增补 |
| 链新闻 ABMedia | https://abmedia.io/andrej-karpathy-ai-gap-5k-20m | “他强调，这些数字只是概略推算，并非正式统计。” | 2026-10-12 09:57 | ABMedia（Neo） | 方向对，但“他强调”是作者替他加的话，原帖未说；他只在第一层写了 Napkin math |

没有找到把 5,000 写成“懂 AI 的人”或把数字写成调查数据的稿件；CellCog、PithWire、FourWeekMBA 反而明确写“无来源/估算”。ic.work 写到“路透研究院 2025 年针对 6 国的调查显示，每周使用 ChatGPT 的受访者仅占 22%”，那是 ic.work 自己补充的外部数据，不是 Karpathy 的数字。

## 扫描说法勘误

1. HN 名次：Claude 取数时写“首页第 8”，我 05:25Z 与 05:32Z 读到第 9；分数 356→360，评论 522→525（Firebase `descendants`）。ChatGPT 扫描 279/454、WorkBuddy 57/25 为较早快照，与增长趋势相容，未能核实其时点。
2. AIHOT：页面上显示的是“热度指数 169”（10/12 13:25 更新），首页热点榜第 1；扫描所报 142 与 12160 在页面上均未见到。
3. “Napkin math” 只在第一层 6B 那一行开头：与已知情况一致；我另核了其余各层，没有第二处限定词。
4. “722 manuscripts”：与 1006 期一致，但 openai/math 现写 719（10/07 撤回 3 篇；10/08 merge 提交）。若正文引用 Karpathy 这句，需注明“722 是 10/06 初始数字”。
5. 引用的 4 月旧帖时刻：与已知情况一致（2026-04-09 20:10:52 UTC）。
6. 帖子有 “Last edited” 标记：已知情况只记了 19:12:37Z。原始发帖是 19:10:46Z（北京 03:10:46），19:12:37Z 是编辑版；两版仅差一个重复段落（见 A1）。
7. 已知情况称“第三层”含 “Building apps, copying apps, translating apps, decompiling apps from binaries...”：已核，逐字一致；原帖不含 ArtCraft 或任何具体项目。
8. 官方用户数：已知情况未给；A4 已补全，其中 “Gemini 应用 1B 月活（8/11）”“AI Overviews 2.5B（5/19）”“ChatGPT 1.2B 周活（10/05、10/07 官方页）”均有一手。扫描类材料中出现的“OpenAI 9/8 称 1B 周活”是**OpenAI 全产品**口径，不是 ChatGPT 单品；TechnologyChecker 等统计站称 “OpenAI told developers on 29 September 2026 that ChatGPT reaches 1.2 billion weekly users” 仅见二手，OpenAI 官方 10/05、10/07 页面一手证实该数字。
