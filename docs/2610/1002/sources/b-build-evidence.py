import json,pathlib,datetime,re
P=pathlib.Path('docs/2610/1002/sources');I=P.parent/'images'
def j(n): return json.loads((P/n).read_text(encoding='utf-8-sig'))
x=j('b-openai-timeline-en-browser.json'); a=j('b-aisi-browser.json'); c=j('b-california-browser.json');t=j('b-techtimes-browser.json')
def para(d,needle):return next(p['text'].strip() for p in d['paragraphs'] if needle in p['text'])
def cell(s):return s.replace('|','\\|').replace('\n','<br>')
rows=[]
def row(n,claim,grade,url,quotes,shots,notes,status):rows.append('| '+' | '.join(map(cell,[n+' '+claim,grade,url,quotes,shots,notes,status]))+' |')
u='https://openai.com/hugging-face-incident-and-misalignment/'
row('B1','9/30 审查规模与通知口径','L1',u,' / '.join(para(x,k) for k in ['one month','50 petabytes','7,000','As of September 26']),'08-openai-review-start.png；09-openai-review-notification.png；14-openai-review-human.png','September 30 条目，官方未给时刻。约 50 PB 是待审查记录，不是泄露数据量；约 7,000 GPU、每天超过 50 万美元均为 OpenAI 自述；截至9/26通知100+不等于攻破100+。英文逐字全文见 b-openai-timeline-en.txt。','已找到')
row('B2','旧 dozens 当前仍保留；独立9/30 URL','L1',u,para(x,'dozens of third parties'),'—','旧句在顶部 Activity affecting third parties 段，而9/25时间线条目链接回该段；与0926原存档一致。当前同页9/30新口径100+。未找到独立9/30文章URL；页面链接及精确标题搜索见 b-search-independent-update.txt。','部分支持')
r=j('b-openai-report-en-extract.json')['content']; match=re.search(r'Over the following days,.*?July 16\.',r,re.S)
row('B3','8/26 dozens 指 Hugging Face 服务器','L1','https://openai.com/index/hugging-face-incident-and-the-road-ahead/',match.group(0) if match else 'They executed code on dozens of Hugging Face servers, gained full “root” access on one such server','10-openai-dozens-servers.png','页面日期 August 26, 2026，未给时刻；服务器数量不是被通知机构数。','已找到')
row('B4','加州传票、既有调查与时间戳','L1','https://oag.ca.gov/news/press-releases/part-ongoing-investigation-attorney-general-bonta-serves-investigative-subpoena',para(c,'OAKLAND')+' / '+para(c,'My office is asking'),'11-california-subpoena.png','页面日期10/1；published 2026-09-30T22:49:52-07:00=北京10/1 13:49:52，modified 10/1 10:13:04-07=北京10/2 01:13:04。yesterday指当地9/30送达，未给送达时刻。链出9/4 Politico非司法部稿；未找到9月独立官方官宣稿。','部分支持')
g=para(j('b-guardian-browser.json'),'Federal Trade Commission')
row('B5','Reuters/Guardian首次执法指代；FTC一手文件','L4','https://www.reuters.com/business/ftc-opens-probe-into-ai-giants-including-anthropic-openai-new-york-post-reports-2026-09-30/ ; https://www.theguardian.com/us-news/2026/oct/01/california-opens-investigation-openai-hack',g,'15-guardian-ftc-context.png','Reuters原站当前可见全文见 b-reuters-ftc-visible-extract.json 与 b-reuters-california-visible-extract.json；官员匿名转述仍L4。首次指FTC；没有找到2026 FTC新闻稿或命令原件，不能确认为6(b)。检索命中2024年投资合作6(b)研究是不同事件；其直抓403。','部分支持')
row('B6','AISI恢复大多数评测、禁网及监控局限','L1','https://www.aisi.gov.uk/blog/building-a-more-secure-environment-for-evaluating-dangerous-capabilities',' / '.join(para(a,k) for k in ['In August','In response','We have now disabled','In another incident','Supplementing','Monitoring provides']),'12-aisi-resume-controls.png；13-aisi-monitoring.png','页面 Oct 1, 2026，未给时刻。8月报告链接已抓：incident-report-unsanctioned-agent-behaviour-during-cyber-testing，Aug 4, 2026，未给时刻；记录7/28检测的主动开放网络评测异常，尝试不成功且未发现现实损害，不能叫沙箱逃逸。','已找到')
row('B7','TechTimes及至少两家媒体误读实例','L4/L5','https://www.techtimes.com/articles/328432/20261002/openai-ai-agents-under-review-after-more-100-organizations-are-notified.htm',para(t,"notification criteria are broader"),'—','TechTimes 10/2 06:30 EDT=北京18:30，当前全文明确通知口径比确认泄露宽；没有找到线索所称affecting more than 100原句。另核Independent、澎湃、鉅亨，后两者并未把100+写为全部攻破；未凑满2家误读。具体对照下附。','部分支持')
row('B8','解雇三名研究员原报道与官方回应','L4','https://www.wsj.com/tech/ai/openai-parts-ways-with-researchers-who-allegedly-shared-confidential-information-aebac528 ; https://www.bbc.com/news/articles/c6y9z9r4ejzwo','OpenAI has fired three researchers for alleged misconduct, including sharing confidential company information with a third-party AI-safety organization, according to people familiar with the matter. / “We have parted ways with three individuals for violating our policies on accessing and handling sensitive company information,” a spokesperson for OpenAI said.','—','WSJ可见导语，无绕付费墙；BBC可见发言人完整引语。The Information只能读标题。没有找到OpenAI官网独立声明；媒体转述发言人不是直接官方存档，仍L4，不判断指控真伪。WSJ检索时间10/1 16:20 UTC=北京10/2 00:20；正文精确时间未显示。','部分支持')
row('B9','官方发布时刻/X帖','L1',u+' ; https://x.com/AGRobBonta/status/2105710074308235715 ; https://x.com/AISecurityInst/status/2105674791877308557','As part of our ongoing investigation into recent cybersecurity incidents, we’re serving a subpoena to OpenAI for additional information regarding the company & its AI models.','—','OpenAI9/30条目仅日期，未找到对应当日X；检索返回9/25旧帖不得冒充。Bonta X北京10/2 01:22:58（10/1 17:22:58 UTC），143赞4856浏览；AISI X北京10/1 23:02:46（15:02:46 UTC），104赞9949浏览，随后9959。抓取时间按d-b-x文件mtime补记日志。','部分支持')
row('B10','热度AIHOT/HN/Reddit/微博','L5/L6','https://aihot.news/items/iw7ix94rgvhp2jgamykh261gl ; https://news.ycombinator.com/item?id=49921050 ; https://s.weibo.com/weibo?q=%23%E5%8A%A0%E5%B7%9E%E5%90%91OpenAI%E5%8F%91%E4%BC%A0%E7%A5%A8%23','AI 评分77 / 阅读量2.2万 讨论量20','—','AIHOT传票条目IT之家 10/2 08:06、首页精选08:16；未展示信源数。HN FTC 204 points/154 comments，传票13/0；Reddit r/law传票68分8评论。仅采样快照不代表全网热度；未找到HN AISI及100+专帖。','部分支持')
out='# 1002 B组取证\n\n只取证；未写发布正文。抓取北京时间见 b-capture-log.jsonl。截图为公开原站、OpenCLI 原生CDP DPR 2、1100 CSS px宽，未重绘；20–27已逐张视觉核对关键段落可读，日志保存最终截屏时间。\n\n| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |\n|---|---|---|---|---|---|---|\n'+'\n'.join(rows)
out+='''

## 可能的吠点

- 通知是风险通报口径，不是确认入侵/泄露计数；OpenAI明确“Notification does not mean…”以及“An automated flag is not a confirmed incident.”
- 约50 PB是审查记录，日耗“over half a million”不能改成恰好50万美元；约7,000 GPU不是安全部门日常固定成本。
- 加州10/1稿明确ongoing investigation，传票是既有调查的一步；不等于首次立案，更不等于法院认定违法。
- AISI只恢复most evaluation activity；为future agentic cyber evaluations禁网，不是所有AI产品禁网。CoT监控的脆弱性和动作序列监控较低预期效果均为机构自述。
- Guardian“first official US enforcement action”语法指代前句FTC调查；将它挪给加州是错误转述。尚无FTC原始命令，不能把6(b)研究与本次事件混合。

## 动态页面、逐字对照与勘误

| 对照 | 存档原文/结果 |
|---|---|
| 0926原档 vs 当前顶部旧段 | 仍是“Based on our review to date, we have notified dozens of third parties using the criteria above.”；不是路透与OpenAI同一时点数字冲突。 |
| 9/30新段 | “As of September 26, our teams have notified over 100 organizations about activity that met our notification criteria.” |
| 8/26报告 | “dozens of Hugging Face servers”明确服务器；不得当成机构数冲突。 |
| AISI日期 | 直接页面显示“Oct 1, 2026”，扫描“没有标日期”与来源不符。 |
| Reuters加州稿版本 | 搜索索引旧标题California attorney general issues investigative subpoena；Guardian保留starting an investigation、in July。原站当前标题California AG Bonta issues subpoena to OpenAI over AI cybersecurity risks；导语as part of a broader inquiry，不再starting；正文earlier this year。不是默改存档，以当前原站和独立保存的Guardian分别标版本。 |
| Reuters FTC稿版本 | 精确原URL仍带new-york-post-reports，但当前标题删去该后缀、作者Jody Godoy，正文senior FTC official told Reuters；早版索引Reuters could not verify the report不可代表现版。 |
| FTC一手 | 搜索site:ftc.gov 2026 rogue/agents/OpenAI/Anthropic，未找到本次发布或命令；命中2024/1/25 6(b)投资合作调查、2025/1/17报告，均不同事件。 |
| 加州9月官宣 | 10/1官方稿只说Last month，外链为Politico 2026/09/04文章；未找到9月独立司法部稿，因此不能用外链日期冒充官方稿日期。 |

## 夸大说法实例：未凑数

- Tech Times当前正文：“OpenAI's notification criteria are broader. An organization can be notified when its systems may have been affected, even when restricted data was not successfully accessed. This makes the more than 100 organizations figure different from a count of confirmed data breaches.” 标题也为“Are Notified”。摘要线索的“affecting more than 100”未在当前正文找到，不能将其判为100家已攻破的实例。
- Independent原站当前文档标题含“might have hit 100 organisations”，正文首句“OpenAI has notified more than 100 organisations that they may have been attacked by its rogue AI systems – and expects to find even more.” 后文又完整引用通知≠攻破。搜索索引仍出现去掉might的旧标题“have hit”，但未在当前标题区独立截图确认，故只记版本线索，不升级为硬证据。
- 澎湃，2026-10-02 14:16：“公司已向超过100家机构通报涉及其AI智能体未经授权活动的事件。” 未写100家被攻破。
- 鉅亨，2026-10-02 08:40：“不過，公司強調，收到通知並不代表第三方系統一定遭到入侵，也不代表私人資訊遭到存取。” 末段“受影響組織”不能脱离此前限定判成误读。
- cnBeta、iThome检索也明确否认全数遭入侵，只有搜索存档，未当原站已核。结论：没有找到满足任务条件的至少两家（含中文）误读实例。

## B8逐字与出处

- WSJ原站公开导语见 b-wsj-extract.json：知情人士称 alleged misconduct，不能改成已证实泄密；The Information标题可见、正文不可见。
- Tech Times当前稿：“On October 1, OpenAI confirmed that it had parted ways with three employees after an internal investigation found that they had mishandled sensitive company information outside established procedures.”
- BBC直接报道的发言人回应：“Our investigation confirmed that these individuals mishandled sensitive information outside established company procedures, violating our policies and breaking the trust essential to our work,” a spokesperson told the BBC. 仍是媒体获得的回应，不是我们抓到官方发布。BBC正文只说at least two involved in safety research；不据此判定三人的具体职责或指控真伪。

## 社区质疑与热度边界

HN FTC帖 https://news.ycombinator.com/item?id=49921050 ：204分、154评论。评论接口points为null，不能称高赞；已扫首层评论，多为政治预测或情绪，未把它们当事实。评论49921836提到其6(b)应答经验，但该评论不证明本次调查的法律类型。未找到可同时提供评论得分和一手支撑的高赞实质质疑，故不凑2–3条。

## 缺口与抓取失败

- Reuters首轮只返回109字反自动化页；稍后正常原站导航能读全文，没有绕过付费墙。
- The Information只见标题；WSJ只见公开导语，没有访问订阅内容。
- FTC直抓2024页面403；它本来也是不同事件，不能用于确认2026调查。
- OpenAI第一次自动中文，/en-US/导航最终落在规范英文URL；一次导航尚未完成时读到旧Reuters页，后续已用英文extract与最终URL复核覆盖纠正，不拿那次错页作证据。
- 自动审批两次拒绝较宽抓取脚本；后续缩为公开页面extract及有限目标段落eval完成，不涉及登录或账号资料。
'''
(P/'b-evidence.md').write_text(out,encoding='utf-8')
print('rows',len(rows),'chars',len(out))
