# 1117 来源与制作状态

期号1117（上期1116+1）；线索为 Develata 回贴的两份扫描：ChatGPT 定时任务的 stateful rescan（`AI吠点-邮件-2026-11-18-0708.md`，扫描终点北京 11-18 07:05）与 WorkBuddy HTML 选题扫描 1118-06（北京 11-18 06:05）。主帖由 Develata 11-18 下午按 Claude 推荐拍板：GPT-6.2 系统卡漏报障碍（HN 北京 11-18 16:0x 为 1246 分 / 770 评）与微软 MAI Code 2“98% 性能、40% 成本”（HN 349 分 / 152 评）。本期为“对你”句、吠点至多 3 条、纠正进标题等试行规则（2026-11-18 起）的第一期。

派工单 `.handoff/2026-11-18-1117-evidence.md`（`barking handoff` 生成的文件名日期与“本机时区为美东”有误，已改为 11-18 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`：Browser Bridge 扩展未连接，派工单写明截图直接走 `barking shot`。三组执行方：A、B 为 Claude Sonnet 5.5（`claude-sonnet-5-5`），C 为 Claude Haiku 5.5（`claude-haiku-5-5`，effort high），北京 11-18 约 16:25 派出；C 组约 6 分钟、A 组约 24 分钟、B 组约 22 分钟交回，未催交。取证方全部是 Claude 家族，反向核验（Codex）须逐行核对事实清单与存档。

## 主帖

1. GPT-6.2 系统卡：[系统卡](https://deploymentsafety.openai.com/gpt-6-2-astra-sol)（Published November 17, 2026，官方未给时刻；4.1.3.1 Blocker Deception、4.1.3.2 Broken Search Tool，官方 PDF 第 90–92 页）、[发布博客](https://openai.com/index/introducing-gpt-6-2-astra-sol/)、[新模型页](https://developers.openai.com/api/docs/models/gpt-6.2-astra)、[旧模型页](https://developers.openai.com/api/docs/models/gpt-6-astra)、[帮助中心：Work 与 Codex](https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex)、[Codex 定价页](https://developers.openai.com/codex/pricing)、[HN 50535230](https://news.ycombinator.com/item?id=50535230)。第三方：[AA 模型页](https://artificialanalysis.ai/models/gpt-6-2-astra)、[Vals 模型页](https://www.vals.ai/models/openai_gpt-6.2-astra)（均未入正文）。
   - Figure 35/36 为图片，数值由 A 组读图、Claude 复核 Figure 35。“with dev prompt” 只有 GPT-6.2 两个模型，旧模型没有加提示词的数。
   - WorkBuddy 扫描的“降价只到 27.2 万 token、长请求比上一代还贵”不成立：GPT-6 Astra 旧模型页同样有 272K 以上整单 2 倍 / 1.5 倍加价，同口径新版仍便宜（Claude 北京 11-18 16:1x 直取两页核对）。
   - Free、Go 不在任何官方名单里，但页面未明说“不可用”。
   - 未找到明确夸大实例；外媒标题 “twice as likely to hide” 与数字相符（22.28/11.09≈2.0），“hide” 比原文 “omit” 重。The New Stack 被 Cloudflare 拦，只见摘录。
2. 微软 MAI Code 2：[发布博客](https://microsoft.ai/news/introducing-mai-code-2-and-mai-code-2-flash/)（北京 11-18 02:15）、[Ignite 总览](https://microsoft.ai/news/microsoft-ignite-2026/)、[模型页](https://microsoft.ai/models/mai-code-2)、[GitHub Copilot 价目表](https://docs.github.com/copilot/reference/copilot-billing/models-and-pricing)、[GitHub 更新日志](https://github.blog/changelog/2026-11-17-mai-code-2-and-mai-code-2-flash-in-github-copilot)、[HN 50535885](https://news.ycombinator.com/item?id=50535885)；微软 AI CEO 苏莱曼海外社媒长文（fxtwitter 存档 `b-suleyman-fx.json`）。
   - 博客脚注 “MAI-Code-2 cost is shown at its 50% promotional price”；Ignite 总览页与苏莱曼长文写 “40% of the price”，全文无 promotional。
   - 价目表脚注给绝对价与促销期：$0.50 / $2.50 至 2027-05-17，之后 $1.00 / $5.00。
   - 盲测偏好率博客旧版 67%（Wayback 北京 11-18 08:36）、模型页旧版据 B 组为 68%，现三页均 64%；对象 GPT-6 Sol。未入正文。
   - AA 的 FrontierCode 1.1 页已标 Legacy，读数与微软图不一（Opus 52.2 vs 56.0）；AA 上未找到 MAI Code 2 数据。
   - Reuters 401、FourWeekMBA 与 Techzine 403，未读到。
   - 本号推算（不进正文）：成本按字面五折还原约为 GPT-6.1 Sol 的 81%。

## 速览

- [Gemini 免费版 Tasks](https://blog.google/innovation-and-ai/products/gemini-app/tasks-in-gemini-free-new-features/)（北京 11-18 01:00）。
- [Google Wallet 上线美国 iOS](https://blog.google/products-and-platforms/platforms/google-pay/wallet-available-ios-united-states/)（北京 11-18 01:00）。
- [FrontierHarness Eval v1.1](https://runta.com/blog/introducing-frontierharness-eval-v1-1/)（11-17，官方未给时刻；厂商自家评测）。
- [Google/UCL HelpBench](https://blog.google/security/how-google-is-making-cybersecurity-advice-from-ai-more-helpful/)（北京 11-17 15:00）与 [arXiv:2606.24819](https://arxiv.org/abs/2606.24819)。“低于65%”比例博客 1 in 10、论文 14%，不一致，速览不用。
- [OpenAI 医疗行政智能体](https://openai.com/index/advancing-computer-use-for-healthcare-teams/)（11-16，官方未给时刻；评测对象 GPT-6 Astra）。
- [Isomorphic Labs C 轮](https://www.prnewswire.com/news-releases/isomorphic-labs-announces-5-billion-series-c-funding-302942062.html)（北京 11-17 20:00）。
- [MAI Image 2.7](https://microsoft.ai/news/introducing-mai-image-2-7/)（北京 11-18 02:16）与 [AA 图像编辑榜](https://artificialanalysis.ai/image/leaderboard/editing)。
- 备用未收：[MAI Cyber 2 Flash](https://microsoft.ai/news/introducing-mai-cyber-2-flash/)。
- 未收：Windows 编码代理遥测、Siri 每日额度（1116 主帖）；加州起诉 OpenAI（1116 速览）；FBI ARTeX、OpenAI Daybreak 进北约（风控）；ChatGPT Wallet、“Google AI 回答让搜索流量降 40%”（线索未核）。

## 备注

- Reddit 三组均被登录页拦截，无 Reddit 热度。
- openai.com、help.openai.com 在 headless Chrome 下渲染为空白，A 组改用 Codex 定价页与论坛公告截图；系统卡截图取自官方 PDF（宽 1224 px，非 2 倍像素比）。
- 已删除（北京 11-18 收尾时一次删除，取证记录里仍提到它们）：A 组 `images/06-aa-astra-62.png`、`07-vals-astra-rank7.png`、`08-aa-astra-62.png`、`12c-community-availability.png`、`a-tmp-*.png`（8 张）；B 组 `images/17-chart-flash.png`、`21-copilot-price-mai.png`（宽 2000 px 超限）、`26-ignite-lineup-98-40.png`，`sources/b-ignite-wayback-20261118030821.*`（无快照，空）、`b-modelpage-wayback-20261117184519.gz`（已解压）、`b-ghblog.*`、`b-aa-mai.*`、`b-aa-mai2.*`、`b-aa-ms.*`（猜测 URL 的 404）；C 组 `sources/c-aa-image-edit.*`（误用 URL 的 404）。`b-heat.json` 保留作 Reddit 失败记录。
- C 组 `capture-log-C.md` 原含本机绝对路径，已改为仓库相对路径。
