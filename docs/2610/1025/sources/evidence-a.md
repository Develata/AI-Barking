# A 组取证：十年前 CNN 胜过前沿 LLM 的皮肤癌测试（1025 期）

执行方 claude-sonnet-5-5。取证时段北京时间 2026-10-26 20:32–21:00。时间一律北京时间（UTC+8），括号保留原时区；官方只给日期时写“官方未给时刻”。本机时区美东（EDT），北京 = 本机 + 12 小时。医学题：只记录实验设计与结果，不含任何诊疗建议。

存档文件在 `docs/2610/1025/sources/`，前缀 `a-`（html、txt、headers、fetch-log、shots-log、heat）；截图在 `docs/2610/1025/images/01–12`。HTML 原件、zip、截图只存本地（不入库）。

## 0. 先看这几条（与派工单“已知情况”相比的新发现）

1. **这篇 10-25 的文章是 10-11 长文的 CNN 扩展版。** 作者 10-11 在个人站 drmagnuslynch.com 发了同题系列的前作 “Can frontier LLMs diagnose skin cancer?”（14 min read，只测两个 LLM，同一 200 例、同一批 LLM 数字：84/85、97/96 留下、32/22 放走、3/4 黑色素瘤放走）。10-25 的 lynchlab.net 文是 5 min 精简版，加入 CNN 对照。因此 LLM 部分的数字 10-11 起就已公开；“新”的只是 CNN 这一列。两处共用同一个数据包（zip 的 README 指向 drmagnuslynch.com 那篇）。存档：`a-drmagnuslynch.html|.txt`。
2. **“更准”与“更不容易漏掉癌”在原文表格里恰好相反。** 表中黑色素瘤被放走：GPT-6 Astra 3/30，Claude Fable 5.1 4/30，CNN 预设阈值下 18/30（调阈值后 2 或 1）。六分类 CNN 对 LLM 是 125 对 84/85，分诊却是 CNN 对 LLM 漏得更多。原因（作者 10-11 文自述）：LLM “managed this largely by keeping almost everything”，良性只放走 32 与 22 例。
3. **“默认阈值”不是原文用词。** 原文写 “its preset threshold”；模型页（lynchlab.net/skin-ai-inception）写明预设阈值是在验证集上定的、使 90% 的目标病灶得分高于阈值（“research threshold set on the validation images so that 90% of the lesions in question scored above it”）。“matched”（调后）在原文里没有定义，调后的良性放走数没有给，所以“多转诊多少例”原文没有数字。
4. 作者在文末自述：分析与初稿由 Claude Code 生成，被测的两个 LLM 之一也是 Anthropic 的（Claude Fable 5.1）。10-25 文里有这句，需要在正文里点明。

## 1. 事实清单

级别按 EDITORIAL 事实分级：作者个人博客与自家数据包算作者署名的一手材料，暂按 L1（作者本人发布）记，**L1 还是 L3（第三方原始页面，仅限其自身数据）由 Claude 定**；评论、二次传播按 L5/L6 记。状态列只用四个词，附加说明都在“条件/口径/时区”列。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1a 作者身份：Dr Magnus Lynch，皮肤科医生 | L1（作者自述） | https://lynchlab.net/llm-skin-cancer | “I am a dermatologist.” / “Dr Magnus Lynch is a Consultant Dermatologist and Mohs Surgeon at Guy's Hospital and Honorary Reader (equivalent of Associate Professor) at King's College London. His clinical work covers skin cancer, Mohs and facial reconstructive surgery, and lasers; his research covers the cell biology of skin cancers and scars and the use of AI in dermatology.” 实验室页：“The Lynch Lab is based in St John's Institute of Dermatology at King's College London, on the Guy's Campus.”（https://lynchlab.net/） | 01 | 状态说明：自述，身份只有作者自家站点自述，未另查 King's/GMC 登记；GIGAZINE 写成 “Lynch Institute”，系误写 | 已找到 |
| A1b 发布日期 | L1 | 同上 | “By Dr Magnus Lynch · 25 October 2026 · 5 min read” | 01 | 官方未给时刻。HN 提交 2026-10-26 01:15:38 北京（2026-10-25T17:15:38Z）。前作 10-11 见 0 节 | 已找到 |
| A1c 被测模型与设置：GPT-6 Astra、Claude Fable 5.1、Inception v3 | L1 | 同上；数据包 `llm-skin-cancer-test.zip` 内 README.txt | “I tested the ability of GPT-6 Astra and Claude Fable 5.1 to diagnosis skin cancer” / “Both models got the same prompt at default settings, and neither refused a case.” README：“Models: GPT-6 Astra (OpenAI API) and Claude Fable 5.1 (via OpenRouter). Default settings, no system prompt, no cap on reply length, one run per case.” | 04 | Claude 走 OpenRouter 转发（README 自述）；我对原始回复复算：两模型均有推理 token（GPT 合计 17,557，Claude 23,719），即“默认设置”下推理处于开启状态，档位原文未写 | 已找到 |
| A1d 200 例构成 | L1 | 同上 | “I drew 200 lesions at random, all confirmed by histopathology: 100 that should not be discharged: 30 melanomas, 25 basal cell carcinomas, 25 squamous cell carcinomas (including Bowen’s disease) and 20 actinic keratoses. 100 that are safe to discharge: 50 benign naevi (moles) and 50 seborrhoeic keratoses.” / “Every lesion was biopsied, including the benign ones, so these are harder to diagnose: a mole that somebody decided to cut out is one that worried a clinician.” | 02 | 10-11 文补一句：“I drew 200 lesions at random within six diagnoses”（按病种分层抽取）与 “Real clinics see far more harmless lesions than cancers. I used an even split”（50:50 不代表临床患病率）。我核对 cases.csv：MEL30/BCC25/SCC25/ACK20/NEV50/SEK50，200 个不同 lesion_id，safe 100 / not safe 100 | 已找到 |
| A1e 输入与提示词 | L1 | 同上；zip `prompts.txt` | “The LLMs were given both images and nothing else, and asked, as a clinician might use them, for the most likely of the six diagnoses (or “unsure”), a confidence, a second choice, and whether the lesion could be safely discharged with no treatment, biopsy or follow-up.” prompts.txt：“You are assisting a dermatologist. You are shown two images of the same skin lesion: the first is a clinical close-up photograph, the second is a dermoscopic image.” / “Task 2. Decide whether this lesion can be safely discharged, meaning it needs no treatment, no biopsy and no follow-up. Answer "yes" or "no".” | 04 | 先临床近照、后皮肤镜图；无年龄、部位、病史；要求只回 JSON（含 diagnosis、confidence、second_choice、discharge、features）。10-11 文：“It got both images and nothing else (no age, no body site, no history) and was made to commit to a decision” | 已找到 |
| A1f 每例运行次数 | L1 | 同上 | “Each LLM case was run once; answers vary between runs.” | 08 | 10-11 文补充复跑：图 1 那例对每个模型再跑 3 次，“GPT-6 Astra discharged it twice and kept it once. Claude Fable 5.1 kept it all three times”，“The figures in this article are from the first run of each case.” | 已找到 |
| A1g 数据包与代码 | L1 | https://d2gaqzonlm544h.cloudfront.net/skin-ai/data/llm-skin-cancer-test.zip | “The cases, prompts, every LLM reply and the scoring code are in one download” | 10 | 已存档 `a-data-zip/llm-skin-cancer-test.zip`（562,309 字节，仅本地；响应头 Last-Modified: Sun, 25 Oct 2026 17:11:30 GMT = 北京 10-26 01:11，比 HN 提交早约 4 分钟；README 另有 “Added 12.10.26” 的补充实验，为 100 例黑色素瘤的患者式提问，不在本期表格内）与解压文本 `a-data-zip/x/`（cases.csv、prompts.txt、results/*.jsonl、scripts/*.py；未运行其中任何脚本）。CNN 的逐例输出不在包内，只有 LLM 回复，所以 CNN 数字无法独立复算 | 已找到 |
| A2 六分类正确数：CNN 125、GPT-6 Astra 84、Claude Fable 5.1 85 | L1 | 同上（表格与图 3） | 表：“Correct diagnosis (of 200) / GPT-6 Astra 84 (42%) / Claude Fable 5.1 85 (42%) / CNN (Inception v3) 125 (62%)”；“Correct in first or second choice 134 / 138 / 173”；“Melanoma as first choice (of 30) 8 / 11 / 13” | 07、05 | **GPT 是 84，Claude 是 85**（我用原始回复复算：GPT 84，Claude 85，top-2 为 134 与 138，一致）。125/200 = 62.5%，原文写 62%。原文**没有置信区间或显著性检验**；仅说 “Two hundred lesions are enough to show the pattern but not to rank close results”。我按 Wilson 95% 区间复算（非原文）：CNN 55.6–68.9%，GPT 35.4–48.9%，Claude 35.9–49.4%（`a-wilson.out.txt`） | 已找到 |
| A3a LLM 的“能否放走”结果 | L1 | 同上（表格） | 表：“Unsafe lesions kept (of 100) 97 / 96 / …”；“Safe lesions discharged (of 100) 32 / 22 / …”；“Melanomas discharged (of 30) 3 / 4 / …” | 07 | “放走”由模型自己回答 “yes/no”。我复算：GPT 留下 97 个应留的、放走 32 个良性、放走 3 个黑色素瘤；Claude 96、22、4；被放走的“不应放走”病灶全部是黑色素瘤（GPT：case_038、138、200；Claude：015、038、147、200；两者同放：038、200；并集 5 例，与 10-11 文 “5 of the 30 melanomas were sent home by at least one model” 一致）。10-11 文：“Neither sent home any other kind of cancer.” | 已找到 |
| A3b CNN 预设阈值下的分诊 | L1 | 同上（表格）；https://lynchlab.net/skin-ai-inception | 表：“Unsafe lesions kept (of 100) … 77 at its preset threshold; 98 or 99 when matched”；“Safe lesions discharged (of 100) … 76 at its preset threshold”；“Melanomas discharged (of 30) … 18 at its preset threshold; 2 or 1 when matched” | 07、11 | 由此（我的推算，原文未直接写）：CNN 预设阈值下放走 23 个“不应放走”的，其中 18 个是黑色素瘤，另 5 个是其他癌/光化性角化病，分病种原文未给；良性被误留 24 个。分诊敏感度/特异度原文未写成这两个词；按表计：GPT 97%/32%，Claude 96%/22%，CNN 77%/76%。黑色素瘤被放走率：GPT 3/30（Wilson 95% 约 3.5–25.6%）、Claude 4/30、CNN 18/30（约 42.3–75.4%）。模型页：“At the thresholds used here, 6 of the 30 melanomas scored below the threshold on their dermoscopic image and 14 of 30 on their clinical photograph.” | 已找到 |
| A3c ChatGPT 扫描的 3/30、4/30、18/30（措辞需改） | L1 | 同上 | 同 A3b | 07 | 三个数字与表一致。**“默认分诊阈值”是扫描方的说法**，原文是 “preset threshold”（见 A3d） | 已找到 |
| A3d “默认/预设阈值”指什么 | L1 | https://lynchlab.net/skin-ai-inception | “Thresholds: each score is compared with a research threshold set on the validation images so that 90% of the lesions in question scored above it, separately for each image type. For dermoscopic images they are 17.0% (malignant) and 21.7% (malignant or actinic keratosis); for clinical close-ups 35.5% and 53.2%. On the internal test, the malignant threshold caught 92% of malignant dermoscopic images while passing 81% of benign ones, and 91% of malignant clinical photographs while passing 69% of benign ones.” / “The thresholds are research settings, not a safe cut-off.” | 11 | 即：在验证集上定为 90% 召回的研究阈值，分图像类型。10-25 文：“they were not used to choose the model or set its threshold”（200 例没参与定阈值） | 已找到 |
| A3e CNN 的放走判断如何由分类概率换算 | L1 | 同上 | 文章：“It scored both images of each lesion, combined them with most of the weight on the dermoscopic image, and chose from the same six diagnoses.” 模型页只说每个分数与上述阈值比较（“malignant”与“malignant or actinic keratosis”两档，分图像类型） | 04、11 | 两张图的合并权重具体数值、哪一档阈值对应“放走”、两档如何组合，文章与模型页均**未写**；zip 里没有 CNN 代码。三个数并排对不上：表中“放走黑色素瘤”18/30；模型页单图阈值下低于阈值的黑色素瘤皮肤镜图 6/30、临床图 14/30；六分类错的 17 例里首选为良性的只有 14 例（8 痣 + 6 脂溢性角化病）。即“放走”口径既不等于单图阈值、也不等于六分类 argmax 落在良性，原文没写，此处不替作者解释 | 部分支持 |
| A4 CNN 调阈值后的漏放数与代价 | L1 | 同上（表格） | “98 or 99 when matched”（应留的被留下）；“2 or 1 when matched”（黑色素瘤被放走） | 07 | 数字：被留下 98 或 99，黑色素瘤放走 2 或 1。**“matched”原文无定义**，也没说 98/99、2/1 各对应哪个 LLM（顺序未标）；**调后良性放走数、转诊/活检比例原文未给**，所以“多转诊多少例”无法从原文得出。作者是否事后在这 200 例上调：原文只说预设阈值没用这 200 例，“matched”是事后在这 200 例上对 LLM 取齐的推断（我的推断，原文没说）。DEV 的转述写 “When Lynch matched the CNN's discharge rate to the LLMs”（L5，属二手解读，原文无此句） | 部分支持 |
| A5a 训练数据来源与规模 | L1 | 同上；模型页 | “fine-tuned all layers on 68,011 dermoscopic and clinical images from the ISIC Archive on a single H100 GPU” / 模型页：“87,206 labelled ISIC Archive images were divided so that images of the same patient, the same lesion, or near-identical photographs always stay together, and no lesion appears in more than one set. 68,011 were used for training; 8,523 for validation …; 8,517 as the internal test …; 1,286 from a clinic not used in training … as an external test; and 869 were held out for comparison with large language models, including the 600 sample images on this page.” | 04、11 | 68,011 ✓；来自 ISIC Archive，许可含 CC-0/CC-BY/CC-BY-NC | 已找到 |
| A5b 与测试集是否重叠（数据泄漏） | L1 | 同上 | “These 200 test lesions, and every other photograph of the same lesion or patient, were removed from its data (including the validation set) before training, so it had never seen them, and they were not used to choose the model or set its threshold.” | 04 | 作者的处理：同病灶、同患者的其他照片全部剔除，且不参与选模型与定阈值。训练池中是否含 MILK10k 的**其他**病灶：模型页贡献机构里列有 “the MILK study team”，数量原文未给，“原文未提”。（HN 评论 vikramkr 以此质疑“同分布主场优势”，见 A8，属 L6 线索）。另一处不一致：文章称随机样例按钮 “draws from these same 200 lesions”，模型页称样例是 “300 lesions … 600 in all”，二者未对齐 | 部分支持 |
| A5c “约二十分钟”微调 | L1 | 同上；模型页 | “A ten-year-old network, fine-tuned for about twenty minutes, beat both of them at recognising skin cancer.” 模型页：“It made 20 passes over the training images (about 50 seconds each)” | 无（文字在原文第 3 段；未单独截） | 20 × 约 50 秒 ≈ 17 分钟，与“约二十分钟”相容；指的是单次训练耗时，不含前期试错与阈值设定 | 已找到 |
| A5d 只用皮肤镜图还是两张图 | L1 | 同上 | “It scored both images of each lesion, combined them with most of the weight on the dermoscopic image” | 04 | 两张图都用，皮肤镜图权重大；图 1 例“依据皮肤镜图”：“The CNN got the right answer based on the dermoscopy view.” | 已找到 |
| A6 作者列出的局限与结论措辞 | L1 | 同上 | ① “Small numbers. Two hundred lesions are enough to show the pattern but not to rank close results, and 30 melanomas give wide uncertainty on the figure that matters most. Each LLM case was run once; answers vary between runs.” ② “Hard cases. Every lesion was biopsied. On ordinary images from the archive the CNN separated lesions that should and should not be discharged far better (area under the ROC curve 0.96, against 0.86 here).” ③ “Contamination. MILK10k has been public since 2025, so the LLMs could have seen it in training; if anything that would have helped them. The CNN certainly had not.” ④ “Not state of the art. Inception v3 is a baseline. State of the art skin cancer diagnosis AI uses ensembles of images transformers trained on more data, and the regulated products built on them perform considerably better.” ⑤ 结论：“A small, old, specialised network using an outdated architecture recognises skin lesions far better than two frontier LLMs.” ⑥ “I designed the test and made the clinical rulings. The analysis and the first drafts were produced with Claude Code, Anthropic’s coding tool, which runs on a Claude model; one of the two LLMs tested is also Anthropic’s. Scoring was done by script against the histology.” | 08、09、10 | “非前瞻”“不构成临床建议”这两种措辞**文章里没有**；“不能用于诊断”只在 CNN 演示页：“Not a medical device and not for diagnosis. … It will miss skin cancers, including melanoma. It must not be used to decide whether to see a doctor.”（模型页）。患者式提问一段（图 09）：“both LLMs advised seeing a doctor in almost every case, harmless lesions included: for 99 and 98 of a further 100 biopsy-proven melanomas.” | 部分支持 |
| A7 MILK10k 数据集 | L1（ISIC 官方数据集页） | https://doi.org/10.34970/648456 （页面落在 ISIC Archive） | “MILK10k consists of 10480 images, each representing a paired clinical close-up and dermatoscopic image for 5240 lesions.” / “Of the 5240 lesions, 95.7% were biopsied or excised, with histopathology serving as the gold standard for diagnosis.” 页面字段：Published 2025-07-01T14:19:34Z（北京 22:19）；Attributions “MILK study team”；Licenses “CC-BY-NC”；“This content is free to use, modify, and share for non-commercial purposes, as long as you provide credit to the original creator.” 发布方 publisher：ISIC Archive | 无（存档 `a-milk-doi.html|.txt`、`a-isic-home.*`） | 11 个诊断大类；供图机构：维也纳医科大学皮肤科、安卡拉大学、昆士兰大学 Mayne 全科学院、纪念斯隆-凯特琳癌症中心、斯科普里独立研究者；引用文献 10.1016/j.jid.2025.06.1594（未取全文）。原文称 “released through the International Skin Imaging Collaboration in 2025” ✓。图注与许可行见截图 01、03 | 已找到 |
| A8 HN 热度与评论 | L5/L6 | https://news.ycombinator.com/item?id=50233779 | 标题 “A 10-year-old CNN outperforms frontier LLMs for skin cancer diagnosis”，提交者 sungam | 12 | 热度读数：20:38 北京 586 分 / 250 评（页面存档）；20:52 渲染 593 分 / 250 评；Claude 先前 582/248（Algolia）。提交时刻 2026-10-26 01:15:38 北京（2026-10-25T17:15:38Z）。HN 不向匿名读者显示评论得分，下文以页面排序（HN 默认排序，大致按得分与时间）代替，**得分未取到**。`barking heat` 的 Reddit 搜索被登录墙挡住（记入 `a-heat.json` errors），未取到 Reddit 帖 | 部分支持 |
| A9 二次传播里的夸大/写歪实例 | L5 | 见第 4 节 | 见第 4 节 | 无 | 3–5 篇，见第 4 节；中文代表性稿：搜两轮（英文、中文关键词）未找到中文媒体的同题报道，只找到英、西、日、德、俄语稿 | 已找到 |

## 2. 可能的吠点（线索摘要没提或说反的条件与限制）

每条标出处；文章指 https://lynchlab.net/llm-skin-cancer（10-25），前作指 https://www.drmagnuslynch.com/can-llms-diagnose-skin-cancer（10-11）。

1. 分诊口径上 CNN 更差：黑色素瘤被放走 18/30，对 LLM 的 3/30 与 4/30（文章表格，截图 07）；“更准”只说六分类正确数，不等于漏诊更少。
2. LLM 漏得少，靠的是“几乎全留”：良性只放走 32 与 22 例，“A clinic that followed these decisions would still see seven or eight of every ten benign lesions.”（前作）；患者式提问下两模型对 99 与 98 例黑色素瘤都建议就医，作者的读法是 “This was caution and not recognition: melanoma was named as a possibility in only 60 and 44 of those 100 replies.”（前作）。
3. “matched”未定义，调后良性放走数与转诊比例未给（文章表格）；98/99 与 2/1 各对应哪个模型未标。
4. 预设阈值定在验证集 90% 召回，且“放走 18/30”的口径原文没写（表中 18/30、模型页单图 6/30 与 14/30、六分类里首选良性的 14 例，三个数对不上，见 A3e）；在这批“更难的”活检病灶上，黑色素瘤只有 12/30 被留下（模型页：6/30 在皮肤镜图、14/30 在临床图上低于阈值；表中合并后放走 18/30）。作者自己的说法：“The thresholds are research settings, not a safe cut-off.”
5. 测试集是活检过的病灶、良性也经活检、50:50 分层抽取：文章 “Every lesion was biopsied, including the benign ones”；前作 “Real clinics see far more harmless lesions than cancers. I used an even split”，并称 “In real use a model would probably reassure a larger share of harmless lesions than it did here.”
6. “能否放走”的标准答案由作者自己裁定：“I designed the test and made the clinical rulings, including which diagnoses count as safe to discharge.”（前作）；前作 FAQ：50 例痣里 14 例是少见亚型（肢端、先天性、蓝痣、Reed 痣），“a dermatologist might not discharge without thought”，作者称只看 36 例普通痣“does not change the picture”。
7. LLM 单次运行，放走决定不稳：图 1 的黑色素瘤复跑，GPT 三次里放走两次，Claude 三次都留下（前作，只在前作里有；10-25 文只剩 “answers vary between runs”）。
8. 输入只有两张图，无年龄、部位、病史，且被迫二选一（前作）；HN 评论 vintermann 质疑“真实诊疗不止看一张图”（L6）。
9. 分析与初稿由 Claude Code 生成，被测模型之一也是 Anthropic 的（文章 “Run it yourself” 一节，截图 10）；评分由脚本对照组织学。
10. CNN 的训练池里含 MILK study team 贡献的 ISIC 图像，数量原文未给（模型页，截图 11）；文章只保证这 200 例及同病灶、同患者照片被剔除（截图 04）。同源分布优势属推断，未证实。
11. “十年前”指网络结构（2015 年发表），权重是作者这次在 ISIC 的 68,011 张图上微调的；GIGAZINE 写成 2016 年。
12. 作者自己说 CNN 是 baseline，“State of the art … perform considerably better”，且 “I am sure that fairly soon AI will be as good as a dermatologist at diagnosing skin cancer from images. These two models are not there yet.”（前作结尾）。
13. 前作对 LLM 放走黑色素瘤的评价：“When they did discharge, melanomas went home with the harmless lesions. There were only a few, and even a few is too many.” 前作没有 CNN 的分诊数字，所以这句评价在 10-25 文里没有对应的 CNN 结论。
14. 文章与模型页不一致：文章称样例按钮取自“these same 200 lesions”，模型页称 300 例 600 张；原文表格 “Hard cases” 一条的 AUC 0.96 与模型页 0.944/0.875 是不同集合，勿混用。

### HN 社区质疑（L6，仅线索；评论得分未取到，括号给页面顶层顺序）

- 比较不公平、同分布主场优势、提示与评分口径：vikramkr（顶层第 27 条）https://news.ycombinator.com/item?id=50237333 ：“The CNN has a home field advantage because the test images are going to be in distribution with the training images.” 及“all 200 of the the test images were biopsied by the clinician originally … You're asking the model to do the latter and grading it on the former.”
- 过拟合基准、不泛化：Legend2440（顶层第 3 条，首屏靠前）https://news.ycombinator.com/item?id=50234247 ：“You can always beat a specific dataset with a small model by finetuning on a train/test split. But it doesn't generalize well to 'wild' images, because they are out-of-domain.” 同串 EagnaIonat 回复（id=50234615）认为这是正常的分类器训练与测试，要判断脆弱性应换数据集测灵敏度与特异度。
- 只给一张图、真实诊疗不是“是不是癌”而是“要不要活检”：vintermann https://news.ycombinator.com/item?id=50241183 。
- 应给 LLM 上下文里的训练样本再比：biotechbio https://news.ycombinator.com/item?id=50237026 。
- 提交者 sungam 在多条回复里以作者口吻发言（如 “I used it as a baseline to compare to the LLMs … Modern approaches use ensembles of vision transformers and have proven accurate in clinical trials”，https://news.ycombinator.com/item?id=50234562 ；“all models have the potential to miss skin cancer”，id=50234834），账号与作者是否同一人原文未说明，未证实。
- 多数高位评论是“专用模型赢通用模型不意外”（如 tasuki id=50235094、fastball id=50234151），没有对分诊数字的质疑；没有评论讨论 18/30 与 3/30、4/30 的对比。

## 3. 扫描说法勘误

| 扫描/初步核对的说法 | 对照原文 | 结论 |
|---|---|---|
| ChatGPT：“默认分诊阈值下漏放黑色素瘤 GPT 3/30、Claude 4/30、CNN 18/30” | 数字与表一致；原文词是 “preset threshold”，不是“默认分诊阈值”（见 A3d） | 数字对，措辞改为“CNN 预设阈值（验证集 90% 召回）” |
| ChatGPT：“把 CNN 阈值调到与 LLM 相近的风险水平后漏放降到 1–2 例，但转诊/活检比例随之变化” | 表中 “2 or 1 when matched”、被留下 “98 or 99 when matched”；“matched”未定义；转诊/活检比例变化**原文未给数字** | 前半对，后半原文没写 |
| ChatGPT：“LLM 每例只跑一次” | “Each LLM case was run once” | 对；前作另有 3 次复跑，放走决定不稳 |
| Claude 初步：六分类 84 与 85，没分清谁 | GPT-6 Astra 84，Claude Fable 5.1 85（原始回复复算一致） | 补全 |
| Claude 初步：“良性的也做了活检，所以更难” | “Every lesion was biopsied, including the benign ones, so these are harder to diagnose” | 对 |
| Claude 初步：“fine-tuned for about twenty minutes” | 文章正文原句；模型页 20 passes × 约 50 秒 | 对 |
| Claude 初步：图 1 “耳部浸润性黑色素瘤厚度不足 0.8 mm，两 LLM 判脂溢性角化病可放走，CNN 判对（依据皮肤镜图）” | 图注与正文一致（截图 01） | 对 |
| 派工单：“MILK10k 是 ISIC 2025 发布的公开数据集” | ISIC 页：发布 2025-07-01，MILK study team 作，CC-BY-NC，10,480 张/5,240 例 | 对；发布方是 ISIC Archive |
| 派工单默认：这是“今天的新实验” | 前作 10-11 已发布同一 200 例的 LLM 结果；10-25 只是加 CNN | 新发现，见第 0 节 |
| 线索摘要：“CNN 更准” | 六分类正确率 62% 对 42%；分诊漏放 CNN 反而更多 | 不能等同，见第 0、2 节 |
| 热度：HN 582 分 / 248 评（Claude 北京 10/26 20:10 读 Algolia） | 本次读数 586/250（20:38）、593/250（20:52） | 随时间上涨，非冲突 |

## 4. 夸大/写歪的二次传播实例

| 发布方（语种） | 原站 URL | 原句逐字 | 发布时间（北京；括号原时区） | 与原文差在哪 |
|---|---|---|---|---|
| PromptZone（英），作者署名 Ayaka Nkrumah | https://www.promptzone.com/ayaka_nkrumah/a-10-year-old-cnn-outperforms-gpt-4o-on-skin-cancer-diagnosis-3p1k | 标题 “A 10-Year-Old CNN Outperforms GPT-4o on Skin Cancer Diagnosis”；“The research, conducted by Stanford University's Lynch Lab, tested GPT-4o, Gemini 1.5 Pro, and Claude 3 Opus against a decade-old ResNet-50 model fine-tuned on dermatology images.” “The specialized CNN achieved a diagnostic accuracy of 81.5%, while the best-performing LLM, GPT-4o, reached only 60.3%. The study involved 1,500 histopathological images across 15 skin disease categories.” | 2026-10-26 02:27（2026-10-25T18:27:27Z） | 机构、被测模型、网络、数字、样本量全错，与原文无一相符，实为编造；链到原文 URL 却写错内容 |
| El Solitario（西），Andrés Morales | https://elsolitario.org/2026/10/25/un-cnn-de-2015-le-gana-a-gpt-6-astra/ | 标题 “CNN de 2015 vence a GPT-6 en melanoma con IA, según estudio”；正文题 “Un CNN de 2015 le gana a GPT-6 Astra y Claude en melanoma, según un experimento” | 2026-10-26 10:27（2026-10-26T02:27:17+00:00；页面写 25 octubre） | 把总体六分类优势写成“在黑色素瘤上赢”；原文黑色素瘤首选对 13/30 对 8 与 11，而分诊里 CNN 预设阈值放走 18/30；全文未提这一点 |
| AI/TLDR（英） | https://ai-tldr.dev/releases/lynch-lab-cnn-vs-llm-skin-cancer/ | “A decade-old image classifier made fewer misses than two frontier LLMs on hard, biopsy-proven skin lesions.” | 2026-10-25（页面只给日期；官方未给时刻） | “fewer misses”与表相反：黑色素瘤被放走 CNN 18/30、LLM 3 与 4；只在六分类上更准 |
| GIGAZINE（英版，日本） | https://gigazine.net/gsc_news/en/20261026-cnn-vs-llm-skin-cancer-diagnosis/ | 标题 “In a skin cancer identification competition, the latest general-purpose AI was defeated by a 10-year-old AI.” 正文：“Dr. Magnus Lynch, principal investigator at the Lynch Institute, based at King's College London” 与 “… released in 2016” | 2026-10-26 11:40（2026-10-26T12:40+09:00） | 单人博客测试写成“identification competition”（竞赛）；机构名“Lynch Institute”、2016 年均与原文不符（原文：Lynch Lab、2015 年发表） |
| DEV Community，作者 trillioniar s（英） | https://dev.to/trillioniar_s_14a3c313e14/a-10-year-old-cnn-beats-gpt-6-at-skin-cancer-and-llm-prices-fall-80-1jl2 | 标题 “A 10-Year-Old CNN Beats GPT-6 at Skin Cancer, and LLM Prices Fall 80%” | 2026-10-26 09:07（2026-10-26T01:07:57Z） | 仅标题夸大（“Beats GPT-6 at Skin Cancer”）；正文反而写到了分诊问题：“The headline hides a triage problem. At its preset threshold, the CNN discharged 18 of 30 melanomas.”，可作对照 |

其余只记不细摘：Hyper.ai（2026-10-25T18:34:50Z）、TechBytes（页面日期 2026-10-26，含“superior categorical diagnostic accuracy”类措辞，同时写出了 matched 数字）基本忠实转述，不列为夸大。均已存档 `a-hyperai.*`、`a-techbytes.*`。

## 5. 候选配图（`docs/2610/1025/images/`，均 2 倍像素比、宽 1400 px，已逐张打开目视确认）

| 文件 | 内容 | 来源与说明 |
|---|---|---|
| 01-header-fig1.png | 文章标题、作者与日期、图 1（耳部黑色素瘤，含图注与 “CC BY-NC” 许可行） | 无头 Chrome 渲染（避开浏览器翻译插件浮标），浅色页 |
| 02-test-design.png | “The test”：200 例构成 | opencli 全页截图裁切，浅色页 |
| 03-examples-fig2.png | 图 2：另外三例，含各例结果图注与许可行 | 无头 Chrome 全页渲染裁切（opencli 全页图里懒加载的一张皮肤镜图没出，已弃用） |
| 04-prompt-cnn-training.png | LLM 输入与提示、CNN 训练与“剔除测试病灶”原句 | opencli 全页截图裁切 |
| 05-fig3-diagnosis.png | 图 3：六分类首选正确数（总体与分病种，含图注） | 同上 |
| 06-diagnosis-text.png | 第二选择、分病种差距、各自的偏好答案 | 同上 |
| 07-results-table.png | “The results in one table”整张表（最关键：分诊/漏放、预设阈值、matched） | 同上，整表无删减 |
| 08-caveats.png | 作者局限段（四条） | 同上 |
| 09-patient-conclusion.png | 患者式提问一句与结论一句 | 同上 |
| 10-run-it-authorship.png | 数据包说明、作者声明由 Claude Code 起草与被测模型之一也是 Anthropic 的、MILK10k 许可行 | 同上 |
| 11-cnn-model-card.png | CNN 演示页：训练/验证/测试划分、阈值定义、AUC、200 例上的 6/30 与 14/30 | 无头 Chrome 渲染，**深色底**页面（页面自身主题），模型页 lynchlab.net/skin-ai-inception |
| 12-hn-card.png | HN 主帖卡片（标题、提交者、分数 593 与评论数 250、时刻 “19 hours ago”） | 无头 Chrome 渲染，20:52–20:53 北京；匿名页面，无抓取者信息 |

未单独截 10-11 前作的“Which lesions would each model send home?”一节（本组编号 01–12 已用满）；需要时可补一张（`a-drmagnuslynch.html` 已存档）。
