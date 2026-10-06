# 1005 B组取证：OpenAI textGrain

只取证，不写发布稿。采集北京时间2026-10-06（任务期号1005）；全部浏览器session为1005-b。状态仅指对应说法的支持程度，不代表整个交付已通过验收。B2本地PDF、pdftotext与关键页图未交付；EU Code PDF原件未读到。

## 事实清单

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1a 全球 API 仅部分模型可选，默认关闭 | L1 | https://openai.com/index/eu-text-provenance/ | Starting today, API customers globally will be able to opt in to text watermarking for select models. Text watermarking will remain off by default in the API. | 10-b-openai-rollout.png | b-openai.txt:34；公告日起可 opt in，不是全球默认上线 | 已找到 |
| B1b EU ChatGPT/Codex 未来数周逐步推送 | L1 | https://openai.com/index/eu-text-provenance/ | Over the coming weeks, we will add an invisible watermark to eligible ChatGPT and Codex text output in the European Union. | 10-b-openai-rollout.png | b-openai.txt:36；eligible；不是公告当日所有输出已经加水印 | 已找到 |
| B1c EU 所有套餐；首发不设全球默认 | L1 | https://openai.com/index/eu-text-provenance/ | Rolling out text watermarking in the EU. Over the coming weeks, we will introduce text watermarking to eligible ChatGPT and Codex users across all plans in the EU only. We are not making text watermarking a global default at launch. This regional approach gives us room to learn from real-world use and feedback. | 10-b-openai-rollout.png | b-openai.txt:202；该图为开头，完整后续措辞见全文 | 已找到 |
| B1d 文本检测器限制申请，不向公众开放 | L1 | https://openai.com/index/eu-text-provenance/ | Providing detector access to researchers and expert organizations. Approved researchers and expert organizations can apply starting today. In accordance with the Code of Practice⁠ | — | b-openai.txt:206；逐案批准；勿与公开图像/音频检测器混淆 | 已找到 |
| B1e 80%/95% 为特定内容与长度的检出率 | L1 | https://openai.com/index/eu-text-provenance/ | Shorter or more constrained text is harder to detect. At a target false positive rate of 1%, our detector identified watermarks in about 80% of 200-token passages, compared with about 95% of 400-token passages, for content such as psychology. Detection rates were substantially lower for content such as mathematics, where there is less flexibility in word choice. | 11-b-openai-detection.png | b-openai.txt:54；厂商自报；目标 FPR 1%；200/400 tokens；心理学；不是通用 AI 检测准确率 | 已找到 |
| B1f 同义词替换检测率 92→66→17 | L1 | https://openai.com/index/eu-text-provenance/ | Editing can weaken the watermark. In an evaluation of 400-token passages, replacing 10% of words with synonyms reduced detection from about 92% to 66%. Replacing 25% of words reduced it to 17%. | 11-b-openai-detection.png | b-openai.txt:56；400 tokens；替换词比例10%/25%；本段未标 FPR | 已找到 |
| B1g 替换实验样本与图注 | L1 | https://openai.com/index/eu-text-provenance/ | Editing can substantially weaken the watermark signal. This chart shows how replacing 10% or 25% of the words in a passage affects detection. Results are based on watermarked English responses to questions from ELI5⁠ | 11-b-openai-detection.png | b-openai.txt:64；带水印英文 ELI5 回答；图全含标题、坐标、图例与图注 | 已找到 |
| B1h 对 SynthID 的比较是自家测试 | L1 | https://openai.com/index/eu-text-provenance/ | In our evaluations, textGrain matched or exceeded the performance of other approaches we tested, including SynthID for text. Even so, strong performance under ideal conditions does not guarantee reliable detection in everyday use. | 11-b-openai-detection.png | b-openai.txt:50；含 ideal conditions 限定；不是独立验证 | 已找到 |
| B1i 质量表列顺序 | L1 | https://openai.com/index/eu-text-provenance/ | Unwatermarked text (Astra, max) | 12-b-openai-quality.png | b-openai.txt:76；左列无水印，右列 Watermarked text；派工摘要把含义写反；全部8行见下表 | 与说法不符 |
| B1j 质量无明显差异为 OpenAI 判断 | L1 | https://openai.com/index/eu-text-provenance/ | Across the benchmarks we use to assess Astra, our latest frontier model, we do not see meaningful performance differences with and without watermarking. | 12-b-openai-quality.png | b-openai.txt:70；Astra,max；不自行把数值差异解释为显著或无损证明 | 已找到 |
| B1k 检测解释的限制 | L1 | https://openai.com/index/eu-text-provenance/ | A watermark does not measure human contribution. It can indicate that an OpenAI system generated or processed part of a passage, but not how much human judgment, editing, or creativity went into it. | 13-b-openai-limits.png | b-openai.txt:190；五条完整限制分别逐字摘录 | 已找到 |
| B1l 检测解释的限制 | L1 | https://openai.com/index/eu-text-provenance/ | A watermark does not establish ownership or responsibility. It does not determine who owns the text, whether its use was lawful, whether disclosure was required, or who is responsible for it. | 13-b-openai-limits.png | b-openai.txt:192；五条完整限制分别逐字摘录 | 已找到 |
| B1m 检测解释的限制 | L1 | https://openai.com/index/eu-text-provenance/ | A watermark does not identify the user. It does not associate a person, organization, account, prompt, or conversation with the text. | 13-b-openai-limits.png | b-openai.txt:194；五条完整限制分别逐字摘录 | 已找到 |
| B1n 检测解释的限制 | L1 | https://openai.com/index/eu-text-provenance/ | A watermark does not verify accuracy. It does not tell you whether a passage is true, misleading, harmful, or presented in the right context. | 13-b-openai-limits.png | b-openai.txt:196；五条完整限制分别逐字摘录 | 已找到 |
| B1o 检测解释的限制 | L1 | https://openai.com/index/eu-text-provenance/ | The absence of a detected watermark does not prove human authorship. Text generated with OpenAI tools may be too short, edited, or translated for detection to work reliably. It may also come from an unsupported model, predate watermarking, or have been generated by another company’s tools. | 13-b-openai-limits.png | b-openai.txt:198；五条完整限制分别逐字摘录 | 已找到 |
| B1p 开源与报告更新尚属计划 | L1 | https://openai.com/index/eu-text-provenance/ | Our text watermarking technology, textGrain, adds an invisible statistical signal to the model’s word choices. Our detector looks for that signal to assess whether a passage contains an OpenAI watermark. More details about how textGrain works can be found in our technical report⁠ | 11-b-openai-detection.png | b-openai.txt:46；we plan / coming weeks；未核到已经开源 | 已找到 |
| B3a 法律机器可读标记义务与例外 | L1 | https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-50 | Providers shall ensure their technical solutions are effective, interoperable, robust and reliable as far as this is technically feasible, taking into account the specificities and limitations of various types of content, the costs of implementation and the generally acknowledged state of the art, as may be reflected in relevant technical standards. | — | b-eu-article50.txt:45；Article 50(2)；同段要求 machine-readable；有标准编辑、不实质改变输入/语义等例外 | 已找到 |
| B3b 适用日期与准则性质 | L1 | https://digital-strategy.ec.europa.eu/en/policies/code-practice-ai-generated-content | The obligations under Article 50 of the AI Act (transparency obligations for providers and deployers of generative AI systems) address risks of deception and manipulation, fostering the integrity of the information ecosystem. These transparency obligations, applicable from 2 August 2026, complement other rules like those for high-risk AI systems or general-purpose AI models. They pertain to marking and detection of AI-generated content and labelling of deepfakes and certain AI-generated publications. | — | b-eu-code.txt:19；欧委会页称2026-08-02适用，官方未给时刻；这是法定义务，Code 本身是自愿合规工具 | 已找到 |
| B3c Code 两部分与入口 | L1 | https://digital-strategy.ec.europa.eu/en/policies/code-practice-ai-generated-content | Section 1: Providers - Rules for marking and detection of AI-generated and manipulated content | — | b-eu-code.txt:34；Section 2 是部署方标签；完整准则PDF抓取失败，未把帮助中心的200-token例外当作已直接对照EU原件 | 部分支持 |
| B4a Google 已在 Gemini app/web 使用文本水印 | L1 | https://deepmind.google/blog/watermarking-ai-generated-text-and-video-with-synthid/ | Today, we’re expanding SynthID’s capabilities to watermarking AI-generated text in the Gemini app and web experience, and video in Veo, our most capable generative video model. | — | b-google-text.txt:14；2024-05-14官方页；支持已部署，不足以断言每一模型/地区/短文本均可检出 | 已找到 |
| B4b Google 当前公开验证范围 | L1 | https://deepmind.google/models/synthid/ | Simply upload the image, video or audio clip to your chat, and ask if it’s been created or altered by Google AI. Gemini will check for a SynthID watermark, and let you know if it finds one. | — | b-google.txt:24；图像/视频/音频；未找到公众可验证 Gemini 文本水印的服务。开源算法与拥有Gemini检测密钥不同 | 部分支持 |
| B4c Anthropic 8/2 新模型范围 | L1 | https://support.claude.com/en/articles/16266773-how-claude-marks-ai-generated-content | New models will mark AI-generated content from day one. Claude models launched in the EU on or after August 2, 2026 will support machine-readable marking at launch. Generated text will carry embedded watermarks, and generated files will include Content Credentials (C2PA) where supported. | — | b-anthropic-help.txt:29；范围是8/2及以后在EU推出的模型；旧模型另处过渡期；页面当前支持表已存档 | 已找到 |
| B4d Anthropic 全球范围 | L1 | https://support.claude.com/en/articles/16266773-how-claude-marks-ai-generated-content | Regions. Marking will apply to output from supported models wherever Claude is offered, worldwide. | — | b-anthropic-help.txt:53；supported models；无需借用二手媒体 | 已找到 |
| B4e Anthropic 为什么全球推出 | L1 | https://www.anthropic.com/news/claude-text-watermark | We’re implementing watermarking to comply with the EU AI Act. Anthropic, along with several other major AI model providers and around 190 total signatories, signed the EU Code of Practice on Transparency of AI-Generated Content in July 2026. This requires AI system providers to use methods of “marking” AI-generated text. We’re applying watermarking globally at launch because we don't yet have a durable way to scope it by region. However, we will continue to evaluate different approaches, and will share updates when we have them. | — | b-anthropic.txt:71；官方称尚无持久可靠的地域限定办法；本页2026-08-14，末注9/1更新 | 已找到 |
| B4f Anthropic 用户无法关闭 | L1待核 | https://support.claude.com/en/articles/16266773-how-claude-marks-ai-generated-content | 未找到明确的 cannot disable / no opt-out 原句 | — | 官方两页描述模型级、全球支持范围；不能仅据未列开关就证明无法关闭。站内检索未补到直述 | 未找到一手来源 |
| B5a OpenAI 帮助中心全文与 EU | L1 | https://help.openai.com/en/articles/8912793-provenance-signals-in-openai-generated-content | We’re implementing text watermarking for ChatGPT users in the EU to comply with the EU AI Act, and in line with our commitments under the EU Code of Practice on Transparency of AI-Generated Content, which OpenAI along with several other major AI providers have signed.  | — | b-help.txt:89；全文 b-help.json/.txt；相对更新时间不可转换成精确发布时间 | 已找到 |
| B5b 不插入隐藏字符 | L1 | https://help.openai.com/en/articles/8912793-provenance-signals-in-openai-generated-content | No. Text watermarking changes the statistical pattern of word choices; it does not insert hidden characters or add watermark-only tokens. The watermark is not visible to readers, and copying and pasting the text does not introduce hidden material. | — | b-help.txt:195；统计词选择，不是删隐藏空格就能移除 | 已找到 |
| B5c 短文本/代码例外由帮助中心自述 | L1 | https://help.openai.com/en/articles/8912793-provenance-signals-in-openai-generated-content | To account for these limitations, the EU AI Act Code of Practice on the Transparency of AI-Generated Content does not require watermarks in outputs shorter than 200 tokens—about 150 words in English—or in code snippets. | — | b-help.txt:169；少于200 tokens、code snippets；EU Code原件尚未直接核到，此项仅确认OpenAI如此写 | 已找到 |
| B5d 开启 API 水印不等于得到检测器 | L1 | https://help.openai.com/en/articles/8912793-provenance-signals-in-openai-generated-content | No. Text detector access is currently limited to approved research and academic organizations working to improve how reliably text watermarking works, how easy it is to use, and how clearly the technology and its results can be explained. This is consistent with our commitments under the EU AI Act Code of Practice on the Transparency of AI-Generated Content. | — | b-help.txt:214；批准的研究和学术组织 | 已找到 |
| B2 技术报告作者、版本、FPR、实验对应 | L1 | https://cdn.openai.com/pdf/e9508624-d767-41b6-a26d-e34ca798ada6/textgrain-entropy-calibrated-watermarking-for-language-model-text.pdf | “The conditional false positive rate is α under these assumptions” (p.6) | — | web直接读取20页报告；详见下文。PDF本地下载被自动审批拒绝，未完成pdftotext及14号图 | 部分支持 |
| B6 热度及跟进媒体 | L4/L5/L6 | https://news.ycombinator.com/item?id=49966293 | HN API：points=63，num_comments=52 | — | 另一个派工HN帖只有1分/0评论；媒体精确时刻及AIHOT动态快照见下文 | 已找到 |
| B7 夸大说法实际实例 | L4/L5 | https://gizmodo.com/openai-is-adding-text-watermarks-in-the-eu-because-regulation-works-2000821852 | 见下方原句与上下文 | — | 找到英文 all text outputs 的过宽表述、中文“改写25%即失效”；未找到可靠“全球已上线/公开作业检测器”实例 | 部分支持 |
| B8 社区质疑 | L6 | https://news.ycombinator.com/item?id=49968652 | 见社区线索表 | — | HN评论points=null，不可称已核高赞；Reddit有可见9分评论 | 部分支持 |

## 质量表复核（原站整表，Astra,max）

| Benchmark | 无水印 | 有水印 |
|---|---:|---:|
| Artificial Analysis Intelligence Index |49.57 points|49.76 points|
| AutomationBench |34.09%|34.86%|
| DeepSWE v1.1 |72.80%|71.68%|
| Terminal-Bench 4.0 |53.90%|56.06%|
| Terminal-Bench Science 0.1 |56.90%|60.00%|
| BrowseComp |87.92%|87.35%|
| HealthBench Professional |64.27%|64.60%|
| GPQA Diamond |94.44%|93.94%|

## B2 报告核查与缺口

报告20页，封面日期2026-10-05，未给时刻/版本号。p.1作者：宾大 Xiang Li、Qi Long；耶鲁 Garrett Wen、Xiaohong Chen；OpenAI Arzav Jain、Florent Joly、Mike Lam、Qingquan Song、Weijie Su。p.6 的 α 是在独立性等理想假设下的条件误报率，固定部署密钥仍需经验校准。web全文查找 ELI5 无命中；已读方法、参考文献和相关工作中未核到网页的80/95或92/66/17实验表，也未找到本方法改写/翻译攻击的实测结果，不能拿文献标题当报告实验。网页同义词实验的FPR仍未核实；不能从上一张图的1%推断。未发现数字冲突，只是报告未补齐对应测试。PDF下载被自动审批拒绝，因此本项未完成本地原件存档、pdftotext、pdftoppm与14号图，不能称完整验收。

## 发布时间与热度快照

| 来源 | 发布时刻（北京时间；原时区） | 抓取与数字 |
|---|---|---|
| OpenAI主公告 | 官方仅2026-10-05；无时区/时刻，北京具体日期时刻未核实 | b-openai.json无发布时间meta；不能用AIHOT的23:00冒充官方时刻 |
| 技术报告 | 2026-10-05；官方未给时刻 | 20页，web读取 |
| Anthropic说明 | 2026-08-14，未给时区/时刻；末注2026-09-01更新 | b-anthropic |
| Google文本水印公告 | 2024-05-14，未给时区/时刻 | b-google-text |
| EU Code政策页 | 最后更新2026-07-31，未给时刻 | b-eu-code |
| TechCrunch | 2026-10-06 04:36:48（2026-10-05T20:36:48+00:00） | 原站标题 OpenAI will start watermarking ChatGPT's text in the EU；b-techcrunch |
| The Verge | 2026-10-06 02:08:39（2026-10-05T18:08:39+00:00；可见2:08 PM EDT） | 原站标题 OpenAI is adding text watermarking in ChatGPT and Codex；5条评论；b-verge |
| IT之家 | 2026-10-05 23:58:02（页面未标时区；按该站常用北京时间解释） | 标题 OpenAI 将在欧盟为 ChatGPT 和 Codex 文本输出添加隐形水印；b-ithome |
| HN原文主帖49966293 | 2026-10-05 23:38:55（15:38:55Z） | b-hn-search.json抓取时63分、52评论，精确抓取时刻见JSON |
| HN派工帖49968716 | 页面只给相对时间，未换算 | 09:19:15北京，1分、无评论；实际链接unite.ai，不是主帖 |
| Reddit帖1wymu43 | 2026-10-06 06:54:20（2026-10-05T22:54:20+00:00） | 09:19北京，11分、13评论；b-reddit.json |
| AIHOT事件页 | 列最早10-05 23:00，最新10-06 04:36；聚合时间非官方时间 | 浏览器快照可比范围当前181、峰值192（10-06 07:00），10篇/8来源、48小时22主体；不是互动人数 |

AIHOT事件 https://aihot.news/story/b92e615b-0baf-4821-a3ac-e621dde3db2c ；单条 https://aihot.news/items/xx5mgdemrqmqw5zw410sbawcz 的AI评分60。web较早结果曾显示事件当前186/首页184，浏览器稍后181，按不同抓取时点分别记录，不能合并为同一读数。Reuters以textGrain/水印定向检索未找到报道，不等于确定未报道。TechCrunch/The Verge/IT之家均已打开原站，非仅见聚合转述。

**是否有可信的大规模社交讨论：未证实。** 已核有HN几十条评论及一个小型Reddit帖，AIHOT声称22主体是聚合计数；不足以称“大规模爆发”。Reddit站内textGrain搜索返回大量无关词项，不能作为完整覆盖或无讨论的证据；未做全网或全部社交平台抽样。

## 可能的吠点

- 公告将首发范围与时间限定为EU、eligible和未来数周，全球API则默认关闭；来源B1a–c。
- 水印阳性不区分原创与编辑，不量化人类贡献，也不确认作者/真实性；来源B1k–o、b-help。
- 1%是目标误报率，不是“检测结果有99%概率正确”，更不是对所有文本/用户的保证；网页B1e及报告p.6条件校准。
- 同义词替换400-token样本的17%是剩余检出率，不是零检出，也不是所有改写技术的普遍结论；B1f–g。
- 本次报告没有补上对应ELI5实验的FPR，网页又明确还会更新报告，编辑不能替它补参数；B2。
- 帮助中心另给24种EU语言评测：500个合成英文问题再译23语，1%FPR下西班牙语69.0%、罗马尼亚语42.2%；与心理学80/95是不同测试，不能并成“总体准确率”；b-help.txt:175。
- API开启水印并不自动取得检测器；B5d。

## 社区线索（L6，不当事实）

| 评论 | 逐字摘句/关注点 | 得分与限制 |
|---|---|---|
| https://news.ycombinator.com/item?id=49968652 | “Is there a sort of adversarial attack that is more common?”；引用替换25%后检出17%，质疑抗常见改动能力 | API points=null，未公开；不能称高赞 |
| https://news.ycombinator.com/item?id=49971419 | “1% false positive rate is completely unacceptable” | API points=null；这是评论者评价，不是误报危害的实证 |
| https://news.ycombinator.com/item?id=49972925 | 指向OpenAI旧文关于改写/翻译与非母语写作者风险 | API points=null；旧文尚未在本轮独立存档，不引用为已核事实 |
| https://old.reddit.com/r/ChatGPT/comments/1wymu43/i_read_openais_whole_post_about_watermarking/pe445l8/ | “Surely an organisation that applies and gets approved for access is likely to include academic institutions, or companies providing services such as TurnItIn?” | 可见9分；反驳“教师/客户一定无检测途径”的过宽推断，不证明TurnItIn已获准 |
| https://old.reddit.com/r/ChatGPT/comments/1wymu43/i_read_openais_whole_post_about_watermarking/pe41gx8/ | 作者自述维护指南并制作去水印工具 | 相关商业利益必须披露；不把卖方效果声明当独立证据 |

HN公开API原始评论树保存在b-hn-main.json；Reddit只保存公开帖/评论内容、分数、时刻和链接，未存账号栏/抓取者头像/登录信息。

## 夸大说法实例

- 英文Gizmodo：原句“Passing off ChatGPT’s work as your own is about to get tougher—at least if you live in the European Union. On Monday, OpenAI announced that it would start inserting an invisible watermark, imperceptible to the naked eye but identifiable by machine, into all text outputs generated by its AI models, including ChatGPT and its coding agent Codex.” 发布方Gizmodo、作者AJ Dellinger；北京时间2026-10-06 05:50（原页October 5, 2026, 5:50 PM ET，按当日EDT转换）。争议是首段all text outputs范围过宽；须同时说明首句已有EU限定、下一段明确eligible和未来数周，且说明检测器不公开。因此不能指控该文宣称全球已开启或公开检测器。原站 https://gizmodo.com/openai-is-adding-text-watermarks-in-the-eu-because-regulation-works-2000821852 。
- 中文ic.work：标题“OpenAI 推出文本水印 textGrain，改写 25% 即失效背后的欧盟合规账本”；正文“这项改动替换两成多词汇即告失效的技术”。发布方ic.work，署名Evan Neural。本轮浏览器可见日期2026-10-06、未给时区/时刻；较早搜索索引显示2026-10-05，二者不一致，不自行选定精确发布时间；17%不是完全失效。原站 https://www.ic.work/article/openai-releases-textgrain-text-watermarking 。
- 英文Interestana：原句“The company has not disclosed the technical specifics of how textGrain operates, only that it is "invisible" and "machine-readable."”（引号以存档原文为准）；该站自己标AI-drafted；可见2026-10-05，meta为2026-10-05T18:08:39.000Z，即北京10-06 02:08:39，但恰与其所引The Verge的时间相同，是否为该站独立发稿时刻未核实；报告已公开方法，该句过宽。它虽标The Verge来源，不能把该错误归到The Verge。原站 https://interestana.com/articles/openai-is-adding-text-watermarking-in-chatgpt-and-codex-009tct91 。
- 对清单所举“全球所有文本已经加水印/一键检测作业/检测器已公开”未找到可完整核验的中英文实际实例，不凑。上面列的是本轮实际找到的邻近夸大。

## 扫描说法勘误

1. “北京23:00发布”只见聚合时间，OpenAI官网只给日期；未核实时刻。
2. 质量表确有八行，但派工摘要“加水印vs不加”的列含义反了；正确见整表。
3. Anthropic“8/2”有官方出处，指新模型支持上线；既有模型另行补支持。全球范围成立于supported models；“用户不能关”的明确原句未找到。
4. 检测器不公开；图像/音频验证工具公开不能推成文本公开。
5. EU未来数周、eligible、所有套餐，API全球可选、默认关：均得到原文支持。
6. 92/66/17的FPR仍未知，报告未补出对应ELI5实验；不写成已证1%。
7. TechCrunch和The Verge确有原站报道；Reuters本轮未找到。
8. 派工HN链接仅1分，但不是原文主帖；原文主帖63分/52评论。

## 假设与验收边界

- 文件名按本期派工：sources使用b-前缀，指定交付evidence-b.md/capture-log-b.md为例外；图片用10–13-b-前缀兼顾编号和归属。
- 期号1005不随北京时间跨日改名；日期只有年月日时不擅自补时区或时刻。
- 无公开分数记缺失；搜索未命中只代表检索范围内没找到，不等于不存在。
- 未运行检测器、未独立复现厂商实验、不做法律合规结论。
- 没有commit、push、删除文件，未写发布稿；未改1004和其他组材料。
