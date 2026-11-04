# 1103 期取证：C 组（速览一）

执行方：派工 Claude 调度的子代理，模型 ID claude-haiku-5-5。取证时点：北京时间 2026-11-04 20:46–20:52。本组不截图，只存档与摘句。文本存档见 `sources/c-*.txt`；HTML、PDF 原件只存本地，不入库。

## 速览条目（一句事实草稿，照原文措辞保留限定词）

**C1 Apple Watch 可回看刚才的对话（开发者测试版）**
- 草稿：苹果称，Apple Watch Series 12 与 Ultra 4 上的 Live Rewind 与 Siri Recap “will be available in beta later this year with watchOS 27.3”，用户逐项开启后使用；双击表冠可看此前 15 秒对话的文字片段，目前仅英语。“27.3 开发者测试版已推出”只见于 The Verge、MacRumors（L4，据其报道）；苹果自己 11/3 的文字是将来时。
- 一手：Apple Support 148354（Published Date: November 03, 2026，L1）。
- 发布时刻：官方仅给日期 2026-11-03（官方未给时刻）；The Verge 2026-11-03 18:31 UTC（北京 2026-11-04 02:31，L4）；MacRumors 2026-11-03 10:35 PST（北京 2026-11-04 02:35，L4）。
- 限定词原句（Apple）：“These features will be available in beta later this year with watchOS 27.3.”；“Each Audio Intelligence feature is opt-in.”；“Live Rewind and Siri Recap are not available for users under 18.”；“Siri Recap and Live Rewind will be available in English to start and will not initially be available in the EU. Siri Recap and Live Rewind are not available in China.”
- 热度：HN 50357683，66 分，34 评（北京 2026-11-04 04:25 发帖，采样时点同上）。

**C2 Siri AI 向所有开发者开放**
- 草稿：苹果称，开发者预览（Developer Preview）让任何开发者把 App 接入 Siri AI；预览随 iOS、iPadOS、macOS、visionOS 27.3 测试版提供，App Store 提交“在春季”开放，上架要等。
- 一手：Apple Developer 新闻 “Siri AI is now open to all developers”（L1，https://developer.apple.com/news/?id=gdk2umrf）；Apple 开发者文档 Siri AI Developer Preview（L1，同 ID 的 JSON 版存档）。
- 发布时刻：官方仅给日期 2026-11-03（官方未给时刻）；The Verge 2026-11-03 20:16 UTC（北京 2026-11-04 04:16，L4）。
- 限定词原句：“Starting today with the iOS 27.3, iPadOS 27.3, macOS 27.3, and visionOS 27.3 betas, a new Developer Preview enables any developer to integrate their app with Siri AI”；“with submission to the App Store opening up in the spring”；“This preview provides early access before first shipping to customers in a few months”；“Siri AI is currently unavailable in the EU in iOS, iPadOS, and watchOS.”；“As you explore early, we will follow up with more details about the terms and usage policies”。
- 热度：HN 50359173（4 分，0 评，北京 2026-11-04 06:31）；HN 50359242（1 分，0 评，北京 06:38）；HN 50363502（2 分，0 评，北京 18:17）。HN 无高热帖。

**C3 Google 学校账号默认关闭 Gemini 与搜索**
- 草稿：谷歌称，K-12 学生账号上的 Gemini、Gemini Notebook、Gemini in Classroom 与 Google Search 默认完全关闭，直到学校管理员明确开启；细节“未来几个月”提供。
- 一手：Google 官方博客 “Designing responsible AI for learning”（L1，https://blog.google/products-and-platforms/products/education/ai-learning-era-updates-2026/）。
- 发布时刻：官方仅给日期 2026-11-03（官方未给时刻）。
- 限定词原句：“Access to these products will be entirely off for K-12 student accounts until school administrators explicitly turn them on.”；“We’ll share more specifics with schools in the coming months ahead of the changes rolling out”（未给生效日期）。NYT（L4）另写“early next year”，官方原文无此词，见证据表。
- 热度：HN 50353192，632 分，436 评（Algolia 搜索返回值）；`barking heat` 取到嵌套评论 410 条；北京 2026-11-03 23:25 发帖。该 HN 条目链接的是 NYT 文章（见下）。

## 证据表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1：Apple Watch 双击表冠可回看此前 15 秒对话 | L1 | https://support.apple.com/en-us/148354 | “Live Rewind1 recalls what was just said in the last 15 seconds by showing a text snippet on your Apple Watch when you double-press the Digital Crown” | 无（C 组不截图） | 仅 Apple Watch Series 12、Ultra 4；英语；文字片段，不是录音 | 已找到 |
| C1：随 watchOS 27.3 的开发者测试版提供 | L1（将来时）+ L4（已推出） | Apple 支持页同上；The Verge https://www.theverge.com/tech/1025002/apple-watch-audio-intelligence-watchos-beta ；MacRumors https://www.macrumors.com/2026/11/03/watchos-27-3-audio-intelligence/ | Apple：“These features will be available in beta later this year with watchOS 27.3.”；Verge（据 Verge 报道）：“available in the new watchOS 27.3 developer beta rolling out now.”；MacRumors：“With the first beta of watchOS 27.3, Apple is introducing…” | 无 | 苹果 11/3 支持页用将来时“later this year”；“已推出”只来自 The Verge、MacRumors。Apple 开发者 watchOS 27.3 Beta Release Notes（`sources/c-c1-apple-watchos-27-3-beta-notes-json.json`）是 SDK 变更说明，正文未提 Live Rewind、Siri Recap 或 Audio Intelligence | 部分支持（“开发者测试版”已出为 L4 报道；苹果原文是“将随 27.3 以 beta 提供”） |
| C1：需用户主动开启 | L1 | 同上 | “Each Audio Intelligence feature is opt-in. And for Live Rewind and Siri Recap, you decide when and where each feature is active.” | 无 | 开启后 Siri Recap 会在后台持续“ambiently takes notes on your conversations throughout your day”（同页原句，需保留） | 已找到 |
| C1：不存录音，原始音频由芯片隔离组件处理 | L1 | 同上（Apple Audio Intelligence 隐私概览 PDF 存本地：`sources/c-c1-apple-privacy-pdf.pdf`，文本 `sources/c-c1-apple-privacy-pdf.txt`） | “Audio is not recorded or stored. Audio Intelligence features don't create audio recordings.”；“Raw audio is never accessible.”；“Once processed, this data is immediately deleted.” | 无 | 苹果称；S11 芯片 Secure Exclave | 已找到 |
| C1：文字片段与摘要端到端加密 | L1 | 同上 | “Siri Recap summaries and saved Live Rewind text snippets stored in the Siri app are end-to-end encrypted when synced through iCloud”；“Live Rewind 15-second text snippets disappear permanently … approximately 30 seconds after the Apple Watch display dims, unless you save them to the Siri app or tap ‘Ask Siri’.” | 无 | 限定：仅保存后、需 iCloud 与设备密码（Verge 引苹果隐私概览：“with two-factor authentication and have a device passcode”） | 已找到 |
| C1：限机型、语言、地区与年龄 | L1 | 同上 | “Apple Watch Series 12, Apple Watch Ultra 4”；“iPhone 16 … or later”（支持页机型列表）；“Requires Apple Intelligence and Siri AI (Beta)”；“Available in English with more languages to follow”；“not available in China”；“not available for users under 18” | 无 | 另需 iPhone 16 或更新机型；欧盟暂不可用 | 已找到（年龄限制与 MacRumors 不符，见勘误） |
| C1：Apple 11/3 公告 | L1 | 同上（Published Date: November 03, 2026）；Apple 开发者新闻 https://developer.apple.com/news/?id=gdk2umrf（只讲 Siri AI，不讲 Audio Intelligence）；Apple 开发者 watchOS 27.3 Beta Release Notes（JSON 存档 `sources/c-c1-apple-watchos-27-3-beta-notes-json.json`） | 见上 | 无 | 支持页日期 2026-11-03，无时刻 | 部分支持（有 Apple 支持页公告；未找到 Apple 开发者页专门公告 Audio Intelligence；watchOS 27.3 beta 的 Apple 开发者说明未提及这两项功能） |
| C1：Apple 9 月功能说明 | L1 | https://www.apple.com/newsroom/2026/09/siri-ai-a-profoundly-more-capable-and-personal-assistant-is-here/ | “Siri Recap and Live Rewind arrive in beta later this year.”；“Siri AI begins rolling out today in beta in English” | 无 | 2026-09-14（Apple 新闻稿日期） | 已找到 |
| C2：Siri AI 向所有开发者开放 | L1 | https://developer.apple.com/news/?id=gdk2umrf | “Siri AI is now open to all developers”；“a new Developer Preview enables any developer to integrate their app with Siri AI” | 无 | 2026-11-03，官方未给时刻；对象为开发者预览 | 已找到 |
| C2：27.3 测试版中任何开发者可用 App Intents 暴露操作，用 App Skills 做多步组合 | L1 | 同上；Apple 开发者文档 https://developer.apple.com/documentation/appintents/siri-ai-developer-preview | “Starting today with the iOS 27.3, iPadOS 27.3, macOS 27.3, and visionOS 27.3 betas”；“we’re also introducing skills to let you teach Siri AI how to combine several actions into a single experience”；文档：“The Siri AI Developer Preview is available in iOS, iPadOS, macOS, and visionOS on devices that support Apple Intelligence.” | 无 | 平台仅 iOS、iPadOS、macOS、visionOS；官方文档未列 watchOS；预览期间 API 与功能可能变化 | 部分支持（任何开发者可用属实，但 watchOS 27.3 不在开发者预览列表） |
| C2：上架要等；App Store 提交 2027 年春开放 | L1 | 同上 | “with submission to the App Store opening up in the spring”；“This preview provides early access before first shipping to customers in a few months” | 无 | 官方未写年份。“2027”为推断：Apple 9 月稿把 iOS 27 系列称为“the 2027 software releases” | 部分支持（“春”为官方原文，“2027”为推断） |
| C2：预览功能不对 TestFlight 外部测试与 App Store 版本开放（扫描未提） | L1 | https://developer.apple.com/documentation/appintents/siri-ai-developer-preview | “The Siri AI Developer Preview isn’t available to external testers in TestFlight or to versions of your app that you publish on the App Store.” | 无 | 只对内部测试者（TestFlight）可用 | 已找到 |
| C2：欧盟暂不可用 | L1 | https://developer.apple.com/news/?id=gdk2umrf | “Siri AI is currently unavailable in the EU in iOS, iPadOS, and watchOS.” | 无 | 另：“Apple is working constructively with the EU in an attempt to find a path forward.” | 已找到 |
| C2：条款与使用政策待公布（扫描未提） | L1 | 同 gdk2umrf | “As you explore early, we will follow up with more details about the terms and usage policies for experiences enabled through Siri AI.” | 无 | 限定：上架前的使用条款未定 | 已找到 |
| C3：K-12 学生账号默认关闭 Gemini 与搜索，管理员开启 | L1 | https://blog.google/products-and-platforms/products/education/ai-learning-era-updates-2026/ | “Access to these products will be entirely off for K-12 student accounts until school administrators explicitly turn them on.” | 无 | 2026-11-03；产品为 Gemini、Gemini Notebook、Gemini in Classroom、Google Search（原文四项） | 已找到 |
| C3：关闭清单含 Google Search | L1 | 同上 | “we are now updating our approach to controls for Gemini, Gemini Notebook, Gemini in Classroom, and Google Search.” | 无 | 原文只写“Google Search”，不限定搜索形态；同文另写 Search 中为学生账号做 Guided Learning，先在美国部分学校试点（“pilot program for Search in select U.S. schools later this month”） | 已找到 |
| C3：细节“未来几个月”提供 | L1 | 同上 | “We’ll share more specifics with schools in the coming months ahead of the changes rolling out so they have time to prepare and enable a smooth transition.” | 无 | 未给生效日期 | 部分支持（细节提供时间有原句，生效时间未给） |
| C3：NYT 报道的生效时间与“拒绝 AI 的学校”口径（扫描未提） | L4 | https://www.nytimes.com/2026/11/03/us/google-search-schools-restrict-ai.html （NYT 页面为预览态，正文被截断，“Thank you for your patience while we verify access”；未绕过） ；标题 “Google Will Turn Off Search for Schools That Reject A.I.”；发布 2026-11-03T14:00:07Z（北京 2026-11-03 22:00） | “Google announced Tuesday that early next year it would turn off artificial intelligence tools in its popular software for K-12 students, unless school administrators explicitly choose to make such tools available.”；“Schools that choose to restrict A.I. entirely will also lose access to Google search.”；Phillips（谷歌教育副总裁）：“If a school has banned A.I., he added, students ‘will not have access to search’” | 无 | 据 NYT 报道（L4）。“early next year”为 NYT 用语，Google 官方博客原文只写“in the coming months”，未写时间。Phillips 引语出自 NYT 文中，不在官方博客中 | 部分支持（“默认关闭、管理员开启”一致；“early next year”官方未证实；“拒绝 AI 的学校失去搜索”出自 NYT 与 Phillips 引语） |

## 扫描说法勘误

1. **C1 Apple 11/3 公告**：扫描未说明出处。已找到 Apple 支持页 148354（Published Date: November 03, 2026）。Apple 开发者新闻 11/3 讲的是 Siri AI，不讲 Audio Intelligence。
2. **C1 年龄限制**：未在扫描里出现。Apple 支持页写“not available for users under 18”；MacRumors 写“not available to users under 13. Parental approval is required for Child Accounts under 18.”。两者不一致，以 Apple 原文为准。
3. **C1 限制条件补充**：扫描只写“指定新款 Watch、英语”。原文还要求 iPhone 16 或更新机型并启用 Apple Intelligence 与 Siri AI（Beta）；欧盟暂不可用；中国不可用（Apple 支持页原句）。
4. **C2 平台范围**：扫描“27.3 beta 中任何开发者可用”需限定为 iOS、iPadOS、macOS、visionOS 27.3 测试版。watchOS 不在开发者预览列表；Siri AI 用户测试版则包含 watchOS 27.3。
5. **C2 “2027 年春”**：官方原文只写 “in the spring”，无年份。“2027”为推断（Apple 9 月稿称 iOS 27 系列为“2027 software releases”）。正文若写年份，须注明是推断。
6. **C2 上架限制**：扫描未提 Apple 开发者文档原句“isn’t available to external testers in TestFlight or to versions of your app that you publish on the App Store”。这一条须进速览的限定词范围内判断。
7. **C3 对象**：扫描写“学校账号”，官方原文是“K-12 student accounts”（K-12 学生账号）。
8. **C3 清单**：扫描写“Gemini 与 Google Search 等”。官方清单恰为四项：Gemini、Gemini Notebook、Gemini in Classroom、Google Search，无“等”字。
9. **C3 HN 数字**：扫描称“约 622 分 433 评（帖 ID 未核）”。HN 50353192 实测为 632 分、436 评（Algolia，北京约 20:50 采样），`barking heat` 取到嵌套评论 410 条。数字随采样时点变化，正文如引用须注明时点。
10. **C3 标题与原文口径**：HN 50353192 的链接是 NYT 文章（https://www.nytimes.com/2026/11/03/us/google-search-schools-restrict-ai.html），HN 标题由提交者改写（ceejayoz），与 NYT 标题 “Google Will Turn Off Search for Schools That Reject A.I.” 只差大小写与标点。另一条 HN 50353905（46 分，16 评）直接用 NYT 标题。NYT 标题的主要依据是 Phillips 的引语（“If a school has banned A.I., … students ‘will not have access to search’”），官方博客没有“拒绝 AI 的学校”措辞。因此这不是社区自造的夸大，而是 L4 媒体标题，需要按“据 NYT 报道”署名，并与官方“K-12 学生账号默认完全关闭、管理员开启”并列；它的“reject”口径与官方“default off”不是同一个范围（前者针对学校态度，后者针对所有 K-12 学生账号），正文不能只用标题写法。
11. **C1 HN 数字**：扫描称“66 分 34 评”，与实测一致（50357683，北京 2026-11-04 04:25 发帖）。
12. **C2 HN**：扫描未给 HN 条目。本组以 Algolia 搜索“Siri AI”得到的 HN 条目作为热度记录（50359173、50359242、50363502），热度很低。
13. **C3 生效时间**：扫描称“细节‘未来几个月’提供”，官方一致。NYT 另写“early next year”，官方原文没有这个时间。两者不一致，以官方为准，NYT 的说法须标“据 NYT 报道”。
14. **C1 “已推出”**：扫描称 watchOS 27.3 开发者测试版“提供” Live Rewind 与 Siri Recap。苹果支持页（11/3）用的是将来时“will be available in beta later this year with watchOS 27.3”；“27.3 测试版已推出”只见于 The Verge 与 MacRumors（L4）。

## 未找到 / 失败项

- C1 Apple 开发者页面中，专门讲 Audio Intelligence 的 11/3 公告：未找到（支持页 148354 有，开发者页无）。
- C1 Reddit 热度：未取样（派工要求的是 `barking heat`；本组只跑 HN）。
- C2 Reddit 热度：同上。
- C2 Apple 开发者文档 HTML 版（`sources/c-c2-apple-dev-siri-ai-preview.html`）只抓到标题，是 JS 渲染页；内容以同 ID 的 JSON 版（`sources/c-c2-apple-dev-siri-ai-preview-json.json`）为准。
- C3 Google 博客 HTML 页面全文可取，无失败。

## 备注（供 Claude 判断，不进速览）

- Apple 支持页写明：“Siri Recap sends an encrypted, condensed version of the conversation transcript from your iPhone to Private Cloud Compute.”（转写发生在 iPhone 的 Secure Exclave 内，摘要由 PCC 生成）。“不存录音”成立，但“会生成转写再压缩摘要”是另一层事实，可能用于“可能的吠点”判断。
- MacRumors 写 Live Rewind “can show you a transcript of the last 15 seconds”；Apple 写 “text snippet”。两种用词都在来源里。
- Verge 标题用 “always-on”，正文写 “ambient listening”；Apple 写 “opt-in”。这是媒体框架与官方用词的差别，正文只用 Apple 原词即可。
