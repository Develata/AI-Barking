# 1116 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（Win11 编码代理记名）、`evidence-B.md`（Siri 每日额度）、`evidence-C.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号，`*-raw.png` 为批注底图）。取证：A、B 组 claude-sonnet-5-5，C 组 claude-haiku-5-5，北京 11-17 约 19:23 派出（C 组约 4 分钟、A 组约 38 分钟、B 组约 45 分钟交回）。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：Win11记你用啥AI？Siri每天限量 | 研究者自述 + L1 | ✅（前半问号问研究者与转载稿的说法） | 见 A、B 节 | 前半：博文标题 “Windows 11 Is Collecting Data on Which Coding Agents and Developer Tools You Use”；问号保留其未证实部分（只证到上传队列、单台测试机）。后半：Apple 127901 “have daily usage limits”。读者稿检查后由“Win11偷记AI工具？Siri每天限量”改（读者称两半像同一件事、“偷记”带判断）。 |
| 省流：有研究者称，Win11 会认出你用过哪些 AI 编程工具，记进准备发给微软的诊断数据 | 研究者自述 | ✅（署“研究者称”） | `a-blog.txt` 第 20、117、155 行 | “record which coding agents and developer tools have been used … reports it to Windows Diagnostics”；“Windows queues the coding-agent usage report for upload to Microsoft”（条件：允许可选诊断数据）。 |
| 省流：但他只介绍了一台测试机上的实验，也没证实数据真发出去了 | 研究者自述 | ✅ | `a-blog.txt` 第 75、141、155 行；`a-claude-blog-v2.txt` 同句仍在 | 单台 Parallels ARM64 VM（build 26200.9974）；“An ETW event by itself would not establish that anything leaves the machine.”；验证止于 DiagTrack 上传队列，无网络抓包。干净安装那台只看了诊断开关默认值。 |
| 省流：苹果公布 iCloud+ 各档的 AI 加量倍数，最高20倍，但没说基础额度一天能用几次 | L1 | ✅ | `b-apple-127901-en.txt` 第 93–103 行；图 16、`siri-table-raw.png` | “Up to 20X Standard Device Limit”（12 TB）；全页无具体次数（B1-12；9to5Mac、AppleInsider、Macworld 亦称未公布）。 |

## A Win11 按程序名记录 AI 编程工具

研究者博文 https://xusheng.dev/posts/windows-coding-agent-telemetry/ （作者 Xusheng Li，自述在 Vector 35 开发 Binary Ninja；按“研究者自述”署名，类同 1115 期 uBlock 研究者）。三个版本：A 组北京 11-17 19:24 存档 `a-blog.*` 与 Claude 19:22 读到的一致（v1）；Wayback 快照 20261116230024（北京 11-17 07:00）与 v1 逐字相同（`a-wayback-blog.txt`）；Claude 北京 11-17 20:1x 重取 `a-claude-blog-v2.*`（v2，`Last-Modified: Tue, 17 Nov 2026 11:40:31 GMT` = 北京 19:40:31）。v1→v2 唯一改动：删去第 113 行一整段（见吠点②），字数 1480→1437；页面无更正说明，作者帖子线程（`a-claude-fx-thread.json`，北京 20:16 取）只有 11-16 的两条，HN 评论区未见作者。交付前再比对一次（北京 20:38 重取，Last-Modified 未变、正文与 v2 相同）。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 一位研究者发文称 | 研究者自述 | ✅（不写日期） | 页脚 “2026-11-16 00:00”（占位时刻）；首发帖北京 11-16 10:00:25 | 反向核验 NEEDS_VERIFICATION 6 后删去“11月16日”：博文真实发布时刻不明，晚于北京 11-16 20:34（正文 ETW 事件时间戳），早于 11-17 06:06（Last-Modified）/07:00（Wayback）。 |
| 他借助 AI 翻查 Win11 系统文件 | 研究者自述 | ✅ | `a-blog.txt` 第 20 行；`win-vm-raw.png` | “I recently asked AI to help me look through the latest Windows binaries for undocumented features and functionality.” |
| 发现一个负责诊断的系统组件会按程序名认出 Codex、Claude 等 AI 编程工具 | 研究者自述 | ✅ | `a-blog.txt` 第 30、95–110 行；图 03 | MissionControlAggregator.dll，由 DiagTrack（Connected User Experiences and Telemetry）的子进程 AggregatorHost.exe 加载；4 个 CLI 槽 copilot/claude/codex/agy，5 个桌面槽 github/claude/codex/cursor/antigravity。 |
| 他把系统自带的命令行程序改名成 codex.exe，运行后组件就记下“用过 Codex” | 研究者自述 | ✅ | `a-blog.txt` 第 117–119 行；图 06、`win-queue-raw.png` | “I copied cmd.exe to a file named codex.exe … The name alone was enough for the component to recognize it as Codex.” |
| 记录随后进了准备发给微软的“可选诊断数据” | 研究者自述 | ✅ | `a-blog.txt` 第 151–155 行；图 07、`win-queue-raw.png` | “the agent-usage report is treated as optional diagnostic data”；“When optional diagnostic data is allowed …, Windows queues the coding-agent usage report for upload to Microsoft.”为立刻出报告，作者把窗口起点拨前 25 小时（未进正文）。 |
| 他称在美国区域全新安装 Win11 时，“可选诊断数据”开关默认打开，可在设置的“诊断和反馈”里关掉 | 研究者自述 | ✅ | `a-blog.txt` 第 170–172 行；图 07b | Windows 11 Home ARM64、最新官方 ISO、区域美国；“Optional diagnostic data is therefore enabled by default, at least in the US setup I tested.”；关闭路径 “Settings → Privacy & security → Diagnostics & feedback”（中文界面“设置 > 隐私和安全性 > 诊断和反馈”）。其他区域未测。 |
| 吠点①：博文只介绍了一台测试机上的记录实验，那台的记录功能已作为实验开着，不是他手动开的 | 研究者自述 | ✅ | `a-blog.txt` 第 75 行（v2 仍在）；`win-vm-raw.png` | “I checked my Parallels Windows 11 ARM64 VM, build 26200.9974 … CLICodingAgentUsage was already enabled as a service experiment. I had not manually enabled the feature.”反向核验 NEEDS_VERIFICATION 7 后由“只在一台测试机上验证过”改：博文只介绍这一台的记录实验，不替作者断言总共测了几台；博文未写队列 JSON 在哪台机器（A 组注）。 |
| 吠点②：原文曾写，用官方安装包装的虚拟机里这项功能默认关，证据不支持说每台 Windows 都开着；17日晚这句被删，页面没注明改动 | 研究者自述 + Claude 比对 | ✅ | v1 `a-blog.txt` 第 113 行、Wayback 快照（`win-deleted-raw.png`）；v2 `a-claude-blog-v2.txt`（该段缺） | v1 原句：“The public ISO’s baseline VM had the coding-agent feature disabled by image default, so the evidence does not support saying that it is active on every Windows installation.”删除时点在北京 11-17 19:24（A 组存档仍有）与 20:1x（Claude 重取已无）之间，`Last-Modified` 为 19:40:31，写“17日晚”。不推测删除原因，不写“改口”。 |
| 吠点③：研究者只查到记录进了待发队列，没证实数据真的发给了微软 | 研究者自述 | ✅ | `a-blog.txt` 第 141、155 行；`win-queue-raw.png` | “An ETW event by itself would not establish that anything leaves the machine.”之后只验证到 DiagTrack `EventStore.db` 的 AsimovEvents 队列；无网络抓包。80aj（北京 11-17 08:14）写“上传队列抓包，证实了数据确实离开设备”（初稿引用，读者稿检查后删）。 |
| 吠点④：研究者判断，这份记录似乎只写某工具“出没出现过”、不记用了多久，报告里也没看到提示词或代码 | 研究者自述 | ✅ | `a-blog.txt` 第 67–71 行；图 04 | “A set Codex bit means the matching tool was observed during the reporting window.”；“This component appears to discard those details [runtime, time in focus …]”（作者用 appears to；反向核验 SHOULD_FIX 3 后由“研究者称……不记”改为“研究者判断……似乎”，并限定“这份报告”）；“I found no prompt text or source code in this report”。 |

未进正文：微软文档把“which programs are launched on a device, how long they run”列入可选诊断数据、未点名编码代理或该组件（`a-ms-configure.txt`；图 08）；截至北京 11-17 约 19:30 未找到微软回应（A4-5，检查范围有限；初稿④，读者稿检查后删）；作者首发帖与博文标题无条件措辞（A3-1）；WorkBuddy 所称“转载站写未提供 build 号”“写成 usage history”在 15 篇二手稿中未复现。

## B 苹果 Siri AI 每日额度

Apple Support 127901 https://support.apple.com/en-us/127901 （`b-apple-127901-en.*`；页脚 “Published Date: November 16, 2026”，官方未给时刻；Claude 北京 11-17 19:22 读到同文）。该页 9-09 起已存在（Wayback `b-apple-127901-wayback-20260910.*`），11-16 为改版；中文页 zh-cn/zh-tw/zh-hk 仍是 9 月旧文（B2-1、B2-2）。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 苹果帮助页标注11月16日更新 | L1 | ✅ | 页脚日期 | “Published Date: November 16, 2026”，官方未给时刻与时区，未换算北京时间（反向核验第 2 轮后由“苹果11月16日更新帮助页”改）；是改版而非新页，故写“更新”。 |
| Siri AI、修图等要用苹果服务器上大模型的功能每天有上限 | L1 | ✅ | 第 37–45 行；`siri-limit-raw.png` | “Some Apple Intelligence and Siri AI features rely on powerful server-side models and have daily usage limits”；类别含 Siri AI、“Intelligent Photo Editing Tools including Clean Up, Extend, and Spatial Reframing, and Image Playground”、快捷指令 AFM Cloud、第三方 PCC；“aren't limited to”。 |
| 用完后定计时器这类日常请求照常，复杂请求要等额度重置，或升级到 200GB 及以上的 iCloud+ 云存储套餐 | L1 | ✅ | 第 55–57 行；`siri-limit-raw.png` | “you can still make everyday Siri requests like "Set a timer for 3 minutes"”；“advanced requests are paused until your limit resets or you upgrade to an eligible iCloud+ plan.”“For more access, upgrade to an iCloud+ plan with at least 200 GB of storage.”（反向核验 BLOCKER 2 后补“200GB 及以上”：50 GB 也是 iCloud+ 但不加量）。约当地午夜重置，有三种例外（未进正文）。 |
| 免费用户和 50GB 档是基础额度，200GB 档最多2倍，12TB 档最多20倍 | L1 | ✅ | 第 89–103 行；图 16、`siri-table-raw.png` | Free、50 GB 为 “Standard Device Limit”；200 GB “Up to 2X”、2 TB “Up to 5X”、6 TB “Up to 10X”、12 TB “Up to 20X Standard Device Limit”。表题 “Total …” 与正文 “increased usage limit that is shared” 并存，2X 是总量还是增量页面没说（未进正文）。 |
| 从27.3版系统起提供 | L1 | ✅ | 第 37 行 | “increased access is available with eligible iCloud+ plans, starting with the 27.3 release.”未写 beta 或正式版；“27.3 beta 2 已实施”只见 MacRumors、9to5Mac、AppleInsider（L4），开发者发行说明无此条（B3-2）。 |
| 多出来的额度由同一账号的设备和共享家人一起用，先到先得 | L1 | ✅ | 第 75 行；`siri-table-raw.png` | “an increased usage limit that is shared by all devices signed in to your Apple Account and across any Family members sharing your plan, on a first-come, first-served basis.”基础额度按设备计（“Each device has its own daily usage limit per feature category”）。 |
| Siri AI 目前不支持中国大陆账号 | L1 | ✅ | `b-apple-127893-en.txt` 第 79 行；图 20 | “Siri AI will not currently work if your Apple Account region is in mainland China.”（127893，Published November 12, 2026）。苹果中国 10-31 新闻稿：12 月 iOS 27.3 面向中国用户推出的 Apple 智能“不包含 Siri AI”（图 21）。 |
| 部分 Apple 智能功能在中国也不能加量 | L1 | ✅ | 127901 第 119 行；`siri-table-raw.png` | “Increased access to select Apple Intelligence features with iCloud+ … is not available in China, Brazil, the EU, UK, or South Korea.”Siri AI 加量那句只列 Brazil、EU、UK、South Korea。 |
| 吠点①：苹果没说基础额度一天能用几次，“最多20倍”到底是几次也就没法算 | L1 | ✅ | B1-12 | 页面无次数；“generous”是苹果自评；“Standard Device Limit”未定义；“Limits may vary by feature category, request complexity, system demand …”。The Verge 题注 “will require an iCloud Plus subscription to avoid limits”（初稿引用，读者稿检查后删）。 |
| 吠点②：50GB 档也要付钱（国内每月6元），额度却和免费用户一样 | L1 | ✅ | `b-apple-108047-en.txt`（China mainland ¥6，含税）、127901 表；图 22 | 中国大陆 50 GB ¥6/月（“Taxes are included”）；美国 $0.99。IT之家（北京 11-17 07:50）写“50GB（免费）”（初稿引用，读者稿检查后删；图 24）。 |
| 吠点③：苹果6月发布会新闻稿就写了部分功能每天限量、可靠 iCloud+ 加量，这次新公布的是各档倍数 | L1 | ✅ | `b-newsroom-wwdc26-intelligence.txt` 第 270、372 行（2026-06-08 新闻稿脚注） | “Some features, including image generation, have daily usage limits because they rely on powerful server models. Increased access is available with most iCloud+ subscription plans”。9-09 版 127901 写 “In the future, Apple will implement usage limits … Increased access … will be available for a fee.”（图 18）。脚注未截到图（OCR 定位失败），以文本存档为据。 |

未进正文：AFM Cloud Pro 快捷指令 Free/50 GB 不可用；Home App 摄像头台数（2 TB 1 台、6 TB 2 台、12 TB 5 台）；MacRumors 称 iCloud+ 价格不变、12 TB 以上暂无更多（L4）；Logic Pro 已有 $49.99 用量包（The Verge，L4，未核）；中文帮助页仍是 9 月旧文。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| AMD 称已完成收购 World Labs，李飞飞任 AMD 首席科学家 | L1 | ✅ | https://newsroom.amd.com/news/amd-completes-acquisition-world-labs/ （`c-amd-newsroom-complete.txt`） | “AMD (NASDAQ: AMD) today announced the completion of its acquisition of World Labs”；“Dr. Fei-Fei Li joins AMD as executive vice president and chief scientist, reporting directly to Su.”官方日期 11-16，未给时刻。完成稿无金额；9-28 签约稿 “all-stock … approximately $8.2 billion”（未上图）。 |
| HF 托管推理的语言模型改走 OpenRouter，不能再用自带供应商密钥 | L1 | ✅ | https://huggingface.co/blog/openrouter （`c-hf-openrouter.txt`） | 范围是 Inference Providers（HF 托管推理）：“Today, we are switching Inference Providers from our own custom router to OpenRouter's.”；“Requests made to language models now go through OpenRouter”；“so you can no longer use your own API key for a provider with these models.”图像、视频、音频照旧（“Custom keys for image and video providers keep working.”）。官方日期 11-16。 |
| Databricks 编码自测：两模型成功率同为89.1%，按标价成本差约32倍 | L1 | ✅ | https://www.databricks.com/blog/introducing-databricks-ai-index-state-enterprise-ai-tokenomics （`c-databricks-aiindex.txt`） | “GPT-6 Luna and Claude Opus 5.5 both achieved an 89.1% task success rate. However, Luna averaged just $0.05 per task compared with $1.62 for Opus 5.5, a roughly 32x difference.”任务取自 Databricks 私有代码库与隐藏测试；成本按公开标价估算；未写档位。官方日期 11-16。 |
| 笔记应用 Mem 宣布2027年1月15日关停，12月18日起笔记只读 | L1 | ✅ | https://get.mem.ai/blog/mem-is-shutting-down （`c-mem-shutdown.txt`） | “Mem is shutting down on January 15, 2027.”；“December 18, 2026: notes become read-only.”公告页 11-16；HN 64 分 / 102 评（北京 11-17 19:25）。 |

未收：加州总检察长起诉 OpenAI（L1 新闻稿与诉状已存档；涉美国州政府，按风控默认不收，交 Develata 定；“至少80起”与诉状“over 80”措辞不同）；英伟达 Vera Rubin“每瓦10倍”、TensorWave 457MW、万拓 One2N（只见 WorkBuddy 扫描，C 组与 Claude 均未找到一手来源）；OpenAI Daybreak 进北约机构、FBI ARTEX（风控）；OpenAI 儿童安全分类器（题材敏感）。

## 批注译注

译注是关键句直译，措辞以上表为准；底图均为 Claude 用 headless Chrome 浅色模式（`--blink-settings=preferredColorScheme=1`）重渲后裁切，记录见 `sources/a-shots-log.jsonl`、`b-shots-log.jsonl`。

| 图 | 译注 | 原句 | 级 | 结果 |
|---|---|---|---|---|
| win-vm ① | 我最近请 AI 帮我翻查最新的 Windows 系统文件，找没有公开说明的功能。 | “I recently asked AI to help me look through the latest Windows binaries for undocumented features and functionality.”（v2 现行版，rects 手量） | 研究者自述 | ✅ |
| win-vm ② | （在作者的 Parallels 虚拟机上）CLICodingAgentUsage 已作为一项服务实验开启，作者没有手动开启这项功能。 | “I checked my Parallels Windows 11 ARM64 VM … and CLICodingAgentUsage was already enabled as a service experiment. I had not manually enabled the feature.” | 研究者自述 | ✅（视觉复核后由“服务端实验”改“一项服务实验”，原文未说明实验由谁控制） |
| win-deleted ① | 用公开 ISO 安装的基线虚拟机里，这项编码代理功能按镜像默认是关的，所以证据不支持说它在每台 Windows 上都开着。 | Wayback 20261116230024 快照原句（v1；v2 已删） | 研究者自述 | ✅；标题注明“17日早存档版（当晚现行版已删去此段）”；快照 20261116230024 为 UTC，北京 11-17 07:00（反向核验 SHOULD_FIX 4 后由“11月16日”改） |
| win-queue ① | 我把 cmd.exe 复制成名为 codex.exe 的文件，在交互式 Windows 会话里运行。光凭名字，组件就把它认成了 Codex。 | “I copied cmd.exe to a file named codex.exe and ran it in the interactive Windows session. The name alone was enough for the component to recognize it as Codex.” | 研究者自述 | ✅ |
| win-queue ② | 单凭一条 ETW 事件，并不能证明有任何数据离开了这台机器。 | “An ETW event by itself would not establish that anything leaves the machine.” | 研究者自述 | ✅（anything 译“任何数据”） |
| win-queue ③ | 允许可选诊断数据时（在我测试的美国区域设置中默认开启），Windows 会把编码代理使用报告排进上传给微软的队列。 | “When optional diagnostic data is allowed (enabled by default in the US setup I tested), Windows queues the coding-agent usage report for upload to Microsoft.” | 研究者自述 | ✅ |
| siri-limit ① | 部分 Apple 智能和 Siri AI 功能依赖强大的服务器端模型，设有每日使用上限，以保证所有人的体验稳定可靠。 | “Some Apple Intelligence and Siri AI features rely on powerful server-side models and have daily usage limits to ensure a high-quality, reliable experience for everyone.”（rects 手量） | L1 | ✅ |
| siri-limit ② | （达到上限后）仍可发出日常 Siri 请求，例如“设一个3分钟的计时器”。 | “you can still make everyday Siri requests like "Set a timer for 3 minutes"”（rects 手量） | L1 | ✅ |
| siri-limit ③ | 达到上限后，高级请求会暂停，直到额度重置或你升级到符合条件的 iCloud+ 方案。 | “If you reach your limit, advanced requests are paused until your limit resets or you upgrade to an eligible iCloud+ plan.” | L1 | ✅ |
| siri-table ① | 要更多额度，需升级到至少 200 GB 的 iCloud+ 方案。 | “For more access, upgrade to an iCloud+ plan with at least 200 GB of storage.” | L1 | ✅ |
| siri-table ② | 免费与 iCloud+ 50 GB：标准设备额度。 | 表格 Free、iCloud+ with 50 GB 两格 “Standard Device Limit”（rects 手量） | L1 | ✅ |
| siri-table ③ | iCloud+ 12 TB：最多为标准设备额度的20倍。 | 表格 iCloud+ with 12 TB “Up to 20X Standard Device Limit”（rects 手量） | L1 | ✅ |
| siri-region ① | 通过 iCloud+ 提高部分 Apple 智能功能的额度，以 Apple 智能可用为前提，在中国、巴西、欧盟、英国和韩国不提供。 | “Increased access to select Apple Intelligence features with iCloud+ is subject to Apple Intelligence availability and is not available in China, Brazil, the EU, UK, or South Korea.” | L1 | ✅ |

## 反向核验

第 1 轮：Codex（gpt-6-astra，effort medium，只读、联网），北京 11-17 约 20:20 起、20:34 交回，`barking review --reverse`，提示词末尾另附“博文 v1/v2 两个版本分别核对”的说明。结论：2 BLOCKER、3 SHOULD_FIX、3 NEEDS_VERIFICATION。Claude 逐条核实：

1. BLOCKER：HF 速览条目丢了“Inference Providers（托管推理）”范围，“自带密钥”比原文宽。采纳，改为“HF 托管推理的语言模型改走 OpenRouter，不能再用自带供应商密钥”。
2. BLOCKER：省流卡列倍数却没带地区限制；“升级 iCloud+”包括了不加量的 50 GB。采纳：省流卡用“；”拼上正文“Siri AI 目前不支持中国大陆账号，部分 Apple 智能功能在中国也不能加量”；正文与事件卡改“升级到 200GB 及以上的 iCloud+ 云存储套餐”。
3. SHOULD_FIX：“不记用了多久”丢了作者的 appears to。采纳，吠点④改“研究者判断，这份记录似乎只写……报告里也没看到……”。
4. SHOULD_FIX：Wayback 快照 20261116230024 是 UTC，图上写“11月16日”。采纳，批注标题改“17日早存档版（当晚现行版已删去此段）”，来源行写“北京 2026-11-17 07:00 快照”。
5. SHOULD_FIX：Databricks 成本是按公开标价估算。采纳，速览改“按标价成本差约32倍”，并写明“编码自测”（40 字上限内删去“每任务”，“成本差”指每任务成本，见上表原句）。
6. NEEDS_VERIFICATION：博文北京发布日期锁不定。采纳，正文删去“11月16日”。
7. NEEDS_VERIFICATION：“只在一台测试机上验证过”是对测试总数的断言。采纳，改“博文只介绍了一台测试机上的记录实验”（省流同步）。
8. NEEDS_VERIFICATION：127901 页写“2027 software releases”又写 27.3。不改：苹果把 iOS 27 等这一代系统称作“2027 software releases”（6-08 新闻稿同一代），27.3 是这一代的版本号，两者不冲突；正文只写“从27.3版系统起提供”，不写已全面实施。

第 2 轮：Codex（同上），北京 11-17 约 20:40 起，提示词附第 1 轮处理记录。结论 PASS / no blocker；确认第 1 轮 1、3、5、6、7 项到位、第 8 项判断成立（苹果 9 月发布稿同时用 iOS 27 与“2027 software releases”）。另 2 SHOULD_FIX、2 NEEDS_VERIFICATION，均采纳：

1. 省流卡 Siri 条漏了“从27.3版系统起提供”。补回；为不溢出，删去句首“苹果……更新帮助页：”一段（主语由标签“Siri”与“苹果服务器”承担）。
2. `images/README.md` 的 Wayback 日期未同步。改为“北京 11-17 07:00 快照”。
3. “苹果11月16日更新帮助页”把未注时区的页脚日期当北京时间。改为“苹果帮助页标注11月16日更新”。
4. 派工时刻写成 19:40，与 A 组 19:24 的抓取记录矛盾。核对后为 Claude 北京约 19:23 派出（初稿误记），`sources/README.md` 与本清单已改。

## 视觉复核

Codex（gpt-6-astra，effort medium，只读），`barking review --visual`，北京 11-17 约 20:45，18 张图（两张封面、省流卡、速览、两张事件卡、两张吠点卡、五张批注图及底图）。结论 PASS / no blocker，4 条 SHOULD_FIX：

1. 省流卡 Siri 条缺 27.3 版本条件：已在反向核验第 2 轮修复（同一问题）。
2. win-vm 译注②“服务端实验”多出“由服务器控制”的含义：采纳，改“一项服务实验”。
3. 吠点卡③“他只查到……”单独截出时主语不清：采纳，正文与卡片改“研究者只查到……”。
4. siri-table 底部地区限制小字太小：采纳，另裁一张窄幅底图 `siri-region-raw.png`（同页地区段与发布日期），新增批注图 `12-siri-region.png`，原表格图去掉该条译注。

其余：高亮未扫入邻句，编号与译注对应，事件卡、吠点卡与正文一致，速览保留限定词，无禁列平台名。修改后 Claude 逐张重看 01、04、05、12。
