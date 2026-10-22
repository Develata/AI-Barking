# 1022 最终图卡独立视觉复核

结论：PASS / no blocker。

审稿：gpt-6-astra（medium），2026-10-22。按 templates/cards/visual-review.md，使用实际 view_image 逐张查看成图与底图，而非仅检查cards.toml或OCR。审稿人与主代理排版工作分开；本审稿人也承担过B组原始取证，因此属于独立于排版/正文修改的复核，不能表述成第三方安全调查或不同厂商模型交叉核验。仅写本报告，未修改产品文件。

## 实际查看范围和结果

| 图 | 检查结果 |
|---|---|
| 00-tldr.png | PASS。初次打开仍为重渲前的“独立模型选取”旧图，已通知主代理；随后重新打开已落盘新版，确认为“OpenAI 披露：ChatGPT 用独立系统选广告，模型利用对话上下文判断需求。”。三条与正文对应，无截断、无新增判断；Claude候补/地区/套餐限定保留。新版文件修改时间本机06:09:05（北京时间18:09:05）。 |
| 00-roundup.png | PASS。6条，主帖三条位于最前且顺序与省流一致，其余三条逐字对应fact-check速览表。保留OpenAI等称、既有数据、提出线索；Mistral称/Small转写/50+；Google称/16–36周/估算孕周。未把转写能力扩大为所有语音模块，未写临床诊断。无溢出。 |
| 00-cover.png | PASS。标题“AI主动上班，谁来把关？”逐字清楚；问号存在。纸艺示意图与审批题意相符，没有虚构新闻数字或人物引语。看板娘手脚无明显多肢，图中文字无明显错字。 |
| 00-cover-wide.png | PASS。相同标题，横构图重新排版；主标题位于中央安全区域，未见裁字。人物与主题物件清楚。 |
| 02-duet-controls.png + raw | PASS。高亮1只含Claude Duet checks…every few hours，两行完整，未吞后面的because；高亮2覆盖A few things…accounts全句三行，未扫入下一句disconnect。编号/颜色/译注对应，even with your approval保留。原图两段均完整，页脚明示同页拼接、中间省略。 |
| 17-litellm-intro.png + raw | PASS。高亮从This version开始，跨三行到execution tools句号，未把All previous versions are safe高亮。译注忠实；上文v1.107.0及下文Enterprise私有版本不受影响仍可见。原图无实质遮挡/裁掉。 |
| 14-php.png + raw | PASS。标题已为“官方强烈建议立即升级”，未夸为官方命令。高亮准确覆盖All PHP users…update immediately两行，译注一致；完整Who is at risk段保留。 |
| 15-node.png + raw | PASS。高亮覆盖版本线更新整句两行，包括末行issues；26.x/24.x/22.x无漏。依赖清单完整保留；没有把图解释为全部历史版本修复。 |
| 16-rust.png + raw | PASS。高亮仅为only updates…dependencies，两行覆盖webpki-root-certs跨行单词；未扫入to address。译注准确，上方Rustup1.29.2与工具定义、下方证书问题上下文都在。原图最底Further details一行保留，可读。 |
| 03-ads.png + raw | PASS。高亮information is…multiple conversations两行完整，不含后面2)。译注只陈述状态保留，与原句一致；没有添加出售数据或广告影响回答判断。页脚明确同页拼接、中间省略。 |

10张最终图及6张底图均实际打开。所有中文标题、译注、页脚未见被禁止的平台名（原文截图不算）。荧光高亮透明，编号位于左边缘且不遮字；本次cards.toml未配置underline，不存在需核验的新下划线。原文原有链接下划线仍保留。

批注卡截图与译注紧邻成组；部分短卡上下留白较多，但文字可读，无重叠、溢出或压缩成不可读字号，不构成需修问题。正式图片是否经过平台二次压缩不在本地检查范围。

## 广告短审

PASS。已对照a-ads-stack.txt第110–140、188–212行。原文说明独立服务/模型/代码；orchestrator理解上下文、提名候选，auction负责最终选择和定价。正文“用独立系统选广告，模型利用对话上下文判断需求”以及省流“独立 AI 协助选广告”避免了模型直接决定最终广告的错误概括。正文保留用户选择个性化广告时才参考历史聊天非敏感信息，并将不出售数据、不影响回答明确记为OpenAI声明，而非独立审计结果。

标题“另一套 AI 在选广告”作为系统层简写可接受，正文立即交代独立系统和模型角色，没有声称排除拍卖环节。

## Voxtral 日期原件短审

PASS。c-voxtral-dates.json中的json_ld_raw与c-voxtral-3.html实际内嵌字段一致：

- datePublished = 2026-10-20T12:00:22.000Z，即北京时间10月20日20:00:22。
- dateModified = 2026-10-21T09:56:32.450Z，即北京时间10月21日17:56:32.450。
- HTML SHA256实际计算为16A99C07C09CE354316A2F8E7F8CD2B776AC5C9E23FBDA2B0F206806D8A67F7E，与记录一致。

该材料证明10/20发布、10/21更新，不能写10/21首发。当前卡片无“今日首发”，fact-check已正确区分；速览收录日期不等于原文发布日期。

## 未覆盖与证据边界

没有联网刷新动态公告，没有在真实平台上传或模拟二次裁切/压缩，没有对PNG做逐像素同一性证明；原图完整性按并排视觉核对，不应把“未见改动”升级为密码学像素验收。未审全部备用长图、封面落选候选或所有底层取证文件。未独立运行Duet、审计广告系统、复现安全漏洞或验证医学论文实验结果。上述边界不影响本轮成图与文字一致性结论。

本轮最终结论：PASS / no blocker。未发现需要列为SHOULD_FIX的问题。
