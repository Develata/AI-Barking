# 1115 来源与制作状态

期号1115（上期1114+1）；线索为 Develata 回贴的两份扫描：ChatGPT 定时任务的 stateful rescan（`AI吠点-邮件-2026-11-16-1938.md`，扫描终点北京 11-16 19:36）与 WorkBuddy HTML 选题扫描 1116-18（北京 11-16 18:02）。两份扫描都把 Qwen4-Max、uBlock 当成各自台账里的旧题，但本号“已发过”与“近 3 天速览已收”里都没有；Claude 对照去重、现查 HN 前台热度（Algolia，北京 11-16 23:5x：Qwen4-Max 731 分 / 367 评第一，uBlock 505 / 146 第三）后列候选，Develata 定“A 千问与混元合帖、B uBlock”。Project Zero 已在 1114 速览收过，不再收；黄仁勋复任传闻沿 1112 期风控不收。

派工单 `.handoff/2026-11-16-1115-evidence.md`（`barking handoff` 生成的文件名日期与“本机时区为美东”按美东算，已改为 11-16 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor` 通过。三组执行方：A、B 为 Claude Sonnet 5.5（`claude-sonnet-5-5`），C 为 Claude Haiku 5.5（`claude-haiku-5-5`，effort high），北京 11-17 约 00:0x 派出；C 约 4 分钟、B 约 13 分钟、A 约 22 分钟交回。

## 主帖

1. 千问 Qwen4-Max 与混元 Hy4 flash：[Qwen 博客](https://qwen.ai/blog?id=qwen4-max)（官方只给日期 11-16；JS 渲染，opencli 取得中英文版）、[Hy4 flash 官方页](https://hunyuan.tencent.com/research/100145?langVersion=zh)（只给日期；扫描给的 hy.tencent.ai 链接返回 567）、[腾讯混元官方帖](https://x.com/TencentHunyuan/status/2122217770314301469)（北京 11-16 14:38，“18x”对 Haiku 5.5，正文未用）、[AA Qwen4 Max](https://artificialanalysis.ai/models/qwen4-max)、[AA Qwen3.8 Max](https://artificialanalysis.ai/models/qwen3-8-max)、[Vals Index](https://www.vals.ai/benchmarks/vals_index)、[HN 50513215](https://news.ycombinator.com/item?id=50513215)。
   - 千问“降约六成”：中文版 26.58→11.16 元（-58%），英文版 $4.43→$1.67（-62%），同一项 Work360 测试；IT之家标题“降低超 60%”与其正文 58% 不一。正文合写“约六成”。
   - AA 每任务 $10.02→$3.27（-67%，比官方说的还多）；Vals 每次测试 $8.73→$9.33（没降），tokens/test 26.59m→27.27m 几乎持平而单价下调，费用反升原因不明，正文只写“没降”。AA 两页只写 Reasoning Yes、未标 max/high；Vals 未标档位。
   - 扫描说“1/20（中文）与 18x（英文）差 2 倍”不成立：1/20 是 WorkIndex 上对 Hy4，18x 是官方帖对 Haiku 5.5（high），对象不同；即便同口径，5% 与 5.6% 也只差约一成。
   - 42.1 / 51.4 取自官方页图表数据（`a-hy4flash-zh.html` 第 212 行），盲测表在官方页是图片（alt “【中】模型盲测结果.png”），以图 11b 为据。
   - 权重：HF（author=tencent）与 ModelScope（Tencent-Hunyuan）在北京 11-17 00:25 只有 Hy4-preview 两个；腾讯官方 GitHub 只有 preview（A 组）。交付前再查一次。
   - 扫描称的“token 最多少 50%”“编程基准 19.2 对 44.4”在官方页都找不到；Arena 名次页被 Cloudflare 拦，只见转述，不用。
2. uBlock Origin 通用 XSS：[研究者文章](https://iter.ca/post/ubo-uxss/)（北京 11-16 02:30，发布后改过）、[GitHub 安全公告](https://github.com/gorhill/uBlock/security/advisories/GHSA-cmqw-9r2w-6wv2)（北京 11-16 00:27）、[1.76.0 release](https://github.com/gorhill/uBlock/releases/tag/1.76.0)、[Firefox 扩展商店](https://addons.mozilla.org/en-US/firefox/addon/ublock-origin/)、[HN 50513160](https://news.ycombinator.com/item?id=50513160)。
   - 公告字段“Patched 1.75.4”，时间线列 1.75.2、1.75.4、1.76.0 共四项修复；1.76.4 的“[security]”条目指向另一条 Mozilla bug，与本漏洞无关（扫描过读）。“升级到 1.75.4 仍不安全”无来源，不写。
   - 点击诱骗作者没实现（“I didn’t bother actually implementing this”），演示页按钮可见；Chromium MV2 只写到能取出 WAR secret；uBO Lite 作者称“similar but less problematic”，公告未提。
   - 触发前提：公告写需有以 WAR 响应的 prevent-xhr/prevent-fetch 规则；作者称默认规则里有一条 `lootdest.*`，B 组未独立复核默认规则集。
   - B 组 `evidence-B.md` 把公告字段解释为“修复链终点的概括”，反向核验指出与时间线自相矛盾；以清单为准，取证文件保留原样。

## 速览

- [Apple 健康 AI 研究](https://www.apple.com/newsroom/2026/11/apple-launches-research-study-to-explore-the-use-of-ai-for-health/)（官方日期 11-15）。TechCrunch 把机型写成“iPhone 15 or later”，官方是 15 Pro 及以上；“约 12 个月”只见 TechCrunch。
- 【风控待定】[欧洲央行副行长 Vujčić 演讲](https://www.ecb.europa.eu/press/key/date/2026/html/ecb.sp261116~db374b56ab.en.html)（11-16）。5.5 万亿美元是“到 2030 年累计资本开支预测”，Reuters 写成 financing need；速览不写这个数。
- 【风控待定】[美联储理事 Barr 演讲](https://www.federalreserve.gov/newsevents/speech/barr20261116a.htm)（11-16，脚注称个人观点）。
- [韩联社：税务师考试第一名自首](https://www.yna.co.kr/view/MYH20261116020700038)（北京 11-16 22:56）。中文媒体“AI 智能眼镜作弊”是早期指控，本人自述为微型摄像头 + 生成式 AI + 手机；主管机关原文未取到。当事人姓名不入正文与卡片。
- 未收：华为云 CloudRobo-WM1.2（发布后官方稿未找到；扫描的 98.35% 实为 CR-VLA-Dual）；Xena 博客“Lean 检查过的定理”（11-12 观点文）；IMF 总裁“超 1 万亿循环融资”（底稿不明）；美国与盟友 AI 供应链合作（未读到宣言原文、涉外国政府）；黄仁勋复任传闻（FT 匿名信源，风控）；Project Zero（1114 已收）。

## 备注

- 批注底图由 Claude 用 `barking shot cropbox/stack` 从取证截图裁出（记在 `a-shots-log.jsonl`、`b-shots-log.jsonl`）。`12h-vals-table-rows.png` 由 Claude 用 opencli 展开 Vals 折叠区后整页截图裁切，1 倍像素比（opencli 截图不能设缩放）。
- A 组截图 12 号之后用字母后缀（02a、10b、11a、11b、12a–12h），避开 B 组 13 号起的区间。
- Claude 自查存档：`a-claude-hf-hy4.json`、`a-claude-modelscope-hy4.json`。
