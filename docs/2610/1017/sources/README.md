# 1017 来源入口

正文：`../doc_1017_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（OpenCode Console 泄露）、`evidence-b.md`（Jack Clark 与 Anthropic 经济情景）、`evidence-c.md`（Meta 商业条款）、`evidence-d.md`（速览）；抓取日志：`capture-log-a.md`–`capture-log-d.md`；派工单：`../../../../.handoff/2026-10-17-1017-evidence.md`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），派工单 60 分钟目标。耗时：D 约 19 分钟、A 约 122 分钟、C 约 123 分钟、B 约 134 分钟（均在 60 分钟催交后收尾；B 组 15 号图未做成）。

选题：一份 ChatGPT 扫描（北京 10/18 11:03）与一份 WorkBuddy 扫描（12:00）。WorkBuddy 推荐的 NIST 曲线种子（1016 已发）、OpenAI 新增 4 份越权报告（1015 速览已收）、Muse Spark Open（1015 已发）、DeepSeek 与 Anthropic 用工研究（1016 已发）均重复；Jack Clark 的 44 万亿两份扫描都没有，是 Claude 在 AIHOT 热点榜上发现的。Develata 选 OpenCode + Clark + Meta（Meta 以能独立复现为条件，C 组已复现）。

## 一手来源

- OpenCode 官方通报：https://x.com/opencode/status/2111519637049319818 （编辑版；初版 https://x.com/opencode/status/2111516673953935833 ）；Anthropic 员工 Lydia Hallie：https://x.com/lydiahallie/status/2111563956666110287 ；OpenCode 员工 dax：https://x.com/thdxr/status/2111572618340188393 ；OpenCode Console 文档：https://opencode.ai/v2/docs/console 、https://opencode.ai/v2/docs/console/providers/ ；HN 讨论（含用户转贴邮件）：https://news.ycombinator.com/item?id=50133401
- IMF/世行年会 Per Jacobsson 讲座录像：https://www.youtube.com/watch?v=Rqt9kWoTJbc （IMF 官方频道；日程 https://www.imf.org/en/meetings/2026/annual/schedule/2026/10/17/per-jacobsson-foundation-lecture-212473 ）；Anthropic 经济情景页：https://www.anthropic.com/institute/econ-scenarios ；工作论文：https://www-cdn.anthropic.com/files/4zrzovbb/website/cf58f84d46a4a76bf5a5b039ac695fba6b80041c.pdf ；路透社报道（转载存档 `b-b24-yahoo-reuters.txt`、`b-b25-thestar-reuters.txt`）；IT 之家报道（`b-b15-ithome.txt`）
- Meta 商业条款预览：https://www.facebook.com/legal/commercial_terms_preview?locale=en_US 、?locale=en_GB 、?locale=fr_FR 、?locale=zh_CN ；现行自助广告条款：https://www.facebook.com/legal/self_service_ads_terms?locale=en_US ；The Verge：https://www.theverge.com/tech/1013409/meta-removes-language-that-may-have-allowed-it-to-scrape-news-sites ；Nieman Lab：https://www.niemanlab.org/2026/10/metas-new-commercial-terms-will-allow-the-scraping-of-news-sites-legal-experts-say/ ；Wayback 10-06 快照：https://web.archive.org/web/20261006082850/https://www.facebook.com/legal/commercial_terms_preview
- 速览：Waymo 多伦多 https://waymo.com/blog/2026/10/waymo-in-toronto/ （复用 1016 存档）；苹果审核指南 https://developer.apple.com/news/?id=vvzx6cul 、https://developer.apple.com/news/?id=92th7e5i ；亚马逊 Bee https://www.aboutamazon.com/news/devices/amazon-personal-ai-bee-frontier-bee-eclipse ；腾讯 Hy-Memory https://memory.hunyuan.tencent.com/ 、https://github.com/Tencent-Hunyuan/Hy-Memory ；Lean https://leodemoura.github.io/blog/2026-10-15-lean-now-ships-with-verified-kernels/

## 待核验与未完成

- Clark 口述年份：IMF 录像自动字幕写“by 2040”，本机 Whisper 听成“by 2030”，路透社写 2030；正文不引口述年份，待人工听一遍。Clark 口述的 44 万亿与 Anthropic 极端情景 44.4 万亿是否同一数字，Clark 未点明，正文分开陈述。
- OpenCode 官方账号 10/16–18 全部帖子未能枚举（opencli 搜索超时）；Nathan Lambert 原帖未找到（只有二手引文，未用）。
- Meta 条款只查了 en_US、en_GB、fr_FR、zh_CN 四个语言版。
- 未进正文：RTX Spark 新闻稿与博客的两个起售价（地区与配置口径未明，速览容不下限定词）；Replit 与 Anthropic（只有 TechCrunch 转述的 CEO 说法）；Elorian Glimpse、Reflection Beam 的 AA 评测（热度低）；Amazon Atlas（风控）。
