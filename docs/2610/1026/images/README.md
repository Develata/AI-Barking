# 1026 配图说明

标题：两家AI撞车？同天称推翻同一猜想

本期期号1026；新闻与取证日期为2026-10-27（北京），卡片日期写10月27日。用户确定主帖顺序：ETH（OpenAI 与 Anthropic）【跟进1006/1008】→ GPT-6.1-Terra 的 AA 评测 → 微软自愿离职。

封面二选一：小红书/抖音用 `00-cover.png`（3:4），公众号用 `00-cover-wide.png`（约2.35:1）。封面后按下表上传，单个平台总计7张。公众号也插省流卡。正文独立可读。

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | 00-tldr.png | 省流 | 三条事实，逐字摘自正文事实段 |
| 2 | 00-roundup.png | 今日速览 | 三条主帖小标题＋Anthropic 招股书两条、noyb/Scale AI、Cloudflare、404 Media |
| 3 | 40-an-lean-caveat.png | 第一条吠点① | Anthropic formal-math/eth README：Lean 检查证明，但没有机器检查命题文件是否就是论文所说 |
| 4 | 41-an-paper-asymptotic.png | 第一条吠点①② | Anthropic 论文：结果纯属渐近（m=5n 时所证上界要 n>10^10000 才优于穷举）；Lean 由 AI 写、AI 审，无已知的人检查 |
| 5 | 42-aa-pareto.png | 第二条事实段、吠点① | Artificial Analysis 文章：最高档每题0.40对1.40美元；低档、中档才在成本前沿上 |
| 6 | 44-msft-veo-scope.png | 第三条事实段 | 微软官方博客：四国参与团队、70级及以下；有些部门提供、有些不提供 |

## 备用图

| 文件 | 用途 |
|---|---|
| 43-aa-hallucination-def.png | 幻觉率定义批注（视觉复核 SHOULD_FIX：源图文字小、手机难读，移入备用；正文已讲清分母） |
| 40-an-lean-caveat-raw.png、41-an-paper-asymptotic-raw.png、42-aa-pareto-raw.png、43-aa-hallucination-def-raw.png、44-msft-veo-scope-raw.png | 批注底图 |
| 01-oa-blog-intro.png、02-oa-blog-asymmetry.png、03-oa-blog-eth.png、04-oa-blog-kakeya.png | OpenAI 博文：导语、指定 shepherd 与信息不对称、ETH 一节、局部光滑一节 |
| 05-oa-eth-abstract-thm11.png、06-oa-lean378-scope.png、07-oa-lean079-scope.png | OpenAI 预印本摘要与定理1.1；Lean 范围说明（378 证的界弱于10/25稿） |
| 08-an-readme-top.png、09-an-readme-lean.png、10-an-paper-abstract-provenance-appE.png | Anthropic README 开头与 Lean 一节；论文摘要、Provenance、附录E |
| 11-agmai-about.png、12-agmai-statements.png | AGMAI：独立、不收费；10/25、10/26 声明 |
| 13-aa-takeaways.png、14-aa-intel-index.png、15-aa-coding-agent-index.png、16-aa-omniscience.png、17-aa-terminal-bench.png、18-aa-evaluations.png、19-aa-model-page.png、24-aa-releases-by-effort.png | Artificial Analysis 文章与各图、模型页、各档位 |
| 20-oa-intro.png、21-oa-factual-error.png、22-oa-terminal-bench.png、23-oa-pricing.png | OpenAI Terra 发布页（深色底） |
| 25-msft-veo-intro.png、26-msft-veo-scope.png、27-msft-veo-full.png | 微软官方博客：开头、范围段、全文（无“60,000”与 AI 字样） |
| 28-msft-jul-header-4800.png、34-msft-jul-not-replaced-by-ai.png | 微软7月裁员信（不是本次）：“not being replaced by AI” |
| 29-cnbc-headline-keypoints.png、30-cnbc-60000-27pct.png | CNBC：6万、占22.3万的27%（匿名信源） |
| 32-verge-participating-teams.png、37-verge-headline.png | The Verge：参与/不参与部门（匿名信源） |
| 33-msft-ar26-employees.png | 微软2026年报：约22.3万全职员工 |
| 36-siliconangle-affected-workers.png | 夸大实例：SiliconANGLE 写成 affected workers / job cuts |

## 封面

封面抽签：画风=霓虹赛博；底色=满幅场景；标题=顶部横排；看板娘=探头；动作=反应

- 本期画风：霓虹赛博雨夜十字路口，两辆一模一样、车顶托着发光网状电路板的赛车迎头相撞，对应“两家 AI 同天撞上同一结论”；看板娘从自动售货机后探头捂嘴震惊。
- 生成：Codex CLI gpt-6-astra medium 图像生成（`-s workspace-write`，本机 danger-full-access 被拦），附仓库根目录 common_images 下的看板娘参考图。种子见 `barking cover-draw` 输出（默认）。
- 候选：A（东方传统年画：石拱桥两辆推车相撞，看板娘茶棚旁观）、B（霓虹赛博）。第 1 轮两组都写着旧标题“两家AI撞车推翻同一猜想？／人还没审完”，反向核验第 1 轮判该说法为 BLOCKER，作废；第 2 轮用新标题“两家AI撞车？”＋副标题“同天称推翻同一猜想”。Develata 看了第 2 轮 B 组后指出看板娘“和以前差距很大”（旧模板要求随画风重画、并把垂耳写成“狗耳”，模型画成半写实成人、竖尖耳），第 3 轮起提示词逐项写明参考图的 Q 版平涂外形。
- 选 B：竖版用第 3 轮（cover-B-3），横版用第 4 轮（cover-B-4；第 3 轮横版胸牌“吠点”二字糊掉）。霓虹大字在手机缩略图里最醒目（Develata 要求封面更吸引读者）；看板娘与参考图一致（Q 版、呆毛、深蓝垂耳、鲸鱼围裙、带白点鲸尾）。与前 3 期（1025 绘本手绘、1024 及以前浅色纸底）不是一路。A 落选：年画风新意足，但标题不如霓虹醒目，且人物未按参考图。
- 核对：竖版、横版主标题逐字「两家AI撞车？」、副标题「同天称推翻同一猜想」、胸牌「AI 吠点」（4 倍放大核对）；无其他文字、logo、真人；看板娘两只手（一只扶售货机、一只捂嘴）、两只垂耳、一条尾巴。
- 已知缺陷：横版看板娘站在右侧（x≈1390–1760），居中 1:1 裁切（x≈551–1370）只含主标题、不含看板娘；第 5 轮写明站位仍画在右侧，未采用。公众号设置方形缩略图时可手动选区，或只保留标题。
- 提示词存档：草稿目录 `cover-B-2/prompt.md`（选中）、`cover-A-2/prompt.md`（落选），不入库。
