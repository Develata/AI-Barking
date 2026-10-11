# 1010 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（OpenAI 打分模型报告）、`evidence-b.md`（Grok Bot 理财智能体）、`evidence-d.md`（速览）；抓取日志见 `capture-log-a.md`、`capture-log-b.md`、`capture-log-d.md`；派工单见 `../../../../.handoff/2026-10-11-1011-evidence.md`。

期次更名：本期起初按本机日期（美东 10-11 凌晨）建为 `docs/2610/1011/`，Develata 指正当天仍算 10 月 10 日，已改名为 `1010`（正文文件改为 `doc_1010_publish.txt`）。派工单、`capture-log-*.md`、`evidence-*.md` 与脚本日志中的 `1011` 路径是当时的工作记录，保留原样。批注底图由 `m-raw-crops.py` 从取证截图裁切（像素未改）。

取证执行方：A、B 组 Sonnet 5.5 子代理；D 组 Haiku 5.5（effort high，按 2026-10-10 决定的试用）。Haiku 试用回读结果：8 条全部交回，摘句均能在存档中检出，主动查出扫描的 5 处错（Mxc 版本号、反编译 token 数、Cockroach 未检出的统计数、尼康措辞、ARC 榜单口径）；Reuters 正文未取到（401），由 Claude 用 firecrawl 补存 `d-d3-reuters.txt`。未发现身份泄露或断章摘句。

## 主帖一：OpenAI 打分模型破坏任务环境

- 报告：https://alignment.openai.com/misalignment-reports/damaging-the-task-environment-to-trigger-a-reset/ （`a-report.txt`、`a-report-browser.txt`；页头 Incident date Oct 6, 2026 / Report updated Oct 9, 2026）
- 列表页：https://alignment.openai.com/ 的 Misalignment Reports（`a-list.txt`；三份报告 First posted 10/9/26）
- 同日另两份报告（对照“加固措施”）：`a-sib-obtaining-public-statistics-with-disallowed-requests.txt`、`a-sib-sending-disallowed-web-requests-and-reaching-a-public-file-service.txt`
- OpenAI 员工 Marcus Williams 首发帖（自述 Monitoring at OpenAI）：`a-x-marcus-raw.json`、`a-x-marcus-thread.json`
- 媒体与中文报道（未入正文）：The Decoder、虎嗅、FourWeekMBA、MadRobot、Tech AI Magazine、AI Weekly 等（`a-*.txt`，夸大实例见 `evidence-a.md` 第 6 节）
- 热度：HN 无本报告提交（取数至北京 10-11 16:03）；员工帖约 5.1 万浏览

## 主帖二：Grok Bot 理财智能体误发

- 当事人 Shane Mac 线程（2026-10-06）：https://x.com/ShaneMac/status/2107486740491669879 及后续各条（`b-fx-*.json`）
- Business Insider 口述（2026-10-09）：https://www.businessinsider.com/personal-ai-agent-grok-bot-posted-bank-details-company-slack-2026-10 （`b-bi.txt`）
- Grok Bot 官方文档（现行）：https://docs.x.ai/grok-bot/approvals-security-and-privacy 、/computer-and-apps 、/faq 等 22 页（`b-xai-docs-md/`）；事故前 Wayback 快照（`b-wayback/`，2026-09-20、09-21）
- 官方产品页与 changelog：x.ai/news、x.ai/news/introducing-grok-bot、Grok Bot changelog（`b-xai-*.txt`、`b-xai-release-notes.md`）
- 其他报道（未入正文）：WSJ（付费墙，首两段）、Futurism、Cybernews、t3n 等（见 `evidence-b.md` B5、B7）
- 热度：HN 61 分 / 66 评；Mac 线程单条浏览 27 万–337 万
- 隐私：Mac 首帖附图（Slack 截图，含部分遮盖的账户数字与同事账号）未截、未摘；`b-fx-2107495338873389121.json` 中被引帖（私人账户的个人财务自述）正文已由 Claude 替换为占位符

## 速览

- 纳德拉长文：https://x.com/satyanadella/status/2108931348857827686 （`d-d1-nadella.txt`、`d-d1-nadella.json`）；The Verge（`d-d1-verge.txt`）
- 英伟达 × Reflection AI：Reuters（`d-d3-reuters.txt`）；Reflection Beam 公告（`d-d3-reflection-beam.txt`）；FT 付费墙未读
- 微软 MXC：Windows 开发者博客（`d-d4-mxc.txt`）、GitHub releases（`d-d4-mxc-releases.txt`）
- 游戏反编译：momo5502.com（`d-d5-momo.txt`）
- Cockroach Labs：官方博客（`d-d6-cockroach.txt`）
- 未收：ARC-AGI-2 88.06%（`d-d2-*`，榜单口径未核）、尼康 Small World in Motion（`d-d7-*`，风控候选，待 Develata 定）、OpenAI gpt-rosalind-discovery 定价（`d-d8-pricing.txt`，热度不足、无发布说明）
- 热度：`d-heat.json`
