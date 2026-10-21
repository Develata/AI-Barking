# 1021 速览取证 B 组

采集时间：2026-10-21 14:51–14:55，北京时间。范围：三条指定官方候选；不改正文、cards.toml、公共 README 或 fact-check。

## 可并入事实清单的速览事实

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 称17种设计蛋白能结合重组蝎毒蛋白，尚未验证解毒。 | L1 | ✅ 已对照一手来源 | https://www.anthropic.com/research/claude-scorpion-toxin-binder-design | 35字；厂商研究报告。实验针对重组神经毒素；211种设计中17种达到其结合标准，非药物、非抗毒血清研发成功。只保留本次公开报告层面的核验。 |
| 谷歌为 Chrome 增加可疑网站警告，需开启增强型保护。 | L1 | ✅ 已对照一手来源 | https://blog.google/security/our-latest-defenses-against-evolving-web-scams/ | 29字；新警告适用 Enhanced Protection。没有采用“十倍安全”的营销概括；不推断所有设备已经同步上线。 |
| OpenAI 称已收购药物研发协作平台 Orchestra Bio。 | L1 | ✅ 已对照一手来源 | https://openai.com/index/openai-to-acquire-orchestra-bio/ | 34字；原文当前标题 acquires、正文 has acquired。不能仅按 URL 的 to-acquire 推断“拟收购”。未来整合效果仍是目标。 |

## 原文证据

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 17种设计满足结合标准，实验仅针对重组毒素，未测解毒 | L1 | https://www.anthropic.com/research/claude-scorpion-toxin-binder-design | “17 met our criteria.” / “Our results show binding only against recombinant toxins” / “we didn’t test any design for neutralization in vitro” | 本组任务无需配图 | 公开正文日期 Oct 20, 2026；HTML datePublished=2026-10-20T00:00:00.000Z（北京10/20 08:00，可能是日期占位，不能据此断言实际发布小时），dateModified=2026-10-20T16:04:01.000Z（北京10/21 00:04:01）。 | 已找到 |
| Chrome 开启增强型保护时新增可疑网站警告 | L1 | https://blog.google/security/our-latest-defenses-against-evolving-web-scams/ | “Safe Browsing's site warnings in Chrome when Enhanced Protection mode is enabled” / “Suspicious site warnings notify users” | 本组任务无需配图 | 正文日期 Oct 20, 2026；HTML datePublished=2026-10-20T17:00:00+00:00，即北京10/21 01:00。 | 已找到 |
| OpenAI 已收购 Orchestra Bio；平台用于协同药物开发 | L1 | https://openai.com/index/openai-to-acquire-orchestra-bio/ | “OpenAI has acquired Orchestra Bio” / “a platform that helps biopharma teams coordinate the complex process of drug development.” | 本组任务无需配图 | 正文 October 20, 2026，所查存档未取得带时区精确发布时间；不强行换算北京具体日期与小时。 | 已找到 |

## 存档及核验边界

- Anthropic：`b-anthropic-scorpion.html`、`.headers`、`.txt`；补充 agent-reach Jina 阅读结果 `b-anthropic-scorpion-jina.txt`。已亲读研究页全文；未独立复现实验、未另读技术报告或数据集。
- Google：`b-google-scams.html`、`.headers`、`.txt`。已亲读正文，并经网页读取工具交叉核对。
- OpenAI：`b-openai-orchestra.html`、`.headers`、`.txt`。已亲读正文，并经网页读取工具交叉核对。
- 批量清单 `b-fetch-list.tsv`；抓取机器记录 `b-fetch-log.jsonl`。三条均 HTTP 200、ok；逐个确认 txt 是正文而非验证页。
- 去重：已核对当前 `daily-scan.md` 的“已发过”和“近3天速览已收”；三事件均未出现。未与本期三条主帖重复。
- 范围限制：不另做社区热度采样；入选热度沿用上游候选线索，本文件不把这种线索写成已独立验证的热度结论。

## 内部限定（不作为速览评论）

- Anthropic：211个设计由180个模型筛选设计加31个人类团队在 Claude 协助下补入的设计组成；不能写成完全无人参与。17个设计均只对单个目标满足标准；没有成功的多毒素结合设计。两家实验室的结合验证也不能等同细胞中和或动物保护试验，且 LqhIII 仅由 Adaptyv Bio 评估。
- Google：“十倍”按增强/标准保护用户收到的平均安全警告数计算，非实测被骗概率降低十倍。服务器端 Gemini 会分析具有高置信诈骗迹象的网页文本；这与可疑网站警告是同篇不同机制，不混写为全体 Chrome 用户默认保护。
- OpenAI：收购事实与“让药物更快到达患者”的愿景分开，原文未在本次公告中量化实际疗效、研发提速或药物获批成果。
