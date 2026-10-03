# 1002 事实核验

标题（定稿）：“谷歌TPU上太空？OpenAI通报超百家”——“TPU上太空”依据谷歌博文 “launched into orbit … Our team has confirmed contact with the satellite”，带问号，正文吠点写明谷歌称“未来数周”将收集在轨数据；“通报超百家”依据 OpenAI 9/30 更新 “notified over 100 organizations”，正文吠点写明通知不等于入侵。

正文：`../doc_1002_publish.txt`。取证：`evidence.md`（A–C，Codex gpt-6-astra，派工单 `.handoff/2026-10-02-1002-evidence.md`）。Claude 在派工前后分别用 Firecrawl 与 curl 独立读取过 Google 两页、OpenAI 时间线页、加州司法部新闻稿、AISI 博客、Epoch、Cloudflare 博文与演示站，并目视抽查截图 09、11、18、26、27、06，与存档一致。选题线索来自四份回贴扫描（北京时间 10/2 10:58、12:01、16:58、20:50），其中的错误说法（在轨运行 Gemini、路透与 OpenAI 数字冲突、“美国首次执法”挂加州、Jev 指数是 Cloudflare 自家基准）均已对原页核正，未采用。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 省流与一：谷歌 Suncatcher 原型卫星搭 SpaceX 火箭入轨 | L1 | ✅ | Google 博文（`a-` 存档；`../images/30-suncatcher-google.png`）；SpaceX 任务页 | “launched into orbit aboard the Transporter-18 rideshare mission with SpaceX”；SpaceX：Falcon 9 于 10/1 11:32 PT 发射（= UTC 18:32 = 北京 10/2 02:32）。正文“北京时间10月2日凌晨”取发射时刻；谷歌博文页面标 Oct 01, 2026；`published_time` 2026-10-01T23:30:00+00:00 = 北京 10/2 07:30（仅见于 Claude 用 Firecrawl 读到的页面元数据，摘录存 `a-claude-google-meta.txt`；Codex 的浏览器存档只有日期 2026-10-01。**该精确时刻标为待核**，正文不依赖它）。“入轨”按谷歌原话 launched into orbit。 |
| 谷歌称已取得联系、运行符合预期 | L1 | ✅ | Google 博文 | “Our team has confirmed contact with the satellite and it is operating as expected.” |
| 谷歌称“未来数周”将收集 TPU 的在轨数据 | L1 | ✅ | Google 博文 | “Over the coming weeks, we'll gather in-orbit data on how our TPUs handle the physical stress of spaceflight and the radiation and thermal extremes of space.” 引号内“未来数周”译 Over the coming weeks。反向核验（视觉复核）指出旧稿“才开始收集”暗示尚未开始，超出原文，已改为“称……将收集”。 |
| 据 NPR 报道，星上4颗 TPU，受散热限制，将每次运行约15分钟 Gemma 模型 | L4 | ⚠️ | NPR（`a-` 存档；`../images/06-suncatcher-npr.png`，备用图） | “The prototype satellite has four specialty chips in it called Tensor Processing Units … The chips will run a version of the company's Gemma AI model for 15 minutes at a time due to heat management constraints, using the open weight model to answer simple queries.” NPR 用将来时 will run，未注明消息来源（发言人或论文），正文按 L4 写“据 NPR 报道”；“受散热限制”取自 due to heat management constraints（反向核验指出旧稿漏了这个原因）。 |
| 耐受五年任务辐射总剂量的结果来自地面测试 | L1 | ✅ | Google 9/24 说明页（`../images/31-suncatcher-ground-test.png`）；Joule 摘要 | “our team tested TPUs in a proton beam facility at UC Davis’s Crocker Nuclear Laboratory … Initial results have shown that our Trillium TPUs hold up remarkably well, and can survive a radiation total ionizing dose greater than what they would receive during a five-year space mission.” 这是地面质子束测试，不是在轨结果；指标限于总电离剂量（“辐射总剂量”为通俗译法），位翻转等单粒子效应另有讨论（Joule 摘要：“characterized for bit-flip errors”），不能扩大为五年全面耐辐射。旧稿“扛得住五年辐射”经反向核验指出丢了指标，已改。Joule 摘要同写 “survive a total ionizing dose equivalent to a 5 year mission life without permanent failures”。 |
| 已披露的散热验证是热真空舱测试 | L1 | ✅ | Google 9/24 说明页（`../images/03-suncatcher-cooling.png`，备用图） | “So far, our team has tested the technology in a thermal vacuum chamber that simulates both the thermal and vacuum environment in space. We’ll see how our new TPU cooling system works in space”。页面只说“目前（So far）测过热真空舱”，没有说这是唯一测试；旧稿“只在热真空舱测过”把未披露当成不存在，经反向核验（BLOCKER）已删“只”。在轨散热表现谷歌称待观察（We’ll see）。 |
| 4颗、15分钟、Gemma 出自 NPR，谷歌两篇博文没写 | L4 | ⚠️ | NPR；Google 10/1 博文、9/24 说明页（均已读） | 已查 Google 10/1 博文、9/24 说明页，均无 TPU 数量、Gemma、15 分钟；Joule 摘要与 Planet、SpaceX 页也无。正文限定为“谷歌两篇博文没写”，不宣称检索穷尽。 |
| 另有转述写成 Gemini | L5 | ✅ | Cocoloop 中英文页（同站，`d-` 存档） | 中文：“每次跑大约 15 分钟 Gemini 推理就停机”；英文：“Each burst lasts around 15 minutes of Gemini inference”。此行只核“该站确实这样写”，不核 Gemma 与 Gemini 孰是；与 NPR 的 Gemma 冲突。 |
| 论文称送往近地轨道的发射价“可能”在2030年代中期降到每公斤200美元以下 | L1 | ✅ | Joule 论文摘要（`../images/07-suncatcher-joule.png`，备用图） | “a learning curve analysis suggests launch to low-Earth orbit (LEO) may reach ≤$200/kg by mid-2030s.” 引号内“可能”译 may；“近地轨道”译 low-Earth orbit（LEO，反向核验指出发射价依目的轨道而变，旧稿漏了范围）；论文采用的现价参照为 $3,600/kg（Falcon 9 reusable），线索里的 $2,700/kg 未取。PDF 返回 403，页码未取得，按 HTML 章节定位。 |
| 81星编队只是示例 | L1 | ✅ | Joule 论文摘要 | “We illustrate the basic approach to formation flight via an 81-satellite cluster of 1 km radius”；illustrate 即示例，不是此次发射规模。 |
| 省流与二：OpenAI 称截至9月26日已通知超100家机构 | L1 | ✅ | OpenAI 时间线页 9/30 条目（`b-` 存档；`../images/32-openai-100-orgs.png`） | “As of September 26, our teams have notified over 100 organizations about activity that met our notification criteria.” 9/30 条目官方只给日期，未给时刻。 |
| 此前页面写“数十家” | L1 | ✅ | 同页顶部 “Activity affecting third parties” 段；本号 0926 存档 `docs/2609/0926/sources/a-openai-third-parties-en.md` | “Based on our review to date, we have notified dozens of third parties using the criteria above.” 该句当前仍在页面顶部，9/30 条目另写 over 100；正文“此前页面写”指 0926 存档时点的口径，是 OpenAI 自己的更新，不是媒体与官方冲突。 |
| 审查约50PB记录，动用约7000块 GB200/GB300，日耗超50万美元 | L1 | ✅ | OpenAI 时间线页 9/30 条目（`../images/08-openai-review-start.png`、`../images/32-openai-100-orgs.png`） | “searching through a large volume of data covering approximately 50 petabytes”；“dedicating about 7,000 GB200 and GB300 GPUs to this effort, at a cost of over half a million dollars a day”。均为 OpenAI 自述；50PB 是待审查记录量，不是泄露量。 |
| 10月1日，加州司法部称已向 OpenAI 送达调查传票 | L1 | ✅ | 加州司法部新闻稿（`b-` 存档；`../images/33-california-subpoena.png`） | 页面日期 Thursday, October 1, 2026；“California Attorney General Rob Bonta yesterday served an investigative subpoena on OpenAI”，即 9/30 送达；元数据 published 2026-09-30T22:49:52-07:00 = 北京 10/1 13:49。正文“10月1日”取官宣日。 |
| 吠点①：通知不等于入侵 | L1 | ✅ | OpenAI 时间线页 9/30 条目 | “Notification does not mean that any private information was accessed, or that there was a compromise of any third-party system.” 通知标准较宽（“We err on the side of notification”）。 |
| 已不入正文：OpenAI 称尚未发现规模或严重度可比 Hugging Face 事件的另一起入侵 | L1 | ✅ | 同上 | “So far, we have not identified another compromise of third-party systems involving our models that is comparable in scale or severity to the Hugging Face incident.” 同条目写 “The review remains ongoing, and we expect to identify more cases”；9/25 条目写 “will take months to complete”。两句因篇幅均未入正文。 |
| 吠点②：传票是加州既有调查的一步，官方称“进行中” | L1 | ✅ | 加州司法部新闻稿 | “as part of the California Department of Justice’s (DOJ) ongoing investigation of incidents resulting from the operations of OpenAI and its artificial intelligence (AI) models … The subpoena is part of a broader inquiry into cybersecurity incidents and risks involving the company and its models.” “进行中”译 ongoing；9 月已官宣对 Hugging Face 事件的正式调查（新闻稿链出 Politico 2026/09/04，未找到司法部 9 月独立稿）。 |
| 不是起诉或违法认定 | 编辑判断 | ✅ | 加州司法部新闻稿；Reuters/Guardian | 新闻稿称 subpoena 属 investigation；Bonta：“my office is committed to determining if that is the case here”，即尚未认定；Reuters/Guardian 写 “starting an investigation”（Reuters 原站现标题与导语已改，见 `evidence.md` 版本对照）。这是对“调查传票”性质的编辑解释，不是官方话。 |
| 省流与三：Cloudflare 发布决策模型 Clef 和 Clef-flash | L1 | ✅ | Cloudflare 博文（`c-` 存档；`../images/16-clef-title.png`、`../images/17-clef-release.png`，备用图） | 博文标题 “Introducing Clef: our open-source decision models”；“We’re fully open-sourcing these models on Hugging Face under an Apache 2.0 license”。博文北京 10/1 23:34（2026-10-01T15:34:02Z），日期与正文“10月1日”一致。“开源”是 Cloudflare 用词，正文吠点③写“Cloudflare 称‘开源’，实际放出权重与推理代码”；反向核验指出旧稿事实句与省流卡单独使用“开源决策模型”丢了范围限定，已改为“发布”。 |
| 只输出结构化答案与概率 | L1 | ✅ | Hugging Face 模型卡 Cloudflare/clef（`../images/28-clef-model-card.png`，备用图） | “turns a state and a schema of typed questions into decisions. There is no free-form text generation and no output parsing.”；博文称输出为 calibrated probabilities 的结构化答案。 |
| 官方称在 Jev Decision Index 上领先 | L1 | ✅ | Cloudflare 博文 | “Clef is currently the leader when evaluated against the Jev Decision Index”；“Clef”指完整版，不含 Clef-flash。 |
| 中位延迟 Clef-flash 38.8毫秒、Clef 209.3毫秒、Jev 524.1毫秒 | L1 | ✅ | Cloudflare 博文延迟表（`../images/19-clef-latency-table.png`，备用图） | Median latency · ms：Clef 209.3、Clef-flash 38.8、Jev 524.1（列序 Clef / Clef-flash / Jev / DiffusionGemma Jev / Kev 9B / Laya，表头已核）；p95：238.6 / 122.4 / 536.0。 |
| 吠点①：39毫秒属于小号 Clef-flash | L1 | ✅ | 同上；模型卡 | 38.8 四舍五入为39；“小号”：Clef-flash 基座 Qwen3.5-9B，Clef 为 Qwen3.8-27B（模型卡）；完整 Clef 中位延迟 209.3 毫秒。 |
| Cloudflare 自家榜第一是 Clef（61.2），Clef-flash 57.1，低于 Jev 57.9 | L1 | ✅ | Cloudflare 演示榜（`b-`/`c-` 存档；`../images/34-clef-leaderboard.png`、`../images/26-clef-demo.png`） | Decision Index 0.2.1：Clef 61.21（#1）、Jev 57.91（#2）、Clef-flash 57.07（#5）；图中四舍五入标 61.2 / 57.9 / 57.1；正文“Clef-flash 为57.1”“Jev 的57.9”即此。“自家榜”指 Cloudflare 演示站，非上游社区榜；未跑的基准计 0（演示站说明，见 `evidence.md` C8）。 |
| 吠点②：成绩是 Cloudflare 自报，上游榜未复现 | L1 | ✅ | Cloudflare 演示榜（`../images/35-clef-self-reported.png`） | “Self-reported. Clef and Clef-flash were run by Cloudflare, the models' authors, and have not been reproduced by the upstream board.” 非 Cloudflare 的成绩沿用上游社区榜 9/28 快照。上游榜（multimodalart 的 Hugging Face Space，Decision Index 0.2.1，“Not affiliated with TypeSafe AI”）未见 Clef。 |
| Jev 延迟含网络往返，Clef 为自家部署，条件不同 | L1 + L3 | ✅ | Cloudflare 演示榜方法页；上游方法页（`c-index-methodology-browser.json`） | 上游：“Jev is TypeSafe's hosted API: a network round-trip from our lab with one sequential HTTPS client … not comparable to on-card latency.”；演示站：“Their latency was measured on the authors' own serving stack, not the upstream reference hardware (1 x NVIDIA RTX PRO 6000), and is not directly comparable.” Clef 延迟具体 GPU、并发、是否含网络，一手页面未写。 |
| 吠点③：Cloudflare 称“开源”，实际放出权重与推理代码 | L1 | ✅ | Hugging Face 模型卡与仓库文件清单（`c-` 存档；`../images/22-clef-license.png`，备用图） | 仓库含 safetensors 权重、joint head、推理代码、tokenizer 与配置，LICENSE 为 Apache 2.0；未见训练代码、训练数据或完整评测脚本（按仓库文件清单，未下载权重）。 |
| 据 The Register，训练数据未公开 | L4 | ⚠️ | The Register（`c-` 存档） | “While described as ‘open source’ in the announcement, Cloudflare AI Platform group product manager Michelle Chen confirmed to The Register that its training datasets aren’t public.” 媒体转述 Cloudflare 产品经理确认，正文按 L4 写“据 The Register”。 |
| 已不入正文但在核验中确认：托管价每百万输入 token 0.24 / 0.09 / 0.042 美元（Clef / Clef-flash / Jev） | L1 | ✅ | Workers AI 定价页；TypeSafe Jev 发布页（`../images/24-clef-pricing.png` 等，备用图） | Clef 约为 Jev 的 5.7 倍；Jev 输出免费，Cloudflare 两卡页面未列输出价；因篇幅未入正文。 |
| 已不入正文：OpenAI 解雇 3 人 | L4 | ⚠️ | WSJ、BBC（`b-` 存档） | OpenAI 发言人向 BBC 称 “parted ways with three individuals for violating our policies on accessing and handling sensitive company information”；WSJ 称 “alleged misconduct”。不确认职责与指控真伪，未入正文。 |

## 备注与未采用的线索

- 扫描称 Reuters 与 OpenAI 的“100+ 对数十家”数字冲突：不成立，是 OpenAI 页面 9/25 与 9/30 两个时点的更新；“dozens”在 8/26 报告里还指 Hugging Face 服务器数，不是机构数。
- 扫描称“美国首次针对失控 agent 的执法”属于加州传票：Reuters/Guardian 该句紧跟 FTC 调查，指 FTC；未找到 FTC 2026 年调查的一手文件，正文未用。
- Tech Times 标题与正文均写 notified，正文明确通知口径宽于确认泄露；线索所称的 “affecting more than 100” 只见于元数据描述，未找到媒体把通知写成被攻破的可靠实例，因此吠点①只引 OpenAI 自己的限定。
- AISI 10/1 博客（恢复大部分评测、暂停网络访问）已存档，因篇幅未入正文。
- Qwen 总榜名次、Anthropic IPO、Pi 1.0 等线索不在本期范围。

## 复核记录

### 卡片视觉复核（跨模型）

方：WSL Codex（gpt-6-astra，effort medium，只读），按 `templates/cards/visual-review.md` 附 13 张图（省流卡、6 张批注图与各自 -raw 底图）。三处手量 rects 高亮（33①、34②、35②）目视均覆盖对应文字，未发现扫入相邻句或漏掉跨行。Claude 逐条处理：

| 级 | 意见 | 处理 |
|---|---|---|
| BLOCKER | 省流卡与 30 号图吠点“未来数周才开始收集”超出原文（原文只说未来数周将收集） | 采纳。正文、省流卡、30 号图吠点统一改为“称‘未来数周’将收集 TPU 的在轨数据”，并重渲 |
| SHOULD_FIX | 省流卡 Cloudflare 条只留跑分，没有“自报”限定 | 采纳。吠点改为“39毫秒属于小号 Clef-flash；成绩是 Cloudflare 自报，上游榜未复现”（两段各自取自正文） |
| SHOULD_FIX | 省流卡 Cloudflare 事实句单独写“开源决策模型”，丢了范围 | 采纳。事实句改为“发布决策模型”，正文吠点③改为“Cloudflare 称‘开源’，实际放出权重与推理代码” |
| SHOULD_FIX | 33、34 号图底图过宽，英文在手机上太小 | 采纳。以无头 Chrome（临时用户目录、2 倍像素比、视口 700 CSS px）从原站重新截取两张底图，原图像素未改 |
| SHOULD_FIX | 33 号图译注“昨天”缺参照日 | 采纳。标题加“10 月 1 日新闻稿” |
| SHOULD_FIX | 省流卡每条应有 2–3 条吠点 | 未采纳。EDITORIAL 写“每条一句事实，加 2–3 条吠点，挑最能纠正标题或流行说法的那几条”，lint 与样例（0926 卡：OpenAI 2 条、Claude 1 条）均按整张卡 2–3 条；本卡共 3 条，首版放 7 条时溢出 872 px |
| NICE_TO_HAVE | 谷歌事实句“轨”孤字成行 | 采纳。事实句改为“Suncatcher 原型卫星搭 SpaceX 火箭入轨” |

### 反向核验（正文、省流卡、译注、封面文字）

方：WSL Codex（gpt-6-astra，effort medium，只读，可联网），范围为 `doc_1002_publish.txt`、`cards.toml`、本表与 `sources/` 存档。该轮读取时文件正在并行更新（成图仍是旧版）。Claude 逐条处理：

| 级 | 意见 | 处理 |
|---|---|---|
| BLOCKER | 成图和本表第 3 行仍写“才开始收集” | 采纳（见上）。成图已按新 cards.toml 重渲，本表旧句已清 |
| BLOCKER | “散热也只在热真空舱测过”的“只”无依据，把未披露当成不存在 | 采纳。正文改“已披露的散热验证是热真空舱测试”，表中相应行重写 |
| SHOULD_FIX | “扛得住五年辐射”漏了指标（总电离剂量），位翻转另有讨论 | 采纳。正文改“耐受五年任务辐射总剂量的结果来自地面测试”，31 号图吠点同步 |
| SHOULD_FIX | 封面“39毫秒”缺统计口径；章节标题“Clef 的39毫秒”又把型号混回去 | 采纳。章节标题改“Clef-flash 的39毫秒”；封面标签加“自报中位延迟”（第 3、4 组候选） |
| SHOULD_FIX | “15分钟”漏了散热约束这个原因 | 采纳。正文加“受散热限制” |
| SHOULD_FIX | “只见于 NPR”的检索范围未写明；混合来源行标 ✅ | 采纳。正文改“出自 NPR，谷歌两篇博文没写”；本表拆成 NPR（L4 ⚠️）与 Cocoloop 转述（L5，只核“该站这样写”）两行 |
| SHOULD_FIX | 发射价漏了“近地轨道”范围 | 采纳。正文加“送往近地轨道的” |
| NEEDS_VERIFICATION | 谷歌博文精确发布时间的字段归属 | 部分采纳。精确时刻来自 Claude 用 Firecrawl 读到的页面元数据（摘录存 `a-claude-google-meta.txt`），Codex 的浏览器存档只有日期；本表已说明来源。正文“北京时间10月2日凌晨”取 SpaceX 发射时刻，不依赖该字段 |
| SHOULD_FIX | 缺配图说明 | 采纳，见 `../images/README.md` |

另：正文文件为 CRLF（与 1001 一致）；`barking lint` 先归一化换行再计字符，终稿计数以最后一次 lint 输出为准（见交付说明）。第二轮反向核验（同方、medium）结论 PASS / no blocker，其 4 条意见（配图说明残留“才收集”、Clef 领先缺主语、谷歌博文精确时刻待核、记录里的字数与封面像素）均已处理：前两条已改，第三条已标待核，第四条已更正。

