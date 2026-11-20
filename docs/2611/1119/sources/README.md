# 1119 来源与制作状态

期号1119（上期1118+1）；线索为 Develata 回贴的三份扫描：ChatGPT 定时任务的 stateful rescan（`AI-Barking_stateful_rescan_20261120T110000Z.md`，扫描终点北京 11-20 19:12）与 WorkBuddy HTML 选题扫描 1120-06、1120-18。三份都把 1109 期已发的黑洞预印本当新题推；WorkBuddy 06 照抄 HN 的“两个隐身模型”，WorkBuddy 18 把 Osorio 案 HN 热度记成 139 分（实为 66–67 分）。Claude 北京 11-20 21:00 前后用 HN Algolia 现查热度，并直取 Architect Labs、Contra 榜单、TechCrunch、商汤 README、Signal 博客与 Anthropic 研究页核对后列候选；Develata 选 Frostbite + Signal + Contra 三条主帖，对华芯片许可只进速览；标题经两轮反向核验后由 Develata 定为“公司称AI设计CPU能效4.59倍，没造”。

派工单 `.handoff/2026-11-20-1119-evidence.md`（`barking handoff` 生成的文件名日期 11-19 与“本机时区为美东”有误，已改为 11-20 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`（北京 22:08）：Browser Bridge 扩展已连接。A、B、C 组 Sonnet 5.5，D 组 Haiku 5.5；用时（harness）A 8、B 12、C 16、D 14 分钟，未催交。

## 主帖

1. Frostbite：[Architect Labs 博客](https://architectlabs.com/blog/frostbite)（页面 November 18, 2026，官方未给时刻）、[Redwood 前作博客](https://architectlabs.com/blog/redwood)（对照“人写规格”的说明）。HN 50572503：北京 22:17 为 301 分 / 207 评（18:01 提交；同博客另有两次低分提交）。
   - 对手 TCPU：双发射、顺序执行，0.895 mm²，型号“因法律原因”不公开；比的是一个 quad（2×2 tile + 4 个 L3 节点，0.792 mm²）对单核。
   - 4.59×（25 项几何平均）= 调优集 10 项 6.02×、留出集 15 项 3.83×；留出 15 项中 5 项后加；Fig. 11 无一项低于 1×，最低 fib 1.15×、最高 conv2d 23.12×。
   - pre-silicon：只到 7 nm 最终 GDS（500 MHz），无硅片或 FPGA 实测；RTL/ISA 开源、FPGA 试用均为计划、无日期。
   - 夸大实例：Notebookcheck 标题 “AMD and Intel, move over”（正文写了 pre-silicon）、Aroged “crushed conventional CPUs”；中文大陆主流媒体未见报道。
2. Signal：[Signal 博客](https://signal.org/blog/protecting-your-privacy/)（20 Nov 2026，官方未给时刻）、[Signal 支持页](https://support.signal.org/hc/en-us/articles/360043469312-Screen-Security)（取证时未更新 Block Siri AI）、路透（北京 17:02，“switching those restrictions on by default”）、The Verge（20:30）、Android Authority、TNW、苹果支持页（Siri AI 在中国大陆不提供）、Signal-iOS 与 Signal-Android 仓库提交。HN 50572348 31 分、50573902 4 分（22:13）。
   - 博文未写“默认开启”，这句只来自路透（L4）与 TNW；正文写“据路透”。
   - iOS 设置 “Block Siri AI”（设置-隐私，来自官方代码提交与博文截图）；Android 是“屏幕安全”，检测到 AI 助手读屏才自动打开，设置页写 “some AI assistants such as Google Gemini”。
   - 博文自认不完整：文件系统读取、macOS、部分 Android 设备不覆盖；macOS 用户可在系统设置 Siri 的 Excluded Apps 里加入 Signal（正文未写）。
   - 生效版本官方未给；代码在 iOS 8.37、Android 8.36.1 的 tag 里（推断，未写进正文）。
3. Contra 设计榜：[榜单](https://contralabs.com/frontierdesign)、[方法](https://contralabs.com/frontierdesign/methodology)、[博客](https://contralabs.com/frontierdesign/blog)、[评分细则](https://contralabs.com/frontierdesign/rubric)、[数据集页](https://contralabs.com/frontierdesign/dataset)。HN 50558559 145 分 / 46 评（截图时）。
   - 17 行中只有 Gestalt 标 stealth；第 2 行 “Human designers · Recruited from Contra”（Wayback 11-19 快照里叫 “Human golden”）；胜率 95% CI 只给胜率、不给 Elo 区间。
   - 官方两两矩阵：人类参考稿对 Fable 5.2 为 60%、对 Gestalt 29%（次数未给）；博客 Gestalt 对参考稿 20/28。
   - 评分细则 Gestalt 83、Fable 5.2 与 Opus 5.5 均 84；人类参考稿 73.6（博客）。
   - 发布串前后不一：X 首帖写 15 个模型、3,591 票、人类第 1；24 分钟后才出 Gestalt 帖。博客与实时榜数字略有差异（1,928/1931）。
   - 夸大实例：17173 “神秘大模型Gestalt杀穿AI排行榜”并称可在 OpenRouter 调用（官方未说）。

## 速览

- 对华芯片许可：[众院外委会通稿](https://foreignaffairs.house.gov/news/press-releases/chairman-brian-mast-exercises-legislative-authority-to-publicly-release-ai-chip-license-information-for-china)（北京 11-20 03:18）与通稿内嵌的两份 ECRA 披露件（Scribd；截至 2026-05-11 申请 3,444,160 颗、批准 879,868 颗、21 份许可，附表 21 行均 “Approved, not shipped”）、[CNBC](https://www.cnbc.com/2026/11/19/nvidia-amd-china-chip-sales-congress.html)。风控：标题、封面不碰。
- Anthropic 424B4（EDGAR，申报日 11-19）“voluntary, revocable … not legally binding”；[TechCrunch](https://techcrunch.com/2026/11/19/anthropics-7-founders-cashed-out-nearly-2b-in-record-breaking-ipo/) 称创始人售股约 19.5 亿美元（未上卡）。
- 【跟进1109】[Anthropic 研究页](https://www.anthropic.com/research/claude-black-hole-physics)（11-17）：22 个候选、一题上界证明的一半沿用 OpenAI 证明思路、9 月 Lean 验证。HN 50563148 518 分。
- Osorio v. Kilmer：判决书（东肯塔基联邦地区法院，11-09 签署，经路透页内链接取得）、路透 11-10、Techdirt 11-20。
- [DeepMind 员工工会站](https://www.gdmunion.co.uk)（无日期；HN 50557885 123 分）。
- 取证但未上卡：[Gemini Mac 版 computer use](https://blog.google/innovation-and-ai/products/gemini-app/computer-use-mac/)（第 2 轮反向核验：40 字装不下全部使用条件，删去）、Palo Alto Networks 收购 Applied Compute、MAI Image 2.7（AA 榜 1215±14 对 1211±13，区间重叠）、Edge Aion Instruct、OpenAI 会计插件、MIT 数据中心空气污染。

## 备注

- D 组 Haiku 交卷中把卖出股东股数写成“23.65 亿股”，实为 23,648,646 股；未进正文。
- 已删除的废弃文件（北京 11-21 收尾一次删除）：A 组 `images/08-next-plans-spec-parsec.png`、`images/08b-next-plans-spec-parsec.png`；B 组 `images/22-reuters-default-on-popup.png`；D 组 `sources/` 下 403/401 错误页与中间包装（清单见 `capture-log-d.md`）；仓库外 `/tmp/x`。
