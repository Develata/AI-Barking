# 1116 期 B 组取证：苹果给 Siri AI 与 Apple Intelligence 设每日额度，按 iCloud+ 档位加倍

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`），北京时间（UTC+8）2026-11-17 19:24–20:00。文件前缀 `b-`；抓取与截图明细见 `capture-log-B.md`。所有存档在 `docs/2611/1116/sources/b-*`，截图在 `docs/2611/1116/images/13–24-*.png`。

读法：级别按 `EDITORIAL.md` 事实分级；状态只用 已找到 / 部分支持 / 与说法不符 / 未找到一手来源。引文一律逐字摘自本地存档（`.txt`，苹果页面存档含 `.headers`）。**Apple 官方支持页只给日期不给时刻，一律写“官方未给时刻”。**

---

## 〇、先看：与“已知情况”和扫描说法不一致或需补充之处（扫描说法勘误前置摘要，细则见第五节）

1. **127901 不是 11-16 才有的新页。** Wayback 快照显示该页自 2026-09-09 起就存在，标题当时是 “Learn about Apple Intelligence usage limits”，正文写 “In the future, Apple will implement usage limits … Increased access to such features will be available for a fee.” 11-16 页脚日期是改版（标题改为 “Learn about iCloud+ with Apple Intelligence and Siri AI”，加入倍数表）。“苹果第一次设每日额度”的说法要谨慎：额度的存在 6 月 8 日 WWDC 新闻稿就写了，9 月 9 日支持页已写“将收费”；11 月新增的是“按档位的倍数表 + 27.3 起”。
2. **“27.3 测试版实施”只有媒体说法（L4），官方只有 “starting with the 27.3 release”。** 苹果开发者 iOS & iPadOS 27.3 Beta 2 发行说明（JSON 存档）完全没有提到使用额度。
3. **中文页（zh-cn / zh-tw / zh-hk）127901 目前仍是 9 月旧文**（无倍数表、无 iCloud+ 档位，只写“需要付费”），与英文页不同步；截至抓取时刻（11-17 19:24）。
4. **中国大陆**：同一英文页两句“not available in”并列时，China 只出现在 Apple Intelligence 那句，**Siri AI 那句没有 China**；但另一官方页 127893 写 “Siri AI will not currently work if your Apple Account region is in mainland China.”，苹果中国 10-31 新闻稿又写 “本次更新不包含 Siri AI”。“China”与“China mainland/中国大陆”措辞不同，照官方原样记，不自行等同。
5. **HN 无社区讨论**：`50520857` 仅 2 分、0 评论（11-17 02:16 北京时间提交）；另有一次提交 `50526863`（1 分、0 评论，11-17 11:02）。Reddit 搜索被登录墙挡住，未取。社区质疑改取 MacRumors 论坛高赞评论（L6）。
6. 页面**未写**“Standard Device Limit”的具体次数（已确认；多家媒体亦称苹果未公布）。

---

## 一、逐条取证表

### B1　Apple Support 127901 英文页

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1-1 页面标题与发布日期 | L1 | https://support.apple.com/en-us/127901 | 标题 “Learn about iCloud+ with Apple Intelligence and Siri AI”；页脚 “Published Date: November 16, 2026” | `13-support-intro-categories.png`、`17-support-region-fineprint.png`（含 Published Date） | 官方未给时刻。HTTP 头 Last-Modified 为 “Tue, 17 Nov 2026 02:30:00 GMT”（=北京 11-17 10:30），是 CDN/缓存时间，**不当作发布时间**；同组 zh-cn 页的 Last-Modified 却是 11-16 16:30 GMT，而其正文日期是 09-09，可见该头不可靠。存档：`b-apple-127901-en.html/.txt/.headers` | 已找到 |
| B1-2 首段：用量上限与升级 | L1 | 同上 | “Some Apple Intelligence and Siri AI features rely on powerful server-side models and have daily usage limits to ensure a high-quality, reliable experience for everyone. To increase your limits, you can upgrade your iCloud+ plan.” | `13` | — | 已找到 |
| B1-3 免费、27.3 | L1 | 同上 | “Starting with the 2027 software releases, Siri AI and Apple Intelligence features are available for free for users with a supported device set to a supported language. Some features rely on powerful server models and have limits where increased access is available with eligible iCloud+ plans, starting with the 27.3 release.” | `13` | “starting with the 27.3 release” 可读作“加量从 27.3 起”，也可读作“限额与加量从 27.3 起”；原文未说 beta 或正式版、未给日期（见 B3） | 已找到 |
| B1-4 四类功能 | L1 | 同上 | “Siri AI” / “Images: Intelligent Photo Editing Tools including Clean Up, Extend, and Spatial Reframing, and Image Playground” / “Workflows and automation: AFM Cloud and Cloud Pro models in Shortcuts” / “Third-party apps using Apple Foundation Models in Private Cloud Compute”；前文 “These include, but aren't limited to” | `13` | 列表**不是穷尽**（“aren't limited to”）。9 月旧版写 “AFM 3 Cloud and AFM 3 Cloud Pro”，11 月版去掉了 “3” | 已找到 |
| B1-5 Limits may vary | L1 | 同上 | “Apple implements daily usage limits on these feature categories and other features to ensure a high-quality, reliable experience for everyone. Limits may vary by feature category, request complexity, system demand, system policies, and other factors.” | `13` | “implements”现在时（9 月旧版为 “In the future, Apple will implement”） | 已找到 |
| B1-6 Home App 摄像头 | L1 | 同上 | “Some Apple Intelligence features in the Home app, such as video summaries, are available for a limited number of cameras: up to 1 camera in the 2 TB plan, up to 2 cameras in the 6 TB plan, and up to 5 cameras in the 12 TB plan.” | `13` | 页面**只列 2/6/12 TB**；Free、50 GB、200 GB 是否为 0 台页面未写（推断为无，未核实） | 已找到 |
| B1-7 Siri AI 额度 | L1 | 同上 | “Siri AI includes a generous daily usage limit. If you reach it, you will be notified, and you can still make everyday Siri requests like "Set a timer for 3 minutes" or "Text Mark that Annika's party is at 123 Main Street.”” / “If you reach your limit, advanced requests are paused until your limit resets or you upgrade to an eligible iCloud+ plan. Advanced requests are more complex, like "Text Mark the address for Annika's party, and tell him what time we need to leave from home to make it on time" or "Add a summary of my San Francisco trip note to this email draft."” | `14-support-siri-limit.png` | “generous”是苹果自评，无数字；“advanced requests are paused”，日常请求不停 | 已找到 |
| B1-8 按设备计、重置 | L1 | 同上 | “When you hit your daily usage limit for a particular feature category, you’ll be notified. Each device has its own daily usage limit per feature category. Once you reach a limit, you can wait until your daily usage limit resets on that device or immediately increase your limit by upgrading to an eligible iCloud+ plan.” / “All feature categories reset daily at the same time, and limits do not roll over to subsequent days. Your limit resets around midnight in your current time zone except:” 三例外：出国按 Apple Account 地区所在国中部时区午夜；家庭共享按购买者时区；未登录则 “around 24 hours after you started using that feature category” | `15-support-device-reset.png` | 额度按“功能类别”分别计 | 已找到 |
| B1-9 加量与共享 | L1 | 同上 | “For more access, upgrade to an iCloud+ plan with at least 200 GB of storage. After you reach your daily usage limit on a device, you gain access to an increased usage limit that is shared by all devices signed in to your Apple Account and across any Family members sharing your plan, on a first-come, first-served basis.” | `16-support-multiplier-table.png` | 先耗本设备自己的额度，再用共享池；共享池由账号下所有设备与家庭成员“先到先得” | 已找到 |
| B1-10 倍数表 | L1 | 同上 | 表题 “Total Apple Intelligence and Siri AI daily usage limits with iCloud+”；列 Free / iCloud+ with 50 GB / 200 GB / 2 TB / 6 TB / 12 TB；行 “Select Apple Intelligence Features and Siri AI”：Standard Device Limit / Standard Device Limit / Up to 2X Standard Device Limit / Up to 5X … / Up to 10X … / Up to 20X Standard Device Limit；行 “Shortcuts using AFM Cloud Pro Model”：- / - / Access to AFM Cloud Pro Model / Up to 5X Access / Up to 10X Access / Up to 20X Access | `16`（全 6 列、两行完整；此图用 930 CSS px 宽 + DPR 1.5，原因见 capture-log：700 px 宽时第 6 列 12 TB 被页面截掉） | 全是 “Up to”；基数 “Standard Device Limit” 未定义数值；表题写 “Total … limits”，而正文写 “increased usage limit that is shared”，**2X 是“总量为 2 倍”还是“再加 1 份”页面未明说**（iDropNews 称是推测，见 B5，L5） | 已找到 |
| B1-11 地区限制 | L1 | 同上 | “Increased access to select Apple Intelligence features with iCloud+ is subject to Apple Intelligence availability and is not available in China, Brazil, the EU, UK, or South Korea. Increased access to Siri AI with iCloud+ is subject to Siri AI availability and is not available in Brazil, the EU, UK, or South Korea.” 另：“Supported camera counts, the standard amount of Apple Intelligence and Siri AI access, and other iCloud+ plan features may change over time.” | `17-support-region-fineprint.png` | 第一句列 China，第二句（Siri AI）**不列 China**；但 Siri AI 自身在中国大陆账号区不可用（见 B2） | 已找到 |
| B1-12 页面是否给具体次数 | L1 | 同上 | 全文检索：无“次/requests per day/数字额度”，仅有倍数、相机台数、“3 minutes”示例 | `13`–`17` | 同类媒体亦称未公布（9to5Mac：“Apple isn’t disclosing specific limit numbers.”；AppleInsider：“What Apple is not making clear is precisely what these figures actually mean.”） | 已找到（确认页面未写次数） |

### B2　中文页与中国区

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B2-1 zh-cn 127901 | L1 | https://support.apple.com/zh-cn/127901 | 标题 “了解 Apple 智能使用限制”；“未来，Apple 将对这些以及其他 Apple 智能和 Siri AI 功能实施使用限制……要增加对此类功能的使用权限，需要付费。”；页脚 “发布日期： 2026 年 09 月 09 日” | `19-zhcn-support-sept-text.png` | 存在，但**仍是 9 月旧文**：无倍数表、不提 iCloud+ 加量、不提 27.3、不提地区排除；列表仍写 “AFM 3 Cloud 和 AFM 3 Cloud Pro”。截至 11-17 19:24 | 已找到（说明：与英文页不同步） |
| B2-2 zh-tw / zh-hk | L1 | https://support.apple.com/zh-tw/127901、https://support.apple.com/zh-hk/127901 | zh-tw “發佈日期： 2026 年 09 月 10 日”，“付費將能使用更多此類功能。”；zh-hk “發佈日期： 2026 年 09 月 10 日”，“如要增加使用此等功能的權限，用户將需要付費。” | 未截（与 zh-cn 同构，存档足以证） | 同上，9 月旧文 | 已找到 |
| B2-3 Siri AI 在中国大陆 | L1 | https://support.apple.com/en-us/127893（“How to get Siri AI”，页脚 “Published Date: November 12, 2026”） | “For China mainland residents” / “Siri AI will not currently work if your Apple Account region is in mainland China.” | `20-siri-ai-availability-china.png`（含同页脚注与日期） | 条件是“Apple Account region”在中国大陆。该页同段脚注：“Certain Apple Intelligence features that rely on server-side models are subject to daily usage limits, including but not limited to Siri AI, … Expanded access to such features will be available for a fee in the future.”（11-12 版本仍写“将来收费”）。zh-cn 的 127893 页存档为未翻译英文（`b-apple-127893-zhcn.txt`） | 已找到 |
| B2-4 Apple Intelligence 在中国大陆 | L1 | https://support.apple.com/en-us/121115（“Get started with the next generation of Apple Intelligence”，“Published Date: November 05, 2026”） | “For China residents” / “Apple Intelligence features will not currently work for supported devices purchased in China mainland. For supported devices purchased outside of China mainland, Apple Intelligence features will not currently work if you are in China mainland and if your Apple Account Country or Region is also in China mainland.” / “New Apple Intelligence features will not be available in China while Apple works through regulatory requirements.”；同页 “Feature Usage Limits … Expanded access to such features will be available for a fee in the future.” | 未截（文字存档） | 11-05 页仍写“目前不可用”；晚于下面 10-31 新闻稿，两者并存，**不一致，原样记录**。zh-cn 版 121115 同文（“目前，在中国大陆购买的支持设备无法使用 Apple Intelligence 功能”） | 已找到 |
| B2-5 苹果中国 10-31 新闻稿 | L1 | https://www.apple.com.cn/newsroom/2026/10/apple-intelligence-features-are-coming-to-users-in-china/ | “这一系列 Apple 智能功能将自 11 月 4 日起随最新的 iOS 开发者 Beta 版开放测试，并将于 12 月 iOS 27.3 正式发布时面向中国用户推出。其他平台的支持将在获得必要审批后推出。”；“本次更新不包含 Siri AI 及 Apple 在 WWDC 2026 上宣布的其他下一代 Apple 智能功能。”；“Apple 与阿里巴巴达成合作，在部分 Apple 智能体验中加入对千问的支持” | `21-cn-newsroom-no-siri-ai.png`（拼接 3 段） | 日期 “2026 年 10 月 31 日”，官方未给时刻。**大陆路径是“iPhone + 千问”的另一套 Apple 智能，不含 Siri AI；该稿未谈每日额度或 iCloud+ 加量**（全文检索无 “额度/使用限制”） | 已找到 |
| B2-6 中国大陆能否加量 | L1 | 127901 en（B1-11）+ 127893 + 10-31 新闻稿 | 综合：Apple Intelligence 的 iCloud+ 加量 “is not available in China”；Siri AI 在大陆账号区本就 “will not currently work” | `17`、`20`、`21` | 推断（未核实）：大陆用户当下既用不到 Siri AI，也买不到加量；12 月大陆版 Apple 智能是否有额度/怎么计，苹果文档**未写** | 部分支持 |

### B3　“27.3 测试版已上线 / 何时生效”

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B3-1 苹果官方对时点的唯一措辞 | L1 | 127901 en | “… increased access is available with eligible iCloud+ plans, starting with the 27.3 release.” | `13` | 未写 beta/正式版、无日期 | 已找到 |
| B3-2 开发者发行说明是否提及额度 | L1 | https://developer.apple.com/documentation/ios-ipados-release-notes/ios-ipados-27_3-release-notes（Beta 2，通过 `…/tutorials/data/…release-notes.json` 取到全文）；https://developer.apple.com/news/releases/?id=11162026a（“iOS 27.3 beta 2 (24C5049d)”，“November 16, 2026”） | 全文检索 “usage limit / Apple Intelligence / iCloud”：只在 StoreKit 条目有无关的 iCloud 账号描述；**无任何关于 Siri AI 或 Apple Intelligence 用量上限的条目**（Siri AI 仅出现于 App Intents Video schema 与 AppEntity 解锁两条） | 未截 | 官方日期无时刻。`b-dev-relnotes-ios273.txt` 仅 5 行（页面由 JS 渲染），以 `.json` 为准 | 已找到（“未写”） |
| B3-3 “27.3 beta 2 开始测试/强制” | L4 | https://www.macrumors.com/2026/11/16/apple-icloud-paid-siri-ai-upgrade/ | “With the latest iOS 27.3, iPadOS 27.3, and macOS 27.3 betas, Apple is starting to test limits and paid upgrades for Siri AI and Apple Intelligence features.” | 未截 | 11-17 02:14:08 北京（2026-11-16T18:14:08Z，页面元数据）；MacRumors 称消息来自 beta 内容与苹果文档 | 部分支持（媒体说法；非苹果原文） |
| B3-4 同上 | L4 | https://9to5mac.com/2026/11/16/apple-intelligence-siri-ai-usage-limits/ | “iOS 27.3 beta 2 is now available, and Apple is adding the ability for users to unlock higher usage limits for Apple Intelligence and Siri AI via iCloud+.” | 未截 | 11-17 02:12:09 北京（页面元数据；dateModified 04:04:47 北京） | 部分支持 |
| B3-5 同上（措辞更强） | L4 | https://appleinsider.com/articles/26/11/16/siri-ai-daily-usage-limits-and-fees-will-launch-with-ios-273 | “From iOS 27.3, Apple is imposing daily usage limits on such requests as Siri AI and image editing tools like Extend.” | 未截 | 页面标注 “Mon Nov 16 2026, 01:16 PM EST”=11-17 02:16 北京；页面元数据无 datePublished。**苹果自己的 Nov 12/Nov 16 页写的是 “implements”，AppleInsider 称“从 27.3 起强加”，与苹果 “increased access … starting with the 27.3 release” 的主语不同** | 部分支持 |
| B3-6 额度“此前是否已在执行”的反向线索 | L5 | https://www.idropnews.com/news/apple-reveals-how-icloud-plus-unlocks-higher-ai-limits-ios-27-3/271205/（11-17 05:40:17 北京） | “While Apple says these changes will be “starting with the 27.3 release,” the wording in its … document suggests that iOS 27.3 will be unlocking the increased access on the higher iCloud+ tiers, rather than enforcing limits that didn’t previously exist.” | 未截 | 仅作线索；与 B3-5 相反 | 部分支持 |
| B3-7 “限额 9-14 对所有人生效” | L5 | https://www.macobserver.com/news/apple-intelligence-daily-limits-icloud-plus-paid-tier/（2026-09-12 11:47:04 北京） | “… the limits take effect for everyone on Monday, September 14.” | 未截 | 与苹果页 “In the future, Apple will implement”（09-09 版）相悖，来源为二手、无苹果出处，**未核实**；仅提示“限额现在是否已在稳定版执行”苹果没有明说 | 未找到一手来源 |

**小结**：苹果一手只确认“从 27.3 起、符合条件的 iCloud+ 有更高额度”；“27.3 beta 2 里已经开始测试/执行”的说法目前只有 MacRumors 等媒体（L4），苹果开发者发行说明无提及。**哪一刻起额度对谁真正生效，苹果未写，不可在正文里写死。**

### B4　iCloud+ 价格

来源：https://support.apple.com/en-us/108047（“iCloud+ plans and pricing”，“Published Date: September 16, 2026”，官方未给时刻；存档 `b-apple-108047-en.*`）；中国大陆页 https://support.apple.com/zh-cn/108047（“发布日期： 2026 年 09 月 29 日”，`b-apple-108047-zhcn.*`）。截图 `22-icloud-price-us-china.png`（拼接 6 段：说明句、两张表的表头、美国行、中国大陆行、脚注 1–4）。**两张表各取一行并拼接，非原页连续截图。**

| 地区（币种） | 50 GB | 200 GB | 2 TB | 6 TB | 12 TB | 脚注 | 状态 |
|---|---|---|---|---|---|---|---|
| United States（USD） | $0.99 | $2.99 | $9.99 | $29.99 | $59.99 | 4. “Residents in some U.S. states have tax added to the monthly payment due to state laws.” | 已找到 |
| China mainland / 中国大陆（CNY） | ¥6 | ¥21 | ¥68 | ¥198 | ¥398 | 3. “Taxes are included in all prices for this country or region.” | 已找到 |
| Hong Kong（HKD）（附带） | HK$ 8 | HK$ 23 | HK$ 78 | HK$ 238 | HK$ 468 | — | 已找到 |
| Taiwan（TWD）（附带） | NT$ 30 | NT$ 90 | NT$ 300 | NT$ 900 | NT$ 1790 | 3 | 已找到 |

说明：页面原句 “After you upgrade, you're billed monthly.” 表题即月费；免费档 “5 GB of storage for free”（“iCloud keeps your information safe, automatically backed up, and available anywhere you go — with 5 GB of storage for free.”）。价格页本身不写 AI 额度。页面 Last-Modified（en 为 11-16 16:30 GMT，zh-cn 为 11-17 02:30 GMT）与页面正文日期不符，不采信。9to5Mac、MacRumors、Macworld 的价格表数字与此页美国行一致（L4 对照）。

### B5　媒体与夸大实例

#### 英文媒体（各存一篇，另补）

| 媒体 | URL | 标题 | 发布时间（北京） | 关键句（逐字） |
|---|---|---|---|---|
| 9to5Mac（L4） | https://9to5mac.com/2026/11/16/apple-intelligence-siri-ai-usage-limits/ | Apple reveals how iCloud+ increases Siri AI and Apple Intelligence usage limits | 11-17 02:12:09（2026-11-16T18:12:09Z；页面作者 Chance Miller） | “The usage limits are per device, not per account. This means your iPhone, iPad, and Mac each have separate allowances. Apple isn’t disclosing specific limit numbers.” |
| MacRumors（L4） | https://www.macrumors.com/2026/11/16/apple-icloud-paid-siri-ai-upgrade/ | iOS 27.3 Beta Adds Daily Siri AI Limits and iCloud+ Upgrades | 11-17 02:14:08 | “Apple is not planning to offer additional usage beyond what’s available with the 12TB iCloud+ plan at this time.”；“Apple's existing iCloud+ plans will include higher daily usage for Siri AI and Apple Intelligence, with no change in pricing.” |
| AppleInsider（L4，opencli 取得，Cloudflare 对 curl 质询） | https://appleinsider.com/articles/26/11/16/siri-ai-daily-usage-limits-and-fees-will-launch-with-ios-273 | 页面标题 “Apple announces Siri AI daily usage limits and charges”；页内标题 “Siri AI daily usage limits and fees will launch with iOS 27.3” | 11-17 02:16（页面 “Mon Nov 16 2026, 01:16 PM EST”） | “What Apple is not making clear is precisely what these figures actually mean. The company is not saying how many Siri AI requests fall within the limits”；“There are no changes to the iCloud+ pricing tiers.” |
| The Verge（L4） | https://www.theverge.com/tech/1034581/apple-intelligence-siri-ai-daily-usage-limits-icloud-subscription | Apple is testing daily usage limits for more complex Siri AI interactions | 11-17 03:24:40（2026-11-16T19:24:40Z；页面显示 “Nov 17, 2026, 3:24 AM GMT+8”） | “Apple hasn’t specified exactly how many times those features and tools can be used each day before limits are reached, but they’ll vary based on the task.”；“most of Apple’s existing iCloud Plus plans (excluding the 50GB option) will include higher daily limits at their same price.” |
| Macworld（L4） | https://www.macworld.com/article/3287111/heres-how-much-youll-need-to-pay-to-use-siri-ai-after-apples-new-limits-kick-in.html | Here's how much you'll need to pay to use Siri AI after Apple's new limits kick in | 页面三处元数据互相不一致：`2026-11-16`；`2026-11-16T19:47:39-08:00`（=11-17 11:47:39 北京）；`2026-11-17T11:00:00Z`（=11-17 19:00 北京）。**全部记录，不取舍** | “Unfortunately, Apple does not publish any specific details on what the limits are.” |

#### 中文稿

| 媒体 | URL | 标题 | 发布时间（北京） | 与苹果原文的出入 |
|---|---|---|---|---|
| IT之家 | https://www.ithome.com/1/028/033.htm | 苹果 iOS 27.3 测试版新增 Siri AI 每日限额，付费订阅 iCloud+ 可提升 2~20 倍 | 2026/11/17 7:50:18（页面显示，按北京时间） | 原句 “50GB（免费）：不增加 AI 使用额度。” —— 苹果价格页 50 GB 为付费档（$0.99；中国大陆 ¥6），免费档是 5 GB；该句把付费的 50 GB 写成“免费”。其余倍数与 9to5Mac、苹果表一致；美元换算（如 $2.99 “约合 20 元人民币”）为 IT之家自算，苹果中国大陆页是 ¥21。“每台设备每天都将获得一定数量的免费 AI 使用额度”忠实。 |
| 凤凰网·原创 | https://original.ifeng.com/c/8y7SefjP8id | 苹果为Siri AI设每日限额，订阅iCloud+套餐最高可提升至20倍额度 | 11-17 08:31:08 +08:00 | 原句 “免费额度按设备分别计算，苹果暂未公布每天可使用的具体次数”，忠实；“最高可提升至20倍”保留 “最高”。 |
| 动察 Beating | https://www.beating.news/flash/377767 | 苹果开始给Siri AI按天限额：用完等明天，或者升级iCloud+套餐 | 页面无元数据；Exa 摘要标注 11-17（未核时刻） | 忠实，写明 “50GB 付费档不增加额度”。 |
| NOWnews（台湾） | https://www.nownews.com/news/6893017 | 蘋果Siri AI不能無限用了！iPhone設上限、付iCloud可提高20倍 | 页面文本 “2026-11-17 12:23:46” | 标题“不能無限用了”暗示此前无限用；而苹果 6-08、9-09 页早写有上限。正文：“根據外媒《MacRumors》報導，蘋果已在最新iOS 27.3測試版中，加入Siri AI每日免費額度機制”，转述 MacRumors，写明“僅於 iOS 27.3 測試版”。 |
| INSIDE（台湾，opencli） | https://www.inside.com.tw/article/42932-apple-siri-ai-daily-limits-icloud-plus | Apple 開始替 Siri AI 設每日額度，iCloud+ 訂閱者最高可享 20 倍用量 | 2026-11-17T13:27:00+08:00（页面） | 写明 “Apple 尚未公布台灣的方案對應，實際級距仍待官方說明”，较谨慎。 |
| 鏡報（台湾） | https://www.mirrordaily.news/story/98524 | Apple AI也要收錢了！Siri不再無上限任你問　收費標準曝光「每月最低90元」 | 2026-11-17T17:46:00+08:00（页面元数据） | 见下“夸大说法实例”。 |

#### 夸大说法实例（把事件写歪的标题/文章）

1. **The Verge 题注（英文，L4）**：原句 “Frequent users of some Siri AI and Apple Intelligence features will require an iCloud Plus subscription to avoid limits.”（11-17 03:24:40 北京；发布方 The Verge）。与苹果原文的出入：苹果页 50 GB 付费档与免费同为 “Standard Device Limit”（要 ≥200 GB 才有加量），最高档 12 TB 也只是 “Up to 20X”，并非“avoid limits”（避免限额）；Verge 正文自己也写 “excluding the 50GB option”。截图 `23-verge-headline-avoid-limits.png`。
2. **鏡報（中文，L5）**：标题原句 “Apple AI也要收錢了！Siri不再無上限任你問　收費標準曝光「每月最低90元」”（11-17 17:46 北京）；正文 “官方已針對Siri AI以及Apple Intelligence制定收費標準，然而每台裝置每天都有固定的免費使用次數”。出入：苹果页没有“Siri 收费”，是既有 iCloud+ 档位附带加量，价格未变（MacRumors：“with no change in pricing”）；苹果未公布任何固定次数；同文末尾自己又写 “仍尚未正式公開Apple Intelligence相關的收費機制”，前后矛盾。未截图（页面有弹窗广告遮挡）。
3. **IT之家（中文，L5）**：“50GB（免费）”，见上表。截图 `24-ithome-50gb-free-list.png`（标题日期与列表拼接 2 段）。
4. **AppleInsider（英文，L4）**：标题 “Siri AI daily usage limits and fees will launch with iOS 27.3” / 页面标题 “Apple announces … charges”；正文 “As Apple has been saying since July 2026, Apple Intelligence compute costs are tied to iCloud+ subscriptions.” 出入：苹果 6 月 8 日新闻稿（`b-newsroom-wwdc26-intelligence.txt`）已写 “Increased access is available with most iCloud+ subscription plans”，不是 7 月；“fees … launch with iOS 27.3” 把 “starting with the 27.3 release” 写成了“上线”；其“约 80% 时通知”等细节苹果页没有（苹果页只写 “you’ll be notified”）。正文又写 “There are no changes to the iCloud+ pricing tiers.”。
5. 找不到的写法：本次检索未见 “Siri 要付费才能用”“最多 20 次”这类写法（Exa 英中检索 + 上列中文稿全文检索；**未穷尽**）。

### B6　热度与社区质疑

| 项 | 数据 | 来源 |
|---|---|---|
| HN `50520857` “iOS 27.3 Beta Adds Daily Siri AI Limits and iCloud+ Upgrades”（指向 MacRumors） | 2 分 / 0 评论；11-17 02:16 北京提交；取得时刻 11-17 19:26:53 | `barking heat … --item 50520857` → `b-heat.json`、`b-hn-50520857.json` |
| HN `50526863`（同链接再次提交） | 1 分 / 0 评论；11-17 11:02 北京 | Algolia 检索（`story`、关键词 “Siri AI”“iCloud+”） |
| Reddit | **失败**：`barking heat` 搜索重定向到登录页（`b-hn-heat.json` errors），opencli 再试一次 old.reddit 搜索同样登录墙。未登录、未绕过 | — |

HN 无评论可摘。替代：MacRumors 文章下论坛高赞评论（L6，仅作线索；摘自 `b-macrumors.txt`；不记用户名，只记帖链接与得分）：

1. 得分 29，https://forums.macrumors.com/threads/ios-27-3-beta-adds-daily-siri-ai-limits-and-icloud-upgrades.2494551/post-34961166 ：“I thought the whole point of the iPhone 15 Pro, iPad Mini A17, and newer was that they could run things like "Image generation and editing tools, including Image Playground and the AI options in Photos like Clean Up, Extend, and Reframe" locally. Why are they being limited or even pushed up into the cloud to begin with?” —— 线索：哪些功能走端侧、哪些走服务器。苹果 6-08 新闻稿称 Image Playground 新模型 “runs on Private Cloud Compute”，Apple 页面称受限类别 “rely on powerful server models”，可核。
2. 得分 21，…/post-34961216 ：“'asking Siri what time you need to leave from home to make it to a location on time' WTF is this? That's something old school Siri/maps could do.” —— 线索：苹果页示例的“高级请求”（须调用服务器模型）与旧版 Siri 能做的边界，不是官方给出的分界线；苹果页只说 “Advanced requests are more complex”。
3. 得分 19，…/post-34961174 ：“But what is that free limit? How do we know how to manage our usage? Or do we just wait for Apple to notify us?” —— 线索：没公布次数；苹果页只写 “you will be notified”。

---

## 二、可能的吠点（原文有、线索摘要没提或说反的条件；L1/L4/L6 标注，进正文前回原文）

1. 标题“每天额度”听上去像固定次数，**苹果一个数也没给**；“generous”是苹果自评。（127901 B1-7；9to5Mac、AppleInsider、Macworld 皆称未公布）
2. **倍数是 “Up to”，基数是未定义的 “Standard Device Limit”**；表题 “Total … limits” 与正文 “increased usage limit that is shared” 并存，2X 是总量还是增量页面没说。（127901 倍数表；L1）
3. **50 GB 付费档额度与免费相同**；要 ≥200 GB 才有加量。（127901 “at least 200 GB”；Verge “excluding the 50GB option”）
4. **每台设备各自有基础额度；加量部分是账号下所有设备和家庭成员共享，先到先得**，一个人可能耗光。（127901）
5. **额度按“功能类别”分别计，重置口径有三个例外**（出国按账号地区中部时区；家庭按购买者时区；未登录 24 小时滚动）。（127901）
6. **日常请求不受限，“advanced requests are paused”**；“日常”与“高级”的分界只给了两个例句。（127901）
7. **页面列表非穷尽**（“aren't limited to”）。
8. **AFM Cloud Pro（快捷指令）Free/50 GB 不可用**，200 GB 只给“访问”无倍数。（127901 表）
9. **最高档 12 TB 即顶，苹果暂无更高档**：“Apple is not planning to offer additional usage beyond what's available with the 12TB iCloud+ plan at this time.”（MacRumors，L4；苹果页只写标准额度“may change over time”）
10. **苹果页允许改动**：“the standard amount of Apple Intelligence and Siri AI access, and other iCloud+ plan features may change over time.”
11. **地区：加量在 China/Brazil/EU/UK/South Korea 不可用（Apple Intelligence），Siri AI 加量在 Brazil/EU/UK/South Korea 不可用**；中国大陆账号区 Siri AI 本就不可用；12 月大陆版 Apple 智能不含 Siri AI。（127901、127893、10-31 新闻稿）
12. **“限额”早就写了，加量是 27.3 起**：6-08 新闻稿 “Some features, including image generation, have daily usage limits because they rely on powerful server models. Increased access is available with most iCloud+ subscription plans”；9-09 支持页 “Increased access to such features will be available for a fee.”；11-05、11-12 页仍写“将来收费”（121115、127893）；11-16 才有价目表。（`b-newsroom-wwdc26-intelligence.txt`、Wayback 快照）
13. **Home App 摄像头 AI 摘要台数按档位限定**（2 TB 1 台 / 6 TB 2 台 / 12 TB 5 台）。8-12 华尔街见闻（L5）转述 iOS 27 beta 5 为 “2TB 5 台、12TB 15 台”，与现页不同，说明数字在 beta 期间改过（未核实原始 beta 界面）。
14. **另有按量付费先例**：Verge 写 “Last week Apple updated Logic Pro with an option to “Buy More Usage” … $49.99 Intelligence Usage Packs”（L4，Apple Creator Studio 类 App 的用量包），说明苹果并非只有 iCloud+ 一条路；与 Siri 额度是否互通，未核实。（`b-verge.txt`）
15. **Siri AI 仍标 “(Beta)”**，苹果 127893 写 “During times of high demand, you might be added to a waitlist first”。（L1）

---

## 三、扫描说法勘误（逐条对照“已知情况”）

| 扫描/初步说法 | 存档所见 | 判定 |
|---|---|---|
| 127901 页脚 “Published Date: November 16, 2026” | 属实；但页面自 09-09 存在，11-16 为改版日期（Wayback 快照 `b-apple-127901-wayback-20260910.*`、`-20261022.*`；CDX 清单 `b-wayback-cdx-127901.json`） | 部分支持（不是新页） |
| 各要点原句（“Starting with the 2027 software releases…” 等） | 与存档逐字一致 | 已找到 |
| 表格数值（Free / 50 GB Standard、200 GB 2X、2 TB 5X、6 TB 10X、12 TB 20X；AFM Cloud Pro 行） | 一致 | 已找到 |
| “Home App 视频摘要摄像头：2TB 1 / 6TB 2 / 12TB 5” | 一致 | 已找到 |
| 地区限制两句 | 一致；注意 Siri AI 句无 China | 已找到 |
| 页面未写具体次数 | 确认 | 已找到 |
| ChatGPT 扫描：9to5Mac、The Verge、AppleInsider 11-16 报道 | 均属实（AppleInsider 11-16 美东 13:16；北京 11-17 02:16） | 已找到 |
| ChatGPT 扫描：“27.3 beta 实施” | 只有 MacRumors/9to5Mac/AppleInsider（L4）；苹果一手只有 “starting with the 27.3 release”；开发者发行说明无提及 | 部分支持 |
| ChatGPT 扫描：HN 只有 2 分转帖 `50520857` | 属实：2 分 / 0 评论；另有 `50526863`（1 分） | 已找到 |
| “苹果第一次给 Siri AI 设每日额度”（本期选题角度） | 苹果 6-08 新闻稿、9-09 支持页、11-05/11-12 页已多次声明限额与“将收费”；iClarified（09-10，L4）称 Siri AI 计划 9-14 起 beta 上线。**新的是价目对应表与 27.3** | 部分支持（角度需改写） |
| 美国价格 $0.99/2.99/9.99/29.99/59.99 | 苹果价格页一致 | 已找到 |
| 中国大陆价格 | ¥6/¥21/¥68/¥198/¥398，含税 | 已找到 |
| 中文页是否存在 | 存在，但 9 月旧文；无倍数、无地区排除 | 已找到（说明同步缺口） |

## 四、假设与未完成

- 假设：IT之家页面显示的 “2026/11/17 7:50:18” 为北京时间。
- 未取：苹果台湾/日本等价格页（仅取了 US、CN 页内含的 HK/TW 行）；9to5Mac/The Verge 评论区；Reddit；Apple Newsroom 上有无针对 27.3 额度的专稿（`https://www.apple.com/newsroom/` 首页存档里未见，已检索“usage limits”仅得 6 月三篇）。
- 6-08 新闻稿脚注的截图未取：该页超长且渲染后 OCR 未定位到脚注，文字存档在 `b-newsroom-wwdc26-intelligence.txt` 第 270、372 行（头：“Some features, including image generation, have daily usage limits because they rely on powerful server models. Increased access is available with most iCloud+ subscription plans, which also include Apple Intelligence support for compatible Home cameras.”）。Claude 若要图，建议对此页单独取一次。
- 倍数的真实含义、额度从哪天起对谁生效、中国大陆 12 月版是否设额度：**苹果均未写，不可补写**。
