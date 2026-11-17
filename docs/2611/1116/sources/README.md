# 1116 来源与制作状态

期号1116（上期1115+1）；线索为 Develata 回贴的两份扫描：ChatGPT 定时任务的 stateful rescan（`AI吠点-邮件-2026-11-17-1308.md`，扫描终点北京 11-17 13:07）与 WorkBuddy HTML 选题扫描 1117-18（北京 11-17 18:06）。主帖由 Develata 11-17 傍晚按 Claude 推荐拍板：Win11 编码代理记名（HN 北京 11-17 19:2x 为 624 分 / 478 评，两份扫描里最热）与 Siri 每日额度（与普通 iPhone 用户直接相关）。

派工单 `.handoff/2026-11-17-1116-evidence.md`（`barking handoff` 生成的文件名日期与“本机时区为美东”按美东算，已改为 11-17 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor` 通过。三组执行方：A、B 为 Claude Sonnet 5.5（`claude-sonnet-5-5`），C 为 Claude Haiku 5.5（`claude-haiku-5-5`，effort high），北京 11-17 约 19:23 派出；C 组约 4 分钟、A 组约 38 分钟、B 组约 45 分钟交回，未催交。

## 主帖

1. Win11 按程序名记录 AI 编程工具：[研究者博文](https://xusheng.dev/posts/windows-coding-agent-telemetry/)（页脚只有日期 11-16；首发帖北京 11-16 10:00:25）、[Wayback 快照 2026-11-16 23:00 UTC](https://web.archive.org/web/20261116230024/https://xusheng.dev/posts/windows-coding-agent-telemetry/)、[HN 50524736](https://news.ycombinator.com/item?id=50524736)、[微软可选诊断数据说明](https://learn.microsoft.com/en-us/windows/privacy/configure-windows-diagnostic-data-in-your-organization)。
   - **博文 17 日晚被改**：A 组 19:24 存档（`a-blog.*`）与 Wayback 快照都有“The public ISO’s baseline VM had the coding-agent feature disabled by image default, so the evidence does not support saying that it is active on every Windows installation.”一段；Claude 20:1x 重取（`a-claude-blog-v2.*`，`Last-Modified` 北京 19:40:31）这一段已删，其余逐字不变（1480→1437 词），页面无更正说明，作者帖子线程与 HN 评论区未见解释。正文只写“原文曾写……17日晚这句被删，页面没注明改动”，不推测原因。批注图 `win-deleted` 取自 Wayback 快照。交付前再比对。
   - 两个开关：记录功能（CLICodingAgentUsage）只在作者一台 Parallels VM 上见到开启（“service experiment”）；可选诊断数据在美国区域干净安装 Home ARM64 时默认开。博文没说两者是否同一台机器。
   - 上传只证到 DiagTrack 上传队列；队列里的事件来自改名的 cmd.exe。
   - 博文与 Apple 页都按系统深色模式渲染成深色底，荧光笔（正片叠底）在深色底上看不见；批注底图由 Claude 用 headless Chrome `--blink-settings=preferredColorScheme=1` 重渲浅色版裁切，A、B 组的深色取证截图保留作依据。
   - WorkBuddy 扫描所称“转载站写未提供 build 号”“写成 usage history”在 15 篇二手稿中未复现；Tessera、Cloud Updates 未找到。
2. Siri AI 每日额度：[Apple Support 127901](https://support.apple.com/en-us/127901)（页脚 11-16；9-09 起已有此页，11-16 改版加倍数表）、[127893 Siri AI 可用地区](https://support.apple.com/en-us/127893)、[苹果中国 10-31 新闻稿](https://www.apple.com.cn/newsroom/2026/10/apple-intelligence-features-are-coming-to-users-in-china/)、[iCloud+ 价格](https://support.apple.com/en-us/108047)、[WWDC 6-08 新闻稿](https://www.apple.com/newsroom/2026/06/apple-intelligence-brings-powerful-ai-capabilities-into-everyday-experiences/)。
   - 页面不给具体次数；倍数均为 “Up to”；50 GB 档与免费同额。
   - “27.3 beta 2 已实施”只见 MacRumors、9to5Mac、AppleInsider（L4），苹果只写 “starting with the 27.3 release”。
   - 中文帮助页仍是 9 月旧文（截至 11-17 19:24）。
   - 6-08 新闻稿脚注截图 OCR 定位失败，以文本存档为据。

## 速览

- [AMD 完成收购 World Labs](https://newsroom.amd.com/news/amd-completes-acquisition-world-labs/)（11-16）。完成稿无金额；签约稿“all-stock，约 82 亿美元”。
- [Hugging Face × OpenRouter](https://huggingface.co/blog/openrouter)（11-16）。语言模型自带密钥失效，图像、视频、音频不变。
- [Databricks AI Index](https://www.databricks.com/blog/introducing-databricks-ai-index-state-enterprise-ai-tokenomics)（11-16）。编码任务来自 Databricks 私有代码库；成本按标价估算；未写档位。
- [Mem 关停公告](https://get.mem.ai/blog/mem-is-shutting-down)（11-16）。
- 未收：[加州总检察长起诉 OpenAI](https://oag.ca.gov/news/press-releases/it%E2%80%99s-golden-state-not-wild-west-attorney-general-bonta-sues-openai-over)（已存档新闻稿与诉状；涉美国州政府，按风控默认不收，交 Develata 定；跟进 1014 的 8 州诉讼，派工单误写 6 州，C 组已纠正）；英伟达 Vera Rubin“每瓦10倍”、TensorWave 457MW、万拓 One2N（只见 WorkBuddy 扫描，C 组与 Claude 搜索均未找到一手来源）；OpenAI Daybreak 进北约机构、FBI ARTEX（风控）；OpenAI 儿童安全分类器（题材敏感）。

## 备注

- 本机为 macOS，Win11 逆向结论不复现，只核原文、微软文档与二手稿之间的出入。
- 已删除（北京 11-17 收尾时一次删除，取证记录里仍提到它们）：A 组 `sources/a-rss2.*`、`a-ms-required.*`、`a-hn-algolia.*`；B 组 `images/b-tmp-*.png`、`sources/.b-opencli-*.json`；C 组 `sources/c-hn-50520857.*`、`c-hn-50524736.*`、`c-sinoxiv-one2n.*`；Claude 深色版 `images/25-wayback-v1-baseline.png`。
