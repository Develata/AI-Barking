$ErrorActionPreference='Stop'
$root='E:/gitclone/AI-Barking/docs/1001'
$out=[Collections.Generic.List[string]]::new()
function TextOf($name){(Get-Content "$root/sources/$name-extract.json" -Raw|ConvertFrom-Json).content}
function Q($name,$pattern){((TextOf $name) -split '\r?\n\r?\n' | Where-Object {$_ -match $pattern}) -join "`n`n"}
function Stamp($name){(Get-Item "$root/sources/$name").LastWriteTimeUtc.AddHours(8).ToString('yyyy-MM-dd HH:mm:ss')+' 北京时间（UTC+8；档案写入时刻）'}
function Esc($s){([string]$s).Replace('|','\|').Replace("`r",'').Replace("`n",'<br>')}
function Row($id,$level,$url,$quote,$shots,$condition,$status='已找到'){
 $links=($shots -split ';' | Where-Object {$_} | ForEach-Object {"[$_]('../images/$_')".Replace("'",'')}) -join '；'
 $out.Add('| '+((@($id,$level,$url,$quote,$links,$condition,$status)|ForEach-Object {Esc $_}) -join ' | ')+' |')
}
$G='https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-argon/'
$M='https://deepmind.google/models/evals-methodology/gemini-4-argon'
$AA='https://artificialanalysis.ai/articles/gemini-4-argon-google-top-three-labs'
$O='https://openai.com/index/disrupting-a-coordinated-model-distillation-campaign/'
$C='https://www.anthropic.com/research/what-work-can-robots-do'
$out.Add(@'
# 1001 期取证清单

仅记录来源、原文、口径与缺口，不是发布正文。核验基线：main / 56390318117f1c942475faa3d72e51378f3ece6b。

**时间边界：本次机器实时时钟换算后，抓取发生在北京时间 2026-10-02 凌晨；不能冒充 10-01 10:05 或 23:07 的历史快照。**每个档案的抓取/写入时间见 capture-log.md。正文页仅给 9 月 30 日而没有时区时，写“官方未给时刻”，不擅自将该日转换为北京时间某一天。社交热度仅是本次样本。

“已找到”只表示相应原文存在，不表示独立验证了供应商自报成绩或归因。L1 官方、L3 第三方自身评测/研究原始发布、L4 媒体、L5 二手站或聚合、L6 社区；L4–L6 不能替代事件事实的一手证据。论文归入 L3 原始研究材料，不等同独立复现。

网页存档是浏览器渲染后提取的 Markdown-in-JSON 及布局/元数据 JSON，不是完整网页 HTML。官方图表保留原始 GIF/PNG；PDF 保留完整文件。候选图 08、12、13、15、19、20 为原图等比缩放；09–10、28–39 为官方/研究者 PDF 原页渲染（多页图仅纵向拼接，未重绘）。截图与 PDF 的出处、页码见下文。

## A. Gemini 4 Argon

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
'@)
Row 'A1 标题、作者、日期与开放范围' 'L1' $G (Q 'a-google' 'Today, we’’re announcing|Safely releasing|We built Gemini 4|# Gemini 4 Argon|Sep 30|Koray Kavukcuoglu') '01-argon-title.png;02-argon-rollout-price.png;06-argon-rollout-soon.png' '标题 Gemini 4 Argon: our next era of frontier intelligence；作者 Koray Kavukcuoglu；Sep 30, 2026，article:published_time=2026-09-30；官方未给时刻/时区。限受信网络防御者；公开开放仍是未来计划。'
Row 'A2 入门价与脚注' 'L1' $G (Q 'a-google' 'Argon will launch|After the introductory period') '02-argon-rollout-price.png;07-argon-price-footnote.png' '美元/百万 token；缓存输入较输入价折扣95%；恢复标准价的具体日期未给。'
Row 'A3 输出上限与网络安全防护范围' 'L1' $G (Q 'a-google' 'significantly expanding|To better equip cyber defenders|CWE-bench') '04-argon-output-limit.png;05-argon-cyber.png' '64K→1M 是输出上限；without cyber guardrails 的对象是 trusted defenders 和 Google 内部团队，并非所有用户。'
Row 'A4 内存与 Rust 迁移' 'L1' $G (Q 'a-google' 'Memory efficiency|Large Scale Codebase') '03-argon-internal-work.png' '保留 once rolled out、estimated、are working、undergoing；不改成已部署成果。'
Row 'A5 官方 19 行×4列对比表' 'L1' 'https://storage.googleapis.com/gweb-uniblog-publish-prod/original_images/gemini-4-argon_table_blog.gif' 'Methodology: deepmind.google/models/evals-methodology/gemini-4-argon' '08-argon-benchmark-table.png' '全部逐行数值见下表；官方原图缩放，3936×3948→1400×1404。按显示数值比较：13项独占最高、1项并列最高、5项不是最高。缺值不补零；不同基准不合成为总分。'
Row 'A6 四项 benchmark 方法条件' 'L1' $M (Get-Content "$root/sources/a-methodology.txt" -Raw) '09-argon-methodology.png;10-argon-methodology-osworld.png' '成功取得官方 PDF；渲染页2–3。通则最高 thinking、pass@1，小基准多次平均但未给具体次数。DeepSWE：Argon 自测 mini-swe agent，竞品来自榜单/卡片；FrontierSWE：Proximal 榜单；Terminal4：Argon 自测，其余官方榜单最高得分档；这两项未列 harness/次数。OSWorld：3次取最大，每次single attempt，offline partial、1080p、500步、Gemini CUA、批量工具、compaction、08.08补丁；Astra取官方博文，Anthropic因只报混合子集而留空。不能说此四项竞品全部由Google自测。'
Row 'A7 DeepSWE / AutomationBench 单项图' 'L1' $G 'DeepSWE v1.1 — Long-horizon software engineering • Higher is better；AutomationBench — Enterprise workflow automation • Score • Higher is better；Methodology: deepmind.google/models/evals-methodology/gemini-4-argon' '19-argon-deepswe.png;20-argon-automationbench.png' '按 Argon / Astra / Fable5.1 / Opus5.5：DeepSWE 77.9 / 74.1 / 67.4 / 74.2%；AutomationBench 51.3 / 41.4 / 31.4 / 42.5%。脚注均为 Methodology 链接；AutomationBench 使用 Zapier private set / 官方榜单（方法PDF页2）。'
Row 'A8 AA 指数、成本、token、开放范围和促销' 'L3' $AA (Q 'a-aa' 'September 30|# Gemini 4|first proprietary|At its current|currently being rolled|Launch discounts|Pricing:') '11-aa-title-cost.png;12-aa-token-cost.png;14-aa-omniscience-details.png' 'AA 2026-09-30，仅日期。high vs Astra(max)/Sol(max)；53/53/52；1.99美元相对3.26约60%，相对Sol2.7倍；标准价3.98；62k/27k输出token/任务；at least one month 且结束日未确认。'
Row 'A9 AA-Omniscience 幻觉、准确率、总分' 'L3' $AA (Q 'a-aa' 'Lowest hallucination rate') '14-aa-omniscience-details.png;15-aa-omniscience-chart.png' 'Argon high：幻觉15%、准确率50%、总分42；Astra max：51%、63%、43；Sol max：54%、总分42。lowest 限 Intelligence Index ≥45 的模型；不是零幻觉。完整图保留全部面板和effort。'
Row 'A10 两处 Terminal Bench 与 AutomationBench' 'L3 / L1' "$AA ; $G" (Q 'a-aa' 'Stronger agentic performance') '13-aa-agent-benchmarks.png;08-argon-benchmark-table.png' 'AA Terminal Bench4：Argon57%、Sonnet5.5(max)64%、Opus5.5(max)60%、Astra59%；Google表：Argon57.4%、Astra58.2%、Fable5.1 57.9%、Opus5.5 66.4%。保留冲突，不归因于舍入或自行取舍；AA AutomationBench-AA78%与Google AutomationBench51.3%为不同口径。'
Row 'A11 AA 模型页即时状态' 'L3' 'https://artificialanalysis.ai/models/gemini-4-argon' (Q 'a-model' 'Not publicly available|\$2\.00|\$10\.00|Intelligence Index|Input Price') '16-aa-model.png' ((Stamp 'a-model-extract.json')+'；指数53，输入2/输出10美元每百万token；图例Not publicly available。页面FAQ模板还出现available via API through 1 provider，不能用模板句推翻官方限量开放。')
Row 'A12 编码 agent 对比页' 'L3' 'https://artificialanalysis.ai/agents/coding-agents/comparisons/antigravity-cli-vs-codex' (Q 'a-agents' '64|63|\$5\.84|\$1\.04|34\.5m|15\.5m|It is intended|It does not include|This chart shows') '17-aa-agents-table.png;18-aa-agents-charts.png' ((Stamp 'a-agents-extract.json')+'；Antigravity CLI+Argon 64 / $5.84 / 34.5m；Codex+GPT-6.1 Sol(xhigh) 63 / $1.04 / 15.5m。Coding Agent Index v1.5；API成本，不是订阅/全部运营成本；wall time不含环境启动、verifier/judge和harness开销。')
Row 'A13 Bloomberg 内部员工、否认与股价' 'L4' 'https://www.bloomberg.com/news/articles/2026-09-30/google-grapples-with-employee-skepticism-about-new-gemini-model' (Q 'a-bloomberg' 'Alphabet Inc|Google unveiled|September 30') '' '原站找到，标题 Google Grapples With Employee Skepticism About New Gemini 4；2026-10-01 03:52北京（原9/30 19:52 UTC），更新05:05（21:05 UTC）。可见导语支持存在对编码表现的内部怀疑；付费墙后员工原话、谷歌否认、盘后1.7%及回吐未核到；未绕过。' '部分支持'
Row 'A14 官方发布时刻' 'L1' 'https://x.com/GoogleDeepMind/status/2105388084154056939' '2026-09-30T20:03:30.000Z' '' '官方X主帖time datetime：2026-10-01 04:03:30北京（原9/30 20:03:30 UTC）；只是该帖发布时间，不认定为全网最早发布。Google正文仅9/30。a-official-x.json 存了时间/公开卡片；显示文字被浏览器自动翻译，不作为英文逐字引语。Google/Koray另外的最早帖未核到。'
Row 'A15 HN、Reddit、X 热度' 'L6 / L1' 'https://news.ycombinator.com/item?id=49913571 ; https://www.reddit.com/r/singularity/comments/1wuj72j/ ; https://x.com/GoogleDeepMind/status/2105388084154056939' 'Gemini 4 Argon solved hallucinations.' '' '本次HN 1579分/1047评；Reddit幻觉帖1305分/246评；相关高票1wufgo3 1538/220、1wufb0a 1349/236。X显示780.9万查看、1831回复、8871转帖、4.1万赞、6133收藏（缩写保留，非精确计数）。各自抓取时间及URL见“热度记录”。不是用户扫描时点。'
$out.Add(@'

### A5 完整逐行抄录

列与单位按官方原图，不加入不存在的 GPT-6.1 Sol 或 Claude Sonnet 5.5。最高仅在有显示数值的四列中比较。

| 行 | Benchmark / 条件 | Gemini 4 Argon | GPT-6 Astra | Claude Fable 5.1 | Claude Opus 5.5 | Argon位置 |
|---|---|---:|---:|---:|---:|---|
| 1 | Vals Index | 68.9% | 63.1% | 65.8% | 67.0% | 最高 |
| 2 | AutomationBench / Score | 51.3% | 41.4% | 31.4% | 42.5% | 最高 |
| 3 | Vals Finance Agent v2 | 65.4% | 53.5% | 58.9% | 58.6% | 最高 |
| 4 | Harvey’s Legal Agent Benchmark | 19.6% | 5.4% | 6.7% | 3.8% | 最高 |
| 5 | DeepSWE v1.1 | 77.9% | 74.1% | 67.4% | 74.2% | 最高 |
| 6 | FrontierSWE v2 | 55.0% | 65.5% | 56.3% | 62.3% | 落后 |
| 7 | Vibe Code Bench | 91.9% | 89.6% | 90.3% | 90.3% | 最高 |
| 8 | Terminal-bench 4.0 | 57.4% | 58.2% | 57.9% | 66.4% | 落后 |
| 9 | PostTrainBench | 45.3% | 44.3% | 40.2% | 49.3% | 落后 |
| 10 | Terminal-Bench Science 0.1 | 57.6% | 68.1% | 52.6% | 63.3% | 落后 |
| 11 | LABBench 2 | 88.8% | 85.4% | 68.6% | 73.1% | 最高 |
| 12 | RiemannBench | 76.0% | 72.0% | 65.6% | 69.6% | 最高 |
| 13 | GraphWalks / Up to 128k, BFS (F1) | 99.7% | 98.7% | 91.4% | 90.6% | 最高 |
| 14 | GraphWalks / 256k to 1M, BFS (F1) | 84.2% | 71.8% | 65.0% | 66.8% | 最高 |
| 15 | Agent’s Last Exam / Pass rate | 39.5% | 34.2% | — | 38.2% | 最高 |
| 16 | OSWorld-2.0 / Offline subset, Partial score | 69.2% | 72.6% | — | — | 落后 |
| 17 | Chartography | 71.6% | 71.0% | 46.2% | 66.3% | 最高 |
| 18 | LVBench | 91.7% | 87.5% | 79.7% | 83.7% | 最高 |
| 19 | CWE-bench v1 | 68.0% | 68.0% | 58.0% | 67.0% | 并列最高 |

图页脚：`Methodology: deepmind.google/models/evals-methodology/gemini-4-argon`。与“只在 DeepSWE 领先”不符；也不支持“全面碾压”。方法PDF还说CWE榜单按pass@1排序、pass@4破同分；这里只计算图中显示值。

## B. OpenAI × Moonshot

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
'@)
Row 'B1 标题、日期、分类与开篇两段' 'L1' $O (Q 'b-openai' '^# Disrupting|September 30|\[Security\]|^We recently identified|^The operators did not') '21-openai-title.png' '2026-09-30；Security；作者OpenAI；官方未给时刻/时区。完整开篇两段按原文保存。'
Row 'B2 时间线、请求/users、手法与脚注' 'L1' $O (Q 'b-openai' '^We saw operators|^The activity began|These figures describe') '22-openai-observed.png;24-openai-footnote.png' '7/1起、7/24–25共16000 requests、4000+ users、一个15000+ users集群、7/28fully disrupted；均为OpenAI陈述，未公开底层日志。users不擅改账户；attempted不改成功。'
Row 'B3 归因整段及证据/回应缺口' 'L1' $O (Q 'b-openai' '^It is unclear whether') '22-openai-observed.png' '全文检索及通读：归因只限core cluster和associated individuals；没有公开具体归因依据，也没有Moonshot回应。此页的缺失不等于其他地方从未披露。'
Row 'B4 第三方服务与后续保护' 'L1' $O (Q 'b-openai' '^We also strengthened|^This work is not finished') '23-openai-partners.png' '第三方服务协作句在How we responded；Partner-hosted句在What comes next；不据此推导所有托管端已同步修复。'
Row 'B5 Moonshot / Kimi 官方回应' 'L1 待查；L4报道' 'https://x.com/Kimi_Moonshot ; https://www.moonshot.ai/ ; https://www.moonshot.cn/ ; https://www.kimi.ai/blog/ ; https://www.cnbc.com/2026/10/01/openai-chinas-moonshot-ai-kimi.html ; https://cyberscoop.com/openai-moonshot-ai-model-distillation-attack/' ((Q 'b-cnbc' 'Moonshot did not immediately')+"`n"+(Q 'b-cyberscoop' 'reached out to Moonshot|not sharing any additional')+"`n"+(Q 'b-register' 'did not receive an immediate response')) '' '未找到本事件的官方回应。已查Kimi公开X档案（置顶7/27、可见最新9/22；未遍历完整历史）、Moonshot中英文主页、主页所链Kimi研究博客（当前列表最新日期7/16，未见本事件回应）、site:moonshot.ai/cn搜索、微博及微信公众号域名定向检索。微博搜索引擎robots限制，公众号未定位到相关官方文章；不宣称完整检索。X搜索适配器超时、搜索页空白。CNBC/Register的no immediate response只限各自截稿；CyberScoop只说已联系置评，不等于明确“未回应”。未向任何人发消息。' '未找到一手来源'
Row 'B6 三家媒体措辞与是否写成功16000次' 'L4' 'https://cyberscoop.com/openai-moonshot-ai-model-distillation-attack/ ; https://www.cnbc.com/2026/10/01/openai-chinas-moonshot-ai-kimi.html ; https://www.theregister.com/security/2026/09/30/irony-alert-openai-whines-that-chinese-model-stole-its-special-ip-that-it-stole-from-everybody-else/5300285' ((Q 'b-cyberscoop' '^# |16,000|September 30')+"`n"+(Q 'b-cnbc' '^# |16,000|Published')+"`n"+(Q 'b-register' '^# |16,000|Wed 30 Sep')) '25-cyberscoop-title.png;26-cnbc-title.png;27-register-title.png' '原文存档均成功；这三家本次可见正文未找到“16000次成功”说法。CyberScoop标题OpenAI reveals ‘‘novel’’ encryption bypass used in distillation attack，10/01 06:17:34北京（9/30 22:17:34UTC）；CNBC标题AI race heats up as OpenAI flags alleged model-copying campaign，10/01 08:04:29北京（9/30 20:04:29EDT），更新10:14:43；Register标题用stole、正文称theft但以OpenAI指控归属，页面可见Wed30Sep21:36UTC→10/01 05:36北京，搜索视图曾为22:36UTC，两视图不一致，未强行统一。'
Row 'B7 窗口前论文与9月更新' 'L3 原始研究' 'https://arxiv.org/abs/2608.09867 ; https://stolen-thoughts.com/stolen_thoughts_update.pdf' ((Q 'b-arxiv' '^# Title|^Authors:|^> Abstract|Mon, 10 Aug')+"`n"+'vendor patches routinely rely on brittle, syntactic API-level template matching and suffer days of propagation lag across secondary clouds.') '28-research-update-title.png;29-research-update-timeline.png' 'arXiv首次提交2026-08-11 01:24:50北京（8/10 17:24:50UTC）；8位作者见摘句。更新PDF首页September2026，页2–3测试状态标9/13，页3时间线又记9/27 Azure缓解、9/28原提取无法复现；页4讨论patch-lag。图片28=PDF页1，29=页3+4。不能只引标题still而忽略后续时间线。'
Row 'B8 官方发布时刻' 'L1 待查' 'https://x.com/OpenAI ; https://openai.com/index/disrupting-a-coordinated-model-distillation-campaign/' '' '' '官方页面只有September30,2026，无时区/时刻；公开X主页可见样本及定向搜索未定位到本帖；搜索页/适配器不可靠。线索北京时间10/01 02:21未验证，不能用媒体或HN时间冒充。' '未找到一手来源'
Row 'B9 HN、Reddit、Techmeme 热度' 'L6 / L5' 'https://news.ycombinator.com/item?id=49912105 ; https://www.reddit.com/r/accelerate/comments/1wue9bs/ ; https://www.techmeme.com/260930/p41' 'OpenAI says individuals associated with Moonshot AI played a significant role in a coordinated model-distillation campaign in July that peaked at 16K requests' '' 'HN本次搜索命中49912105为6分/0评、49918573为4/1，不保证全站最高；Reddit相关帖1wue9bs为71/49、1wucbvq为47/56。Techmeme该簇Bloomberg主来源+11个More链接=12个来源条目，其中1个OpenAI官方，11个媒体；不等于12家独立核验。见热度表和b-techmeme-extract.json。'
$out.Add(@'

## C. Anthropic 机器人就业暴露研究

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
'@)
Row 'C1 标题、日期与 Key findings' 'L1' $C (((TextOf 'c-anthropic') -split '## \*\*Introduction')[0]) '30-robots-title-findings.png' 'What work can robots do?；Sep30,2026；官方未给时刻/时区。Key findings实际是1条指数介绍+4条实质发现，共5个项目，已完整保留；截图为PDF页1–2。'
Row 'C2 Figure3 与分母' 'L1' $C (Q 'c-anthropic' '^Figure 3 summarizes|All job tasks are included') '34-robots-figure3.png' '官方PDF页8–9，含图、图注、前后段。54%认知人际/46%物理；全部工作时间中E0 12/E1 23/E2 10/E3 1%；物理时间74%对应全部工作34%，不是74%岗位。'
Row 'C3 Figure2 与物理任务分档' 'L1' $C (Q 'c-anthropic' 'Levels of robot exposure|^Tasks that robots can’’t|^Robots can do half|^Another 22%') '33-robots-figure2.png' '官方PDF页7。7594物理任务，按时间与就业加权；E0约1/4、E1一半、E2 22%、E3 2%；图中精确值26.3/49.8/22.0/1.9%；分母不是全部任务时间。'
Row 'C4 E0–E3 定义与多数规则' 'L1' $C (Q 'c-anthropic' 'E0: Robot cannot|^This rubric requires') '31-robots-rubric.png;32-robots-method.png' '官方PDF页4–6。majority为至少一半示例、按时间加权、能完成的最不结构化环境。'
Row 'C5 0.3%、40年、70%降价与3%条件' 'L1' $C (Q 'c-anthropic' '^Though robots can theoretically|^For robots to be cost|^Figure 8 summarizes') '38-robots-cost-decline.png;37-robots-figure7.png' '官方PDF页18–20。40年是假设成本按历史约3%/年下降、当前任务/工资等条件下的成本竞争情景；不是就业替代预测；保留humanoids/new manufacturing限定。'
Row 'C6 Figure7、打包工与robotaxi' 'L1' $C (Q 'c-anthropic' '^Figure 7 shows|Employment and total compensation|^The US currently employs') '36-robots-cost-example.png;37-robots-figure7.png' 'PDF页17–18。整套购置安装超200万美元、约14人年、寿命约10年/资本成本8%、每替代一人年约45000美元；人工约49000，机器人可做97%时间的任务，作者称约省2500/年。表中机器人年成本45.4千（固定22.2+变动23.2）；正文为近似数，不能把49−45直接当节省额。robotaxi原文是比司机贵约7000美元，不是总成本7000；these cost estimates are approximate。'
Row 'C7 Figure8 假设降价曲线' 'L1' $C (Q 'c-anthropic' 'Calculated by comparing robot') '38-robots-cost-decline.png' '官方PDF页19（拼接页20保留限定）；完整坐标、图例、图注；成本是统一假设下降，任务按时间和就业加权，6个工资分布点对比。'
Row 'C8 Claude 评分/搜索/估算与局限' 'L1' $C (Q 'c-anthropic' '^Our analyses use|^To determine exposure|^This rubric requires|^Figure 2 gives|^For every exposed task|^Estimates cover|^One caveat|Precise definitions of a robot vary') '31-robots-rubric.png;32-robots-method.png;39-robots-footnote.png' 'PDF页4–6、16、22；官网脚注10：rely on Claude判断哪些机器属于机器人。独立Appendix A–F另存c-appendix.pdf/txt，A.1评分、A.2任务分类验证、A.3暴露、A.4替代口径、D成本、E情景、F提示词；不是逐个真实机器人现场测试。similar if we omit demonstrations只说明该稳健性检查，不能消除全部估算误差。'
Row 'C9 另一处70%——能力限制' 'L1' $C (Q 'c-anthropic' '^We find that capabilities') '35-robots-capabilities.png' '官方PDF页15；分母为physical tasks，指标为能力阻碍采用；与成本下降70%不同。'
Row 'C10 0–5.5% 的另一篇 robotics 研究' 'L1' 'https://www.anthropic.com/research/claude-plays-robotics' (Q 'c-robotics' 'With low-level manipulation|^We evaluate models through|LIBERO direct-manipulation success') '' '已找到官方原文，Jul9,2026（仅日期、未给时刻）；Shmuel Berman、Michael Ilie、Jia Deng、Daniel Freeman。0–5.5%是MuJoCo / LIBERO低层直接操控完整任务成功率，不是9/30就业暴露研究中的机器人任务占比。'
Row 'C11 Yahoo Finance 跟进与逐字对照' 'L4' 'https://finance.yahoo.com/technology/article/anthropic-study-suggests-blue-collar-workers-have-decades-before-robots-take-their-jobs-115957939.html' (Q 'c-yahoo' '^# Anthropic|Thu, October|found that robots|Robot prices have fallen') '' 'Michael B. Kelley；2026-10-01 19:59北京（原10/1 7:59 AM EDT）；不是9/30同日。正文正确区分three-quarters of physical tasks、34%全部工时、0.3%work，且40年保留If that pace holds；标题decades是更宽泛概括，不能说正文忽略0.3%。'
Row 'C12 官方发布时刻' 'L1 待查' 'https://x.com/AnthropicAI ; https://www.anthropic.com/research/what-work-can-robots-do' '' '' '官网只有Sep30,2026，无时区/时刻；官方X主页样本未见该帖，定向X搜索页两次返回空数组。线索北京时间10/01 00:09未核到，不擅自填入。' '未找到一手来源'
Row 'C13 HN、Reddit、X、LinkedIn 热度' 'L6 待补' 'https://www.reddit.com/r/artificial/comments/1wur95r/ ; https://hn.algolia.com/api/v1/search?query=what-work-can-robots-do&restrictSearchableAttributes=url&tags=story ; https://x.com/AnthropicAI' 'Anthropic''s robot study separates task capability from cost. Which assumptions need the closest scrutiny?' '' 'Reddit相关原研究帖本次5分/3评，Yahoo跟进帖1wuzt7k为1/2。HN URL搜索nbHits=0（不是证明全站无帖）；X未定位官方本帖，无互动数；LinkedIn公开网页定向搜索未定位相关帖，未登录。各文件与抓取时间见热度表。' '部分支持'
$out.Add(@'

## 可能的吠点

- A1/A8：announce/rolling out限量测试不能改成全面开放，官方还写将首先面向paid API customers和Google AI Ultra subscribers。
- A2/A8：2/10是入门价；常规定价4/20；AA称至少一个月，但Google正文没有给促销截止日。
- A4：300 TiB带once rolled out，500 TiB–1 PiB带estimated；Zircon改写仍在审计测试，不能写成已生产落地。
- A5/A6：“只在DeepSWE领先”和“全面碾压”均与显示表格不符，且OSWorld有3次取最大、offline partial等条件。
- A9：15%幻觉率的最低范围是指数45+模型；准确率50%低于Astra63%，不能写解决幻觉。
- A10：Google的Opus5.5 Terminal4为66.4%，AA为60%；两者都保留，未找到足够条件来判定哪一个“真”。
- A12：编码agent指数64/63不等于裸模型所有任务强弱，成本和耗时另有明确排除项。
- B2/B3：16000是尝试请求，15000是users组成的一个集群；核心簇归因给关联个人，不是所有操作者都属于Moonshot。
- B1/B4：报告排除了破解加密、侵入数据库、直接访问储存用户对话；第三方部署保护工作尚未完成。
- B5：没找到官方回应只描述检索结果；媒体“未立即回应”不能写成永久沉默或默认承认。
- B7：9月更新PDF的9/13测试表与9/27–28修复时间线并存，不能忽略后者将旧漏洞状态写成抓取时的现状。
- C2/C3：74%是物理工作时间暴露，折合全部34%；暴露等级含高度受控的环境，并不等于岗位替代。
- C5/C7：40年对应特定成本下降情景，作者明确保留新型机器人/制造流程改变路径的可能性。
- C6：打包工是整个机器人组合摊销后的每人年估算，robotaxi的7000美元是差额；不能把购置总价、单台示例价、年成本混用。
- C8：暴露、时间权重和成本估算均依赖Claude；附录有复核和稳健性分析，不等于全部实地测量。
- C9：能力阻碍约70%的物理任务与成本需降约70%是两个不同的量。
- C10/C11：0–5.5%是7月控制实验；Yahoo原文实际标10/1，不能均归为9/30同日发布。

## 夸大说法实例

以下仅确认说法实际存在并对照原文，不把转述者的说法当事件事实；完整站点原文分别归档。标题的偏差与正文是否保留限定分开记录。

| 编号 | 发布方 / 原站URL | 原句（逐字） | 发布时间（北京/原时区） | 对照与范围 | 档案 / 状态 |
|---|---|---|---|---|---|
| D-A1 | Reddit r/singularity / https://www.reddit.com/r/singularity/comments/1wuj72j/ | Gemini 4 Argon solved hallucinations. | 见下面Reddit created_utc换算表 | AA仍有15%幻觉，且限特定基准和模型门槛；帖子正文又用了may，不能忽略这层保留 | d-reddit-a.json / 已找到 |
| D-A2 | 鼓狮 / https://www.gushiio.com/news/29567.html | 谷歌Gemini 4遭内部吐槽：虽碾压OpenAI，但编程实战效果差 | 页面提取未可靠取得独立发布时间；只见正文“10月1日”，原时区未给，未据中文站擅自换算 | 标题“碾压”宽于A5/A8；正文确实写了只向小批伙伴开放，不能称其全文宣称全面开放 | d-a-chinese-extract.json / 已找到 |
| D-B1 | The Register / https://www.theregister.com/security/2026/09/30/irony-alert-openai-whines-that-chinese-model-stole-its-special-ip-that-it-stole-from-everybody-else/5300285 | Irony alert: OpenAI whines that Chinese model stole its special IP that it stole from everybody else | 10/01 05:36北京（页面9/30 21:36UTC）；另一搜索视图22:36UTC，保留差异 | “stole”比官方attempted/core cluster更肯定；仍有OpenAI归属，不能称它无归属地独立证明Moonshot盗窃；未见16000次成功 | b-register-extract.json / 已找到 |
| D-B2 | ic.work / https://www.ic.work/article/openai-disrupts-16000-reasoning-distillation-attacks-citing | 涉及超过 4,000 个违规账户与 15,000 个关联集群 | 2026-10-01，仅日期、时区未给；作者Ada Vector | 官方为一个more than15000 users集群，不是15000个集群；该文后面明确说尝试，不能反过来把它当“16000成功”的实例 | d-b-chinese-extract.json / 已找到 |
| D-C1 | AI Job Risk / https://www.willaitakemyjob.app/blog/will-robots-take-my-job | Probably not soon. Anthropic's new robotics study, published on 30 September 2026, found that robots can already do 74 percent of physical work tasks in the US, but are cheaper than a person for only 0.3 percent of them. | 1 October2026，仅日期、时区未给 | “of them”将0.3%的分母接成physical work tasks；官方0.3%为全部工作时间，此处存在分母混淆；本文保留historical rate条件，不当作无条件40年预测的实例 | d-c-english-extract.json / 已找到 |

C组中文指定类型误读：本次搜索未找到可核对的原站实例，不凑数。Yahoo标题可作“概括变宽”的候选，但正文保留34%、0.3%及条件，因此不列为已确认的数字误读。A组“全面开放”、B组“16000次成功”“黑进数据库”的实际原站实例也未找到；现有实例不能替代这些具体指控。

## 热度记录

所有热度是抓取瞬间值，score可能有模糊/动态变化。Reddit“高票”只限本次query的相关命中，不声称全站最高；与主题无关的高分结果未纳入。时间来自本次档案写入时间，不能补写历史热度。

| 平台/组 | 原站URL | 标题/标识 | 数字 | 发帖北京时间（括号原UTC） | 抓取北京时间 |
|---|---|---|---|---|---|
'@)
foreach($pair in @(@('A','d-reddit-a.json'),@('B','d-reddit-b-targeted.json'),@('C','d-reddit-c-targeted.json'))){
 $posts=Get-Content "$root/sources/$($pair[1])" -Raw|ConvertFrom-Json
 foreach($p in $posts){if($p.id -in @('1wufgo3','1wufb0a','1wuj72j','1wue9bs','1wucbvq','1wur95r','1wuzt7k')){
  $utc=[DateTimeOffset]::FromUnixTimeSeconds([long]$p.created_utc)
  $out.Add('| Reddit '+$pair[0]+' | '+$p.url+' | '+(Esc $p.title)+' | '+$p.score+'分 / '+$p.comments+'评 | '+$utc.ToOffset([TimeSpan]::FromHours(8)).ToString('yyyy-MM-dd HH:mm:ss')+'（'+$utc.ToString('yyyy-MM-dd HH:mm:ss')+' UTC） | '+(Stamp $pair[1])+' |')
 }}
}
foreach($pair in @(@('A','d-hn-a.json'),@('B','d-hn-b.json'))){
 $data=Get-Content "$root/sources/$($pair[1])" -Raw|ConvertFrom-Json
 foreach($p in $data.hits){if($p.objectID -in @('49913571','49914236','49912105','49918573')){
  $utc=[DateTimeOffset]::new([DateTime]::SpecifyKind([datetime]$p.created_at,[DateTimeKind]::Utc))
  $out.Add('| HN '+$pair[0]+' | https://news.ycombinator.com/item?id='+$p.objectID+' | '+(Esc $p.title)+' | '+$p.points+'分 / '+$p.num_comments+'评 | '+$utc.ToOffset([TimeSpan]::FromHours(8)).ToString('yyyy-MM-dd HH:mm:ss')+'（'+$utc.ToString('yyyy-MM-dd HH:mm:ss')+' UTC） | '+(Stamp $pair[1])+' |')
 }}
}
$out.Add('| X A | https://x.com/GoogleDeepMind/status/2105388084154056939 | 官方发布帖 | 780.9万查看 / 1831回复 / 8871转帖 / 4.1万赞 / 6133收藏 | 2026-10-01 04:03:30（2026-09-30 20:03:30 UTC） | '+(Stamp 'a-official-x.json')+' |')
$out.Add('| Techmeme B | https://www.techmeme.com/260930/p41 | Moonshot distillation簇 | 主来源1 + More11 =12条目（含官方1、媒体11） | 聚合快照不是报告发布时刻 | '+(Stamp 'b-techmeme-extract.json')+' |')
$out.Add(@'

## 检索边界与失败

- 浏览器成功读取三家官方正文、AA三页、三家B媒体、Bloomberg可见导语、arXiv、研究者更新PDF、Anthropic正文/附录PDF和Yahoo原站。
- Bloomberg付费墙后内容未取；普通shell联网脚本/curl/Invoke-WebRequest被自动审批阻止，未申请不允许的提权；改用获准的原站浏览器读取与无凭据资源下载。
- X搜索适配器超时，浏览器X搜索空页；官方主页只覆盖可见样本，未当作完整历史。微博搜索引擎robots限制，公众号未定位官方回应文章。LinkedIn无相关公开命中，未登录。
- Yahoo首次state发生TypeError，但随后正文extract和元数据成功，不把整页记为失败。Methodology PDF的网页extract为空，但原PDF下载、文本提取和渲染成功。
- 早期长网页截图有重排/懒加载错误：C组错误截图移至sources/d-rejected-*作为诊断，不是候选配图；有效候选替换为PDF渲染。图片以最终清单为准。
- 未访问付费墙后正文，未用镜像/代理/转载来补Bloomberg受限原文，未自行登录，未进行社交写操作。
'@)
$out -join "`n" | Set-Content "$root/sources/evidence.md"






