# D 组搜索台账

检索时间：北京时间 2026-10-06 09:14–09:24；仅作发现原站的线索，事实以本目录浏览器存档为准。所有 OpenCLI 浏览器访问固定使用 `1005-d`。

## 路由与失败

- 已读 agent-reach、opencli-usage、opencli-browser、smart-search。OpenCLI doctor：daemon 1.8.7 / extension 1.0.24 / connectivity 均 OK；未升级。Agent Reach 1.5.0 检查为最新。
- `conda run -n dl agent-reach doctor --json` 失败：dl 环境不存在；随后直接调用已安装的 `agent-reach doctor --json` 成功。社交 active_backend=null 是未实时验证，不等于未安装。
- Exa：`Codex 28天 每天 重置 10月5日 IT之家 InfoQ 36氪`，1 次，免费 MCP 限额；未配置新密钥。
- HN Algolia 两条 curl 写盘命令被自动审批拒绝（approval required by policy / AskForApproval Never），没有生成目标文件。改为公开 HN 页面正文取证，不绕过拒绝的命令。
- X「显示原文」的点击虽未报错，DOM 仍为机器翻译；停止重复点击，读取当前公开帖子组件的白名单字段 `full_text` / `note_tweet` / 发帖时间及公开计数。未读取 Cookies、抓取者资料或账户配置。初次未完成的 `d-tibo-28-original` 批次由 Ctrl-C 中止，未交付其文件。

## 网页检索（web.run）

web.run 的主要查询组合按主题归并如下（不把主题组数当调用次数）：

1. Tibo Codex 28 天 每天 重置 2026 10 5；Reflection Beam October 5 2026 TechCrunch Reuters；ChatGPT cartoonists signatures Nieman October 5 2026。
2. Codex 28天 2026 IT之家；Codex 每天 重置 新智元；Reflection Beam Reuters October 2026；site:x.com/thsottiaux Day 1 50%。
3. Reuters Reflection Beam；Codex 28 重置，分别限定 ithome.com、36kr.com、infoq.cn。
4. 精确标题 Nvidia-backed Reflection unveils（reuters.com）；新智元站内 2026/10/05 28；36kr 与 InfoQ 的 Tibo 28。
5. 刚刚 Tibo承诺 机器之心；OpenAI立下28天军令状 新智元；Codex 28天 重置（排除聚合/论坛）；ChatGPT cartoonists signatures Hacker News。
6. jiqizhixin 连续28天；reuters Reflection Beam 2026-10-05；help.openai.com October 30 10x；October 29 20x。
7. jiqizhixin Tibo 28；Codex 28 天 重置（排除若干聚合/论坛）；reuters technology Reflection October 5 2026。
8. help.openai.com Pro October 30, 2026；openai.com Pro October 29, 2026；ClaudeDevs reset October 2026；ChatGPT cartoonists signatures OpenAI（找独立跟进）。
9. support.claude.com October 22 reset；ClaudeDevs reset Sep 2026；ithome Codex 28；28天 重置 量子位。
10. Pro tiers 10 20；thsottiaux October 30 20；support.claude.com banked reset October；Reuters 精确标题。

上列按主题归并为10组；另有一次 Reuters 猜测路径的 open（不是搜索，失败）。原站最终由搜索找到的 `/technology/nvidia-backed-...` 路径成功读取；猜测的 `/technology/artificial-intelligence/...` 未采用。

发现并实际读取的原站：IT之家、量子位、Reflection、TechCrunch、Reuters、Nieman Lab、A.V. Club、OpenAI 帮助中心及官方博客、X 公开帖子、HN。搜索命中凤凰/新浪/36氪上的新智元或机器之心转载、Reuters 联合供稿站等，只作定位线索，没有以转载代替原站存档。

## X 搜索（2 次）

1. `(from:thsottiaux OR from:OpenAI OR from:OpenAIDevs) since:2026-10-04 (reset OR "Day 1")`，Latest：取得 Tibo 第 1 天提速与 28 天承诺两张卡片，未出现新的实际重置帖。
2. `(from:ClaudeDevs OR from:claudeai) since:2026-09-22 (reset OR limits)`，Latest：取得 7 张卡片，含 9/22、9/28 重置说明，以及 10/1 特定文档/设计对话两周用量优惠。搜索是有界样本，不证明未命中的帖子不存在。

未继续第 3 次 X 搜索；后续只读取已定位的直接帖子链接。
