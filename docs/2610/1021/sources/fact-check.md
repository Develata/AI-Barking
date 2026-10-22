# 1021 事实核验

最终发布版：856字（包含标题、空格、换行），标题为“claude也出BOT；安全问题扎堆”；各段事实范围沿用下表。907字自审版保存在pre-style-review.txt，旧复核记录保留对应当时版本。历史记录中的1022为制作时的期号，最终归档1021。

状态：用户已确认发布。作者及取证为 GPT 系；独立审稿会话的反向及视觉复核见下文，不称不同厂商模型交叉验证。最终换标题后的双封面由主代理看图核对，未另做独立模型视觉复核。核验日期为北京时间 2026-10-22；从用户上午扫描继续取证至当日傍晚，未把后续核验冒充11:05时已知。✅ 仅指已对照一手资料，不代表独立实测产品、复现漏洞或审计企业隐私承诺。

## 文中事实

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Claude Duet 研究预览，仅美国 Pro、Max 用户可加入候补 | L1 | ✅ | [Anthropic](https://claude.com/resources/articles/claude-duet)，a-duet.txt | 10/21 仅日期；不是正式全面开放；省流卡逐字摘用 |
| 云端虚拟机运行，手机电脑关闭仍可工作，提供每日简报和连接应用任务 | L1 | ✅ | a-duet.txt | morning brief；virtual machine with its own browser；不要求设备开机。均为产品说明，未独立测试 |
| 每隔数小时检查已连接账户，与普通 Claude 共用额度，后台消耗额度 | L1 | ✅ | a-duet.txt | every few hours；same usage limits；不是实时无限运行 |
| 发送、支付、签文件、删除前展示操作并等审批；证券交易和账户间转账即便批准仍禁止 | L1 | ✅ | a-duet.txt | “Important decisions stay with you”段完整限定 |
| 厂商仍在测试何时提醒用户的判断；演示不等于可靠性保证 | L1 | ✅ | a-duet.txt | 原文 continuing to test its judgment；后半为证据边界判断，无独立成功率 |
| LiteLLM 1.107.0 的 PyPI 发行物和部分容器标签含窃密恶意软件，GitHub v1.107.0 源码标签不含恶意文件 | L1 | ✅ | [LiteLLM](https://docs.litellm.ai/blog/security-incident-oct-21-2026)，b-litellm.txt | 最后更新10/21 20:30 UTC，即10/22 04:30北京时间；7个wheel，特定Docker/GHCR标签；不泛化历史版本或Enterprise私有渠道 |
| 发布流程改用攻击者仓库；受影响用户按官方指引清理或重建环境、轮换凭据，不能只卸载或升级 | L1 | ✅ | b-litellm.txt | 仓库 Berri-AI 与 BerriAI 混淆；原文What to do if you are affected要求先清理启动恶意文件与后台进程，最好重建，轮换可接触的凭据。不要在污染环境直接运行pip。初始内部入侵路径仍未确认；本稿只提处置原则，不充当完整操作手册 |
| PHP 修复两项严重远程代码执行漏洞 | L1 | ✅ | [PHP](https://www.php.net/archive/2026.php#2026-10-21-5)，b-php.txt | 10/21发布；修复8.5.12/8.4.27/8.3.36/8.2.35；PoC争论不作为本文事实 |
| Node.js 发布安全更新 | L1 | ✅ | [Node.js](https://nodejs.org/en/blog/vulnerability/october-2026-security-releases)，b-node.txt | 10/21；26.x、24.x、22.x三条版本线，不能写所有历史版本都已修复 |
| Rustup 更新证书依赖，应对 eMudhra 中间证书问题 | L1 | ✅ | [Rust](https://blog.rust-lang.org/2026/10/20/Rustup-1.29.2/)，b-rust-blog.txt；[RustSec](https://rustsec.org/advisories/RUSTSEC-2026-0382.html)，b-rustsec.txt | Rustup 1.29.2，仅rustls-webpki/webpki-root-certs依赖；官方10/20仅日期，36小时边界不确定。用户指定聚合，未宣称本日首发 |
| 投毒、运行时漏洞、证书信任不是同一故障；Rustup升级不等于全部Rust应用修复 | L1 | ✅ | b-litellm.txt、b-php.txt、b-node.txt、b-rust-blog.txt、b-rustsec.txt | 编辑归类，基于各公告范围；受影响组件应用须查自身依赖，不将语言整体称为被攻破 |
| ChatGPT 用独立系统选广告，模型利用对话上下文；独立AI协助选广告 | L1 | ✅ | [OpenAI工程文](https://openai.com/index/building-chatgpt-ad-stack/)，a-ads-stack.txt | 原文明确separate models and separate code；don’t share a content generation model。A组“独立LLM仅部分支持”的初判过严，主核更正为✅；复核指出模型不直接决定最后展示广告，最后经过拍卖，因此改成独立系统、模型协助，不写模型直接选定 |
| 用户选择个性化广告时，考虑历史聊天非敏感信息 | L1 | ✅ | a-ads-stack.txt | If a user chooses to personalize ads；不能省略自愿个性化条件 |
| 广告只面向 Free、Go 方案 | L1 | ✅ | [扩展公告](https://openai.com/index/chatgpt-ads-expands-26-new-countries/)，a-ads-countries.txt | 指符合地区、账户资格的Free/Go，并非每个Free/Go用户都会看到；Plus/Pro/Enterprise无广告 |
| OpenAI 称广告与答案生成分开、不出售用户数据 | L1 | ✅ | a-ads-stack.txt、a-ads-countries.txt | 明确署名公司承诺；使用上下文不等于出售聊天。外部隐私效果未独立审计，正文未将承诺写成已验证结果 |
| 标题“claude也出BOT；安全问题扎堆” | L1 | ✅ | a-duet.txt及上述四项安全公告 | BOT为Duet软件Agent的通俗称呼，不是实体机器人；四项安全事件相互独立，不暗示Duet被攻破，也不表示全球全面开放 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 等称：数千智能体32分钟梳理既有数百万小鼠脑细胞数据，提出线索。 | L1 | ✅ | [OpenAI](https://openai.com/index/scaling-ai-research-for-complex-biological-data/)，c-openai-brain.txt | 10/21仅日期；厂商/伙伴研究叙述；existing experimental data，不是32分钟做完湿实验或产生临床结果 |
| Mistral 称，其中的 Small 转写模型支持50+语种。 | L1 | ✅ | [Mistral](https://mistral.ai/news/voxtral-3/)，c-voxtral-3.txt；c-voxtral-dates.json | 图上标签Voxtral-3；对应Small Transcribe 3；不指Mini批量、Realtime或TTS。发布10/20 12:00:22 UTC = 北京20:00:22，窗口前；10/21更新。原始JSON-LD及HTML哈希见c-voxtral-dates.json，未称今日首发 |
| Google 称，AI 盲扫超声在孕16–36周估算孕周。 | L1 | ✅ | [Google Research](https://research.google/blog/empowering-frontline-maternal-care-with-ai-assisted-ultrasound/)，c-google-ultrasound.txt | 10/21公司介绍，385人主队列；只估算孕周。JAMA全文403，未独立核论文细节，不称医学临床建议 |

主帖速览三行仅摘正文小标题，不新增事实；顺序Claude、安全、ChatGPT，与省流卡一致。

## 批注译注

| 文中事实（图上译注） | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Claude Duet 每隔数小时检查你连接的账户。 | L1 | ✅ | a-duet.txt；02-duet-controls-raw.png | checks your connected accounts every few hours；非连续区域同页拼接，图上明示中间省略 |
| 有些事即便获批也不允许，例如证券交易或账户间转账。 | L1 | ✅ | a-duet.txt；02-duet-controls-raw.png | A few things are off-limits even with your approval, like trading securities or moving money between accounts. |
| 这个版本包含编译后的恶意软件，会窃取凭据并加载远程代码执行工具。 | L1 | ✅ | b-litellm.txt；17-litellm-intro-raw.png | This version指1.107.0；没有扩成所有历史版本 |
| 强烈建议所有 PHP 用户立即升级。 | L1 | ✅ | b-php.txt；14-php-raw.png | All PHP users are strongly encouraged to update immediately. |
| 针对下列问题，Node.js 的 26.x、24.x、22.x 版本线已提供更新。 | L1 | ✅ | b-node.txt；15-node-raw.png | Updates are now available for the 26.x, 24.x, 22.x Node.js release lines for the following issues.；原图保留下方问题与依赖列表 |
| 仅更新 rustls-webpki 与 webpki-root-certs 两项依赖。 | L1 | ✅ | b-rust-blog.txt；16-rust-raw.png | only updates the rustls-webpki and webpki-root-certs dependencies；原图上方保留Rustup1.29.2版本语境 |
| 广告选择过程在独立服务中运行，与 ChatGPT 的回答生成分开。 | L1 | ✅ | a-ads-stack.txt；03-ads-raw.png | running the ads selection process separately from ChatGPT’s answer generation, in its own service；图上只陈述服务分工，不把“不影响回答”升级为已独立验证；同页拼接、中间省略 |

## 反向核验

已完成，PASS / no blocker。仓库工具启动WSL Codex gpt-6-astra、medium、只读、联网独立审稿，报告见reverse-review-astra.md。逐行核对16条正文事实、3条速览、7条译注，并打开11个官方网页。主代理逐条采纳如下：

1. “独立模型选取”不应混淆模型候选推荐与auction最终选择：改成独立系统选广告、模型理解需求；省流、正文、事实表、卡片均同步。
2. PHP标题从“官方要求”改成“官方强烈建议”，与原句一致。
3. Voxtral秒级时间补存原始JSON-LD到c-voxtral-dates.json，带HTML文件哈希；最终短审逐字对照原始HTML确认。

最终广告措辞和时间证据补核见visual-review-astra.md的短审部分。原尝试Claude Sonnet只读复核因OAuth过期失败，失败记录reverse-review-sonnet.md保留，不计入完成证据。复核实际由不同独立GPT审稿会话完成，不宣称跨厂商核验。

## 视觉复核

已完成，PASS / no blocker，报告见visual-review-astra.md。实际打开10张最终图（两封面、省流、速览、6张批注）与6张raw底图。所有高亮、跨行、译注、图上标题、页脚及顺序已检查；新版省流和PHP图在重渲后重新打开确认。

并行WSL视觉审稿因网络重连耗时，报告visual-review-wsl-initial.md使用启动时附带的旧省流/PHP图片，指出旧句与新配置不一致；两项均已通过重渲和重新实际开图闭环。它还称Node译注少“已”字：主代理对最终PNG译注区域放大复查，清楚可见“版本线已提供更新”，配置/事实表也一致，因此不采纳该误读。

主代理另检查横封面中心820×820方形裁切，标题、问号、全身人物完整；双封面实际尺寸1086×1448、1919×820。平台实际裁切/压缩未测试；没有逐像素证明或第三方漏洞复现。视觉审稿人曾承担B组原文取证，但未参与正文或卡片排版，报告明确该角色边界。

## 最终交付检查

以下902字记录为上一轮交付快照；本轮用户要求主代理自审后已更新为907字，当前结果及改动见下节，不将旧独立审稿结论冒称为新一轮独立审稿。

`barking lint docs/2610/1022`：0错误、0警告；标题12/20字，全文902/1000字。`git diff --check`无空白错误。入库候选文本无单个超过1MB的文件；采集响应头未发现Set-Cookie或Authorization字段。`barking offsite`仅生成39个本地原件的大小与SHA-256清单，0失败，没有执行上传。README/daily-scan的已发布索引未改，未提交推送。待用户终审及实际发布。

## 用户要求的主代理自审

已完成，见self-review.md。本轮补充LiteLLM处置原则；省流卡补齐四项安全事件；广告图高亮改为独立服务分工，避免只看图片时忽略历史聊天个性化条件。主代理实际检查重渲后的省流/广告图及其余6张信息卡，未发现剩余阻断项。正文907/1000字，lint 0错误、0警告，offsite39项哈希已刷新。旧独立审稿报告保留为之前版本记录，本轮不将作者自审称为新的跨模型复核。

## 文笔与文风修订

用户要求优化文笔文风后，将报告腔改成短句与具体动作：额度限制写“后台也要算账”；源码与发行物差异写“查过源码，还得看装到手里的包”；移除重复的“使用上下文不等于出售聊天”解释。保留美国Pro/Max候补、数小时间隔、共享额度、审批与禁止交易、受影响版本及部分镜像、Rustup范围、广告个性化选择与非敏感信息等限定；未增加新的事实主张。

第二小标题及速览主帖行同步改成“安全问题扎堆，处置要分清”。省流逐字摘句和6张批注图的文字、底图未改，仅重渲速览图。主代理实际查看新版速览，主标题一行内显示完整，其他5条未改变。lint 0错误、0警告，全文850/1000字，offsite39项清单刷新，无上传、提交或推送。定点独立短审另记style-review-astra.md。

定点独立短审完成：gpt-6-astra，PASS / no blocker。核对润色前后事实及限定词，实际打开当前速览、省流和广告图，未发现事实变化、高亮或裁字问题；没有将未重新联网及未重审其余图卡称为覆盖。
