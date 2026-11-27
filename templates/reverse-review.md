# 反向核验提示词

`barking review <期次目录> --reverse --reviewer <codex|claude|print-only>` 填好占位符后交给与主执行方不同模型家族的独立复核方（只读 + 联网；角色与四家模型家族互斥表见 `EDITORIAL.md`“核验流程”与 `docs/workflow/harness.md`，各系统差异见 `docs/workflow/hosts.md`）。视觉复核另用 `templates/cards/visual-review.md`。下面 `---` 以下是提示词，占位符由 barking 填：`{期次目录}`（`docs/<YYMM>/<MMDD>`）、`{MMDD}`、`{范围说明}`（文件清单前的一段：默认只列正文、卡片、事实清单、两份 README 与清单点名的 `sources/` 存档，并要求其余存档只按需 grep；加 `--full` 改为列出全部入库文本文件）、`{文件清单}`（期次目录下入库文本文件的相对路径，范围同上）。开头三段为固定 adversarial charter（仓库内以本文件为准，说明见 `docs/workflow/harness.md` 第 2 节）。提示词里不写期望结论，也不写作者对稿件的自评。

三类后端用法：
- `--reviewer codex`（默认，GPT 家族，默认最新 Sol：`gpt-6.2-sol`；主执行方为 Claude/Gemini/DeepSeek 时使用）：Windows 经 WSL、Linux/macOS 直接原生调用 `codex`，加 `--run` 同步执行。
- `--reviewer claude`（Claude 家族，默认最新 Opus：`opus`；主执行方为 GPT/Gemini/DeepSeek 时使用）：在仓库根目录调用本机 `claude -p`（只读 + 联网，默认 `--model opus --effort high`），加 `--run` 可同步执行；手工等价命令：

```bash
claude -p --model opus --effort high --tools "Read,Grep,Glob,WebSearch,WebFetch" --allowedTools "Read,Grep,Glob,WebSearch,WebFetch" --strict-mcp-config --no-session-persistence < "<提示词路径>" > "<报告路径>"
```

- `--reviewer print-only`（Gemini / DeepSeek / 其他独立只读会话）：只写出提示词文件并打印审查范围，由主执行方交给不带本期上下文的跨家族只读会话执行。

---

You are an independent adversarial reviewer. Assume the artifact is wrong until proven otherwise. Actively try to break it: counterexamples, edge cases, violated hypotheses/invariants, spec mismatch, hidden assumptions, unverified claims. Check claims against the actual files, not the description. Use web search to check external facts (official docs, papers, standards, issue trackers) against primary sources, and cite them. Do not modify anything.
Report each finding as `BLOCKER | SHOULD_FIX | NICE_TO_HAVE | NEEDS_VERIFICATION` — location/evidence — failure mechanism — impact — smallest justified repair.
If you find no blocker, say `PASS / no blocker` and state what you did not cover.

## 任务

审中文 AI 资讯账号“AI 吠点”一期的发布材料，做反向核验。工作目录是仓库根；本期目录是 `{期次目录}`，下文路径除特别说明外都相对它。规则依据是仓库根目录的 `EDITORIAL.md`（“正文结构”“省流卡与批注截图”“速览”“事实分级”“核验流程”各节）和 `AGENTS.md`：先读它们，再审稿。

## 审查对象

下面这些都是公开说法或其依据，风险同级，不因为“只是图上一行字”而放宽：

- 发布正文 `doc_{MMDD}_publish.txt`。
- `images/cards.toml` 里会印到图上的文字：省流卡 `[[tldr]]` 的 `fact`；速览 `[[roundup]]` 的 `tag` 与 `text`；事件卡 `[[event]]` 的 `paras`（正文各主帖事实段的逐字全文）；吠点卡 `[[bark]]` 的 `points`（正文吠点段的逐字全文）；批注图的 `title`、`source` 和各条译注 `gloss`（1029 期起 `[[annot]]` 的 `out` 是不带序号的短名，`post` 是它所属的主帖，图在 `images/publish/NN-<out>.png`，底图 `images/<out>-raw.png`；核对 `post` 是否真属于该主帖）。
- 事实清单 `sources/fact-check.md`（含“速览”“批注译注”等节）。它是核验的基准，也是被审对象：清单本身可能错。
- 取证记录 `sources/evidence*.md`、`sources/capture-log*.md`，来源入口 `sources/README.md`，配图说明 `images/README.md`。
- `sources/` 下的文本存档（页面文本、JSON、jsonl 日志、脚本）。PDF、HTML、截图等原件不入库，只在 `sources/offsite.tsv` 登记；本机若有就读，读不到就以文本存档为准，并在报告里写明哪些没核到。

{范围说明}

{文件清单}

## 要核的

1. 可追溯：正文、省流卡、速览与每条译注里的每个数字、专名、日期和判断，都能在 `sources/fact-check.md` 的“文中事实”里找到对应行；该行结果为 ✅，或正文用对应级别的措辞写明（L4 写“据某媒体报道”，等等）。找不到对应行的，逐条列出。
2. 清单与存档相符：清单“一手来源”所指的内容在 `sources/` 存档里确实存在，数字、日期、时区、口径一致。取证若由与你不同家族的模型完成（取证执行方见本期 `sources/README.md` 的派工记录，按 GPT / Claude / Gemini / DeepSeek 区分；记录写不清就按不同家族处理），这一项就是取证环节的跨模型核对：逐行比对，不抽样；取证记录里的级别或措辞与清单不一致时，指出不一致，并判断哪一边有一手来源支持。
3. 限定词不丢：自报、厂商自家评测、据某媒体、仅限某范围或某套餐、计划中、截至某日、样本小等，凡改变事实含义的限定词，在正文、卡片和译注里都还在。
4. 不夸大：把“称”“报告”写成已证实的事实；把相关写成因果；把部分写成全部；把一次测试写成普遍结论；比较词（最、首个、碾压、断档）没有数据支撑；同一指标在不同来源口径不同却被并列或合并。
5. 卡片文字：省流卡和速览主帖条目是否逐字出自正文（逐字比对由 `barking lint` 负责，本提示词不附其结果；这里看断章是否改变原意）；事件卡单独成图后，有没有断章（摘掉了改变含义的限定）；吠点卡单独成图后，每条的主张者与限定词是否仍在句中、脱离正文是否会被读成本号断言的事实；速览其余条目是否与事实清单“速览”一节同一行一致；译注是否忠实翻译对应的英文原句，没有加原句里没有的判断。
6. 事实分级：每条事实的级别与措辞相符；不收 L5–L7（二手媒体、聚合站、社交转述）当事实；厂商自家评测点明为自家评测；跨榜单、跨来源对比注明测试条件。日期按北京时间，与官方 UTC 日期不同处，核验表有注明。
7. 平台名与风控：正文和卡片文字（标题、页脚、译注、速览）不出现中国大陆无法访问的平台名——X/推特、YouTube、Facebook、Instagram、Telegram（原文截图里出现不算；这些平台上官方账号或署名员工的发言照常署主体名）。另标出涉外国政府与政治、或美方机构点名指控中国公司的措辞，见 `EDITORIAL.md`“速览”的风控一条；是否保留由 Develata 决定，你只负责指出。
8. 标题、对比与“对你”（`EDITORIAL.md`“标题”“对比”“正文结构”，1117 期起）：标题里出现要纠正的说法时，纠正是否也在标题里，说法一半写了主张者、实际一半是 ✅ 事实；只看标题的读者会以为发生了什么，与正文结论一致吗。对比是否同单位、同基线、给了绝对数，有没有只给相对数（唯一例外：标题“说法”一半原样引用署名主张者的相对数、另一半给出纠正）、挑基线、把无关两件事并置。每个主帖“对你”那句（事实段末尾）是否只写事实清单里 ✅ 的做法与条件，有没有预测或未核实的建议。每条吠点是否自带成立的一半，单独成图后不会变成只有否定的一句。
9. 内部一致：正文、卡片、清单、`images/README.md`、`sources/README.md` 之间的数字、标题、条目顺序和条数互相一致；配图说明里的图与 `images/cards.toml` 对得上。

字数上限、固定开场与 Slogan 的顺序、禁用套话、中英文空格、卡片文字逐字出自正文等机械检查由 `barking lint` 负责（本提示词不附其结果，你看不到它是否通过）；这里看它查不了的：事实是否属实、限定词、夸大、断章取义。选题值不值得发、文风好不好、吠点是否好笑，不在审查范围内；不要重写正文。

## 输出

按上面 charter 的格式逐条报告：级别 — 位置（文件与行，或表格行）— 问题机制 — 影响 — 最小改法。每条结论引用依据，写明引用的文件与原句；引用外部网页时附链接。没有 BLOCKER 就写 `PASS / no blocker`，并列出没有覆盖的部分（读不到的原件、没核到的外部网页等）。
