# 1014 来源入口

正文：`../doc_1014_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（8 州诉讼）、`evidence-b.md`（科学家公开信与 CSO 专家）、`evidence-c.md`（Scion 与 Mononoke）、`evidence-d.md`（速览）；抓取日志：`capture-log-*.md`、`*-shots-log.jsonl`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），派工单按 2026-10-15 提速要求设 60 分钟目标。耗时：D 约 30 分钟、C 约 25 分钟、B 约 35 分钟、A 约 3 小时（中途工具调用排队）。opencli 浏览器扩展本期未连接，截图退回 `barking shot render`/`pdf` 与 Playwright。Haiku 第五次试用：7 条全部交回，主动纠正了 CVSS 8.7 的评分方（Anthropic 而非研究方）与 Jev `box` 不在 danger zone。

选题：一份 ChatGPT 扫描（北京 10/15 11:01）与一份 WorkBuddy 扫描（12:00）。Develata 选 8 州起诉 OpenAI + 科学家公开信 + Scion/Mononoke 署名争议，追加两份扫描都漏掉的 REA 进速览。扫描勘误：ChatGPT 把欧盟调查标成“已发跟进”（只有 OpenAI 的 Hugging Face 事件 1002 发过，Anthropic 三家机构事件未发过），SneakPeek 的 CVSS 8.7 写成研究方评分；WorkBuddy 的公开信“四条诉求”“点名四家公司”与原文不符（三条、未点名），Noam Brown“温度传信号”是信发布前近一个月的播客发言，“禁止自我改进模型”写成 8 州共同诉请（只见于阿肯色州诉状），“公共妨害”只在 AR、WV 诉状。风控排除：欧盟 AI Office 信息调取（涉外国政府，未进速览）。

隐私：诉状涉及的 4 名未成年死者，入库文本存档中姓名替换为 `[MINOR-A]`–`[MINOR-D]`（案名中家属姓氏同时被遮，案号保留）；含原名的 PDF/HTML 原件只存本地与网盘。公开信名单页脚的学者媒体联络邮箱在入库文本中替换为 `[email]`。正文不写任何死者信息。

## 一手来源

- Reuters：https://www.reuters.com/world/eight-us-states-sue-openai-claiming-chatgpt-harms-children-2026-10-14/
- 阿肯色州诉状（23CV-26-2984）：https://media.ark.org/ag/2026-10-14-Filed-Complaint-Case-No.-23cv-26-2984.pdf ；新闻稿 https://media.ark.org/ag/2026-10-14-OpenAI-lawsuit_FINAL.pdf
- 亚利桑那州诉状：https://www.documentcloud.org/documents/28813407-arizona-v-openai/
- 艾奥瓦州诉状：https://www.iowaattorneygeneral.gov/media/cms/261014_Petition_EC459B522F957.pdf
- 爱达荷州诉状：https://www.ag.idaho.gov/content/uploads/2026/10/OpenAI-2026-10-14-Complaint.pdf
- 蒙大拿州诉状：https://content.govdelivery.com/attachments/MTAG/2026/10/14/file_attachments/3817137/001%20OpenAI%20Complaint.pdf
- 内布拉斯加州诉状：https://ago.nebraska.gov/sites/default/files/doc/Court%20Filing_28.pdf
- 西弗吉尼亚州诉状：https://ago.wv.gov/sites/default/files/2026-10/2026.10.14%20--%20Open%20AI%20Complaint%20FILED.pdf
- The Verge：https://www.theverge.com/ai-artificial-intelligence/1011500/openai-state-lawsuits-chatgpt-safety ；Platformer：https://www.platformer.news/openai-states-lawsuit-liability/ ；联合早报：https://www.zaobao.com.sg/news/world/story20261015-9838957
- 科学家公开信：https://ai-security-letter.org/
- CSO Online：https://www.csoonline.com/article/4234854/ai-agents-arent-going-rogue-frontier-labs-are-skipping-security-basics.html
- Anthropic 评测越界事件复盘（7/30）：https://www.anthropic.com/news/investigating-incidents-cybersecurity-evals
- sunshowers：https://sunshowers.io/posts/anthropic-mononoke/ ；HN https://news.ycombinator.com/item?id=50094694
- Anthropic Scion：https://www.anthropic.com/engineering/redesigning-source-control ；Mononoke README https://github.com/facebook/sapling/blob/main/eden/mononoke/README.md
- 速览：REA https://github.com/morluto/rea 、https://rea.tools/ ；OpenAI 弃用页 https://developers.openai.com/api/docs/deprecations ；Deep Review https://research.google/blog/analyzing-technical-documents-with-deep-review/ ；SneakPeek https://noma.security/noma-labs/sneakpeek-how-the-preview-pane-in-claude-code-desktop-can-exfiltrate-your-data ；OpenCode 2.0 https://anoma.ly/notes/opencode-2-0/ ；AA 语音榜 https://artificialanalysis.ai/speech-to-speech 、xAI https://x.ai/news/grok-voice-3
