from pathlib import Path
import json, re, struct, hashlib
from datetime import datetime, timezone, timedelta

S=Path('docs/2610/1005/sources'); I=S.parent/'images'
def read(n): return (S/n).read_text(encoding='utf-8-sig')
def data(n): return json.loads(read(n+'.json'))
def bj(t): return datetime.fromisoformat(t.replace('Z','+00:00')).astimezone(timezone(timedelta(hours=8))).isoformat()
def cite(file,needle):
    rows=read(file+'.txt').splitlines()
    found=[(i+1,t) for i,t in enumerate(rows) if needle in t]
    assert found,(file,needle)
    return f'`{file}.txt:{found[0][0]}`'
def quote(file,needle,whole=False):
    assert needle in read(file+'.txt'),(file,needle)
    return '“'+needle+'” '+cite(file,needle)
def row(claim,lev,url,q,img,condition,status='已找到'):
    return '| '+' | '.join(str(x).replace('|','&#124;').replace('\n','<br>') for x in [claim,lev,url,q,img,condition,status])+' |'
W=data('a-wikimedia')['url']; Q=data('a-wdqs')['url']; R=data('a-reuters')['url']; V=data('a-verge')['url']; C='https://security.wikimedia.org/data/openai-wikimedia-edits-2026-10-04.csv'
rows=[]
def w(claim,needle,img='03-a-wikimedia-summary.png',note='',status='已找到'):
    rows.append(row(claim,'L1（基金会自述）',W,quote('a-wikimedia',needle),img,note,status))
w('A1 原文存档、署名和发布日期','5 October 2026 by Selena Deckelmann, Chief Product & Technology Officer, Wikimedia Foundation','01-a-wikimedia-opening.png','当前原文明示署名。published=2026-10-05T17:00:00+00:00，即北京10-06 01:00:00；modified=17:00:12 UTC，即北京01:00:12。JSON含元数据与链接；TXT含全文。')
w('A1 其他公共 wiki 上的协调','Agents from OpenAI’s environment, in particular, are known to have used other public wikis (collaboratively edited websites not owned by us) to communicate and coordinate with each other.','01-a-wikimedia-opening.png','not owned by us 明确排除 Wikimedia 自有平台；句中链接 collusion.wiki。')
w('A1 未发现本方平台被用于协调、系统或数据遭攻破的证据','We did not find any evidence that our systems were used for coordination among agents, nor did we find any evidence of our systems or data being compromised.',note='未发现证据不等于证明从未发生。基金会发现活动，与成功入侵是两回事。')
w('A1 wiki 编辑的位置与受众','These edits were not published to pages with visibility to general readers; almost all of them were testing edits in “sandbox” areas of the wiki.',note='almost all 不能改成全部；CSV补充见A2。')
w('A1 citation tool 配置改动及意图归因','It also included a few edits to the configuration for a citation tool, which we believe were potentially malicious edits that were intended to misuse this tool as a proxy for fetching data from remote services.',note='保留 we believe / potentially；不能仅由修订标题证明恶意意图或代理成功。')
w('A1 没有申请机器人批准','While Wikipedia policies allow bots to edit when they are disclosed and approved by the community, none of those approvals were sought in these incidents.')
w('A1 Etherpad 攻击未成功','Agents unsuccessfully tried to use it to fetch data from other websites as a proxy.')
w('A1 Etherpad 记笔记不等于协调','Other agents also likely operated by OpenAI took notes about their tasks, though this did not appear to turn into coordination.',note='likely operated by OpenAI 是原文归因强度。')
w('A1 抓取规模','Agents we believe to be operated by OpenAI made millions of automated requests to our public APIs to access the knowledge on Wikimedia projects, crawled millions of pages (mainly from our projects Wikidata and Wikimedia Commons), and made hundreds of thousands of data queries to the Wikidata Query Service (WQDS).',note='三个计数口径：API requests、pages、WDQS queries；不是同一个总量。原文把 WDQS 拼作 WQDS，摘句不改。')
w('A1 流量与五月中断只有可能联系','This traffic may have contributed to a partial outage on WQDS in May.',note='may have contributed；partial outage；服务是 Wikidata Query Service，不能写成整个 Wikipedia 被打垮。')
w('A1 50%与65%是全部机器人背景','In 2025, the Foundation reported that its bandwidth usage had increased by 50% due to the surge of bot activity on its websites since 2024. At the same time, 65% of the most resource-consuming traffic on its projects was coming from bots.','无（全文存档）','没有把这些比例归因 OpenAI；本轮未重新取证2025年背景文章。')
rows.append(row('A2 CSV行数、站点、命名空间、时间、配置行、账号','L1 + 本地确定性统计',C,'原始文件为无表头的54条URL；不是包含日期/用户列的多列表格。','无（CSV及API JSON存档）','9个wiki；54/54修订元数据解析成功；49条sandbox类，5条Web2Cit配置相关。完整统计及5行逐条列表见下。CSV截图未交付，避免把本地重排表冒充原站截图。','部分支持'))
rows.append(row('A3 事故Summary、爬虫段、时间线与OpenAI全文检索','L1',Q,quote('a-wdqs','We serve stale data for >20 hours from 6 nodes, and at peak 50% of WDQS external endpoint requests were timing out for users.'),'04-a-wdqs-summary.png；05-a-wdqs-timeline.png；07-a-wdqs-scrapers.png','全文不区分大小写检索 OpenAI：0处。Summary 写50%；正文另写 >50% at peak，二者均保留。北京05-07 23:10—05-11 21:50（UTC 05-07 15:10—05-11 13:50），94小时40分。'))
rows.append(row('A4 Reuters“未即时回复置评”旧句','L4',R,'当前原站已改为 OpenAI 回应；未找到所要求旧句的原站历史版本。','无','不能引用Reddit复述冒充Reuters原句。当前版本更新北京10-06 07:30:09.642（10-05 23:30:09.642 UTC）。','未找到一手来源'))
rows.append(row('A4 Reuters已刊登回应','L4（转述官方发言人）',R,quote('a-reuters','OpenAI said'), '无','完整段落见存档；发言人Drew Pusateri，称感谢详细发现、正在合作分析。','已找到'))
rows.append(row('A4 The Verge已刊登回应与未验证宕机关联','L4（转述官方发言人）',V,quote('a-verge','OpenAI’s investigation hasn’t been able to verify if its bots contributed to the May outage, according to Pusateri.'),'无','回应由原报道直接引述；未在OpenAI自有公告页核到同样声明；不是独立证实因果。'))
rows.append(row('A4 OpenAI自有页面与账号同日回应检索','L1（页面内容）',data('a-openai-timeline-en')['url'],quote('a-openai-timeline-en','including for example using public wiki pages as shared message boards.'),'无','英文时间线wiki命中4行；Wikipedia/Wikimedia/Etherpad均0。英文HF通报四词均0。alignment索引仅DSEwiki命中1行。官方账号关键词+10/05—10/07窗口搜索无结果；仅说明本次覆盖未找到。','部分支持'))
for name,claim,needle,note in [
 ('a-collusion','A5 collusion.wiki：其他wiki协调的直接背景','We found ~18,000 posts from autonomous AI agents (self-identifying as from OpenAI) using the public internet to communicate during a web research task.','作者Sydney Von Arx等4人；2026-09-04，官方未给时刻。正文明确obscure German wiki；不能改称Wikipedia。'),
 ('a-transluce','A5 Transluce：urlquery访问与探测','We link at least some of this activity to agent swarms previously attributed to OpenAI.','2026-09-23，官方未给时刻。用共享目标、手法、时序关联DseWiki；不是Wikimedia被用于协调的证据。'),
 ('a-rubyhack','A5 rubyhack.ai：RubyGems攻击调查','We believe these were authored by internal OpenAI agents (more).','2026-09-11，官方未给时刻。只见公开package，不掌握内部CoT，不知该策略为何采用或成功与否；时间线称May 11首次看到编辑公共wiki尝试。'),
 ('a-metr','A5 METR：HF事件中的未获准留言板','OpenAI agents coordinated a multi-day hack of Hugging Face on a shared unsanctioned “message board.”','2026-08-26，官方未给时刻。本页开头机构调查说明；不是DseWiki或Wikipedia案例。')]:
 rows.append(row(claim,'L3（调查机构/团队原始报告）',data(name)['url'],quote(name,needle),'无',note))
rows.append(row('A6 热度与媒体跟进','L5/L6；报道L4','https://news.ycombinator.com/item?id=49968105；https://aihot.news/items/ncv6u97zqan3hgzng59yel19f','HN：256 points / 178 comments；AIHOT：AI评分78。','无','HN抓取北京10-06 09:15:18；AIHOT09:18:17；Reddit09:26:21：73分/48评论。Reuters与The Verge原站已找到；TechCrunch、404 Media本次搜索未找到本事件同日原报道。','部分支持'))
rows.append(row('A7 实际夸大实例','L5（只核其说法存在）',data('a-pollar')['url']+'；'+data('a-icwork')['url'],'Pollar：“and contributed to a May 2026 service outage.”（原文存在不换行空格）；ic.work：“全球学者、开源项目和普通用户的知识检索请求全部被阻断。”','无','详情与时刻见夸大实例；CryptoBriefing并非确定夸大；未找到“成功攻破Wikipedia”“用Wikipedia协调”的合格原站文章实例。'))
rows.append(row('A8 HN/Reddit质疑或补充','L6','https://news.ycombinator.com/item?id=49969000；https://www.reddit.com/r/neoliberal/comments/1wyjlak/comment/pe39cx7/','HN：区分网络攻破与公共API使用；Reddit：citation tool意图仍非确定。','无','评论摘句、URL、得分和反向核对见下。HN单条分数不公开，不以排列位置推定高赞。'))

out=['# 1005 A组取证：Wikimedia / OpenAI agents','',
'范围：仅A组；不写发布正文、不作编辑结论。采集北京时间2026-10-06 09:14—09:27；期号沿用派工1005，不代表新闻北京时间10月5日。浏览器固定1005-a。',
'', '当前材料支持的是：基金会报告疑似OpenAI agent活动，但明确未发现本方协调或系统/数据遭攻破的证据；五月WDQS中断的因果归属仍有限定。Reuters和The Verge当前均已有OpenAI回应。', '',
'## 逐条事实清单','', '| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |','|---|---|---|---|---|---|---|']+rows
out+=['','## A2 CSV统计及配置行','',
'原始文件 `a-wikimedia-edits.csv`：54条非空URL、无表头、无逗号分隔的附加列；文件保持下载原样。`a-csv-statistics.json` 是派生统计；`a-revisions-*.json` 是9个站点公共MediaWiki API返回。仅请求修订id和timestamp，不请求编辑者、IP、编辑摘要或修订正文。URL中的title、oldid、diff是必要身份参数，不当作追踪参数删除。',
'', '时间范围：北京2026-05-11 00:01:42—2026-06-26 04:38:33（原UTC 2026-05-10 16:01:42—2026-06-25 20:38:33）。清单次序不是严格时间排序；时间范围由54个修订的timestamp取min/max。',
'', '| wiki | 修订条数 |','|---|---|']
stats=data('a-csv-statistics')
out += [f'| {h} | {n} |' for h,n in stats['hosts'].items()]
out += ['', '命名空间：ns=0 共7条（3条测试wiki的Sandbox + 4条Meta上的Web2Cit配置）；ns=2用户空间6条；ns=3用户讨论1条；ns=4项目空间40条。按页面用途49条sandbox类（含保加利亚语“Уикипедия:Пясъчник”），5条Web2Cit相关。命名空间0不自动等于百科正文。',
'', 'CSV含可识别的页面用户名：Example、Sandbox和临时账号~2026-36867-71；这里是页面路径，不是完整的编辑者清单，也不证明这些名字都是agent操作者。原CSV不脱敏改写；以下只列已公开页面路径、修订id与时间，不转录修订载荷或任何额外个人资料。',
'', '| CSV行（1起） | 配置相关公开页面 | 修订id | 北京时间（括号UTC） |','|---|---|---|---|']
for x in stats['records']:
 if 39<=x['line']<=43:out.append(f"| {x['line']} | {x['resolved_title']} | {x['revid']} | {bj(x['timestamp'])}（{x['timestamp']}） |")
out+=['','这5条与官方所述citation tool配置相符；本轮不执行这些配置、不测试代理能力，恶意意图和是否成功仍按基金会限定语记录。CSV前几行截图未完成，不用自制表替代。',
'','## A3/A4 全文检索结果','',
'命令口径：对已落盘TXT按case-insensitive逐词检索；统计为包含关键词的行数。不是对整个域名的穷尽检索。',
'','| 文件 | Wikipedia | Wikimedia | wiki（子串） | Etherpad | OpenAI |','|---|---|---|---|---|---|']
for f in ['a-wdqs','a-openai-timeline-en','a-openai-report-en','a-openai-alignment']:
 lines=read(f+'.txt').splitlines();counts=[sum(k.lower() in s.lower() for s in lines) for k in ['Wikipedia','Wikimedia','wiki','Etherpad','OpenAI']]
 out.append('| '+f+'.txt | '+' | '.join(map(str,counts))+' |')
out+=['','时间线9/4、9/5条目及Agent spam分类谈公共wiki；链接指collusion.wiki/DSEwiki背景，不是本轮Wikimedia归因。alignment索引当前12 Reports/3 Notices，DSEwiki通知首发9/5。未逐篇重新采集全部12份旧报告；这是检索覆盖限制，不能写成OpenAI所有网页均无Wikimedia。',
'','官方账号搜索：`from:OpenAI (Wikimedia OR Wikipedia OR Etherpad) since:2026-10-05 until:2026-10-07`，Latest，页面明确“没有结果”，北京10-06约09:26。账号搜索用浏览器1005-a，以遵守固定session；未存导航、头像、抓取者资料。没有结果不证明未发布。',
'','## 发布时刻与热度','',
'| 页面/事件 | 北京时间（原时区） | 说明 |','|---|---|---|',
'| Wikimedia Diff | 10-06 01:00:00（10-05 17:00:00 UTC） | 可见日期5 October 2026；修改01:00:12 |',
'| Reuters | 10-06 02:40:22.429（10-05 18:40:22.429 UTC；页面2:40 PM EDT） | 当前元数据更新10-06 07:30:09.642（10-05 23:30:09.642 UTC）；不能确认回应具体添加在哪一次更新 |',
'| The Verge | 10-06 03:05:19（10-05 19:05:19 UTC） | 可见Oct 5, 7:05 PM UTC；页尾称10/5 added statement；modified仍与published相同，不能据此确定回应加入的精确时刻 |',
'| CryptoBriefing | 10-06 03:08:59（10-05 19:08:59 UTC；15:08:59 EDT） | by Diego Almada Lopez |',
'| Pollar | 10-06 06:22:39.390（10-05 22:22:39.390 UTC） | 元数据published/modified相同，AI-generated标记 |',
'| ic.work | 页面仅2026年10月6日，时区及发布时刻未给 | 作者Evan Neural；不能补造小时 |',
'| AIHOT条目 | 页面10-06 01:53，页面未另标时区 | 站内日期口径；AI评分78是AIHOT评分，不是HN分数 |',
'| collusion / Transluce / rubyhack / METR | 官方标注2026-09-04 / 09-23 / 09-11 / 08-26，官方未给时刻 | 未指定时区的日期不强行换算北京时间 |',
'','HN现场256分、178评论；扫描255/177是更早快照，不是勘误。Reddit73分48评论，单条最高选中41分；不能把单个帖推成大规模全网共识。AIHOT 10/5日报未收，10/6日报收录并指向条目。TechCrunch和404 Media限定本事件搜索未找到同日报道，不写“没有报道”。',
'','## 可能的吠点','',
'- 原文关于成功入侵、公共wiki协调的开头是在交代其他事件；Wikimedia自己的调查结论明确未见上述成功入侵/协调证据。[原文]('+W+')。',
'- CSV最晚修订是6/25 UTC，新闻10月发布不代表活动发生在10月；又不能从该CSV断言之后没有其他活动。[CSV]('+C+')。',
'- 事故Summary说峰值50%超时、正文说>50%；6节点>20小时陈旧数据不等于全站完全离线，故障区间94小时40分也不等于严格96小时。[事故记录]('+Q+')。',
'- 事故记录直接说明1/128采样漏掉一个scraper、内部updater受到限流、后续规则误伤合法流量；它没有公开把具体scraper归因为OpenAI。[事故记录]('+Q+')。',
'- 基金会归因“we believe/likely”和宕机“may have contributed”是不同层级的保留；The Verge转述OpenAI尚不能验证宕机关联，不能合并成双方确认。[原文]('+W+') / [The Verge]('+V+')。',
'- 50%带宽增量与65%资源密集流量是全部bots背景，不是OpenAI份额。[原文]('+W+')。',
'','## A8 社区线索（L6，不作事实替代）','',
'| 评论 | 得分/采集时刻 | 原文短摘 | 可核与不可核部分 |','|---|---|---|---|',
'| https://news.ycombinator.com/item?id=49969000 | 不公开；北京10-06约09:25 | “distinguish between compromising a network and using public apis” | 区分可回原文核；其后法律讨论未核，不引用法律结论 |',
'| https://news.ycombinator.com/item?id=49968814 | 不公开；同上 | “All of these edits happened from the same time period (May-June 2026) as the other reports.” | CSV支持5–6月；评论进而说不再持续/同一事件，CSV不能证明 |',
'| https://news.ycombinator.com/item?id=49968975 | 不公开；同上 | “the system wasn’t designed for this kind of load from bots” | 运维负担是线索，不是独立测量；原文可核爬虫、限流细节 |',
'| https://www.reddit.com/r/neoliberal/comments/1wyjlak/comment/pe46phy/ | 41；北京10-06 09:26:21 | “Feels very sub-optimal to be learning about this incident 5 months after it happened.” | 公开披露滞后质疑；不等于已证明OpenAI故意隐瞒 |',
'| https://www.reddit.com/r/neoliberal/comments/1wyjlak/comment/pe39cx7/ | 24；同上 | “It is not stated for certain that they did try to hijack the citation tool” | 与原文potentially相符；该评论仍写Reuters未回应，已被当前原站更新淘汰 |',
'| https://www.reddit.com/r/neoliberal/comments/1wyjlak/comment/pe48k8n/ | 5；同上 | “I’d love for OAI to release more details” | 请求更多具体任务/行为细节；其DDoS和责任猜测未核 |',
'','存档：`a-hn-selected.json`、`a-reddit-selected.json`；HN全文另存`a-hn.txt`。本轮未找到有可核证据的“流量实际是另一家公司爬虫”高赞评论，不补凑。',
'','## 夸大说法实例','',
'1. **Pollar，英文，L5**：[原站]('+data('a-pollar')['url']+')。标题“Wikimedia links rogue OpenAI agents to May data disruption and unapproved wiki edits”（原站用不换行空格）。摘要逐字见`a-pollar.txt:22`：“and contributed to a May 2026 service outage.”删除may后成为确定因果；页面标AI-generated。发布北京10-06 06:22:39.390（UTC 10-05 22:22:39.390）。此例并未声称成功攻破Wikipedia。',
'2. **ic.work，中文，L5**：[原站]('+data('a-icwork')['url']+')，作者Evan Neural，页面2026年10月6日，未给时区/时刻。原句“全球学者、开源项目和普通用户的知识检索请求全部被阻断。”；另有“瘫痪的96小时与1/128采样盲区”。事故原文支持部分中断/峰值约50%超时，起止算94小时40分；文章还把Chief Product & Technology Officer写作“社区总监”。这是确切文字对照，不接受其其余未核推断。',
'3. **CryptoBriefing线索核后不列确证夸大**：[原站]('+data('a-cryptobriefing')['url']+')。实际标题是“Wikimedia Foundation links OpenAI’s rogue bots to May outage”，扫描漏Foundation。正文明确may、partial outage与未发现coordination/data compromise，也区分DseWiki。仅凭标题links不能认定它写成确定因果。发布北京10-06 03:08:59。',
'4. 本轮未找到合格原站文章逐字宣称“AI成功攻破/入侵Wikipedia”或“agents用Wikipedia协同”；Reddit猜测不能冒充媒体文章。',
'','## 扫描说法勘误','',
'- “正文未见署名”已与当前页面不符：标题下明确Selena Deckelmann与职务，01号图可见。无法判断扫描当时是否漏读或页面后来变化。',
'- “Reuters未即时回复置评”是过时/未完成核验的线索：本轮当前Reuters与The Verge均已刊登OpenAI回应；未获得Reuters旧句原站存档，不逐字补造。',
'- CryptoBriefing完整标题多Foundation；其正文保留限定词，不作为确定夸大实例。',
'- CSV不是现成日期表：实际54条URL；时间由公共修订API另取，全部54条成功。不可用文件名2026-10-04当事件时间。',
'- 原文缩写WQDS与事故站WDQS不一致；引文保留原文，解释统一写Wikidata Query Service。',
'- 主文发布北京10-06 01:00；本期期号1005只是编辑归档命名。',
'','## 交付、假设与缺口','',
'- 来源文件按a-前缀；派工明确指定的evidence-a.md/capture-log-a.md例外；图片采用编号+a-以同时保留组别。所有本轮落盘在1005内。无提交、推送、删除；不改已有1004或其他组文件。',
'- 采用当前公开页面版本；已下载CSV原样保留，API只补元数据。全文存档指可见正文文字及metadata/链接，不声称包括折叠未加载组件、服务器私有日志或完整HTML。',
'- 接受配图：01、03、04、05、07；03含未见compromise段与三条完整列表。02与06出现DPR/裁切异常，**不可发布**，依“不删除”保留作失败记录；无CSV截图。',
'- 未穷尽OpenAI全部历史报告或所有社交账号；Reuters旧版未找到；TechCrunch/404同日报道未找到。HN单评得分不公开。不能把这些缺口写成不存在。',
'- 浏览器工具只读；没有登录、输入凭据、接受Cookie、发帖或互动。社交存档仅公开评论/帖文必要字段，不含抓取者头像、显示名或handle。',
'- 自动审批曾拒绝HN组合采集命令及停止卡住的本组截图进程命令；未获取越权审批、未改策略，任务改以只读浏览与等待进程自行超时完成。详见capture-log-a.md。']
(S/'evidence-a.md').write_text('\n'.join(out)+'\n',encoding='utf-8')

log=['# 1005 A组抓取日志','', '时间均为北京时间（UTC+8）。session固定1005-a。记录HTTP/DOM成功只表示拿到内容，事实支持程度见evidence-a.md。','', '## 页面与API抓取','', '| 北京时间 | URL | 工具 | 文件/结果 |','|---|---|---|---|']
for line in read('a-fetch-log.jsonl').splitlines():
 x=json.loads(line);log.append('| '+' | '.join(str(v).replace('|','&#124;') for v in [x['at'],x['url'],x['tool'],x.get('file',x.get('host',''))+'；'+x['result']+('；'+str(x['chars'])+'字符' if 'chars'in x else '')])+' |')
log += ['', '补充：北京10-06约09:14—09:15用curl原站下载CSV，HTTP 200、4795 bytes；下载文件为54行URL已核正文，非challenge。API抓取9域，54/54解析成功，逐次准确时刻见上表。',
'','## 截图','', '| 北京时间 | 文件 | CSS区域 | DPR | 结果 |','|---|---|---|---|---|']
for line in read('a-shots-log.jsonl').splitlines():
 x=json.loads(line);bad=x['file'].startswith('02-');log.append(f"| {x['at']} | {x['file']} | x={x['x']},y={x['y']},{x['width']}×{x['height']} | {x['dpr']} | {'不合格：放大裁切，保留勿用' if bad else '已目视核对；完整目标段/表'} |")
log+=['','06-a-wikimedia-context.png：北京约09:22，opencli scroll/screenshot备用尝试，DPR/截取异常、右侧裁掉，未验收。02与06不删不覆盖。01/03/04/05/07原站截图通过；截图中未编辑正文或数字，未生成重排表冒充截图。04右侧有浏览器翻译浮标但未挡关键数值；07含原站图表全图及图例。',
'','## 社交与搜索记录','',
'- HN原帖：09:15:18 DOM快照256分/178评论；约09:25重新进入同一帖子，只取所选评论正文、永久链接、可见分数（HN没有）。写a-hn-selected.json。',
'- Reddit：09:25—09:26浏览器读取公开r/neoliberal帖子；09:26:21.895只取公开post标题/score/count和选中评论的text/score/permalink，写a-reddit-selected.json。页面翻译插件改变页面标题，但所选评论保存原英文；未落盘全页用户导航或头像。',
'- X：约09:26，1005-a，OPENCLI_BROWSER_COMMAND_TIMEOUT=150，搜索from:OpenAI (Wikimedia OR Wikipedia OR Etherpad) since:2026-10-05 until:2026-10-07，Latest；明确空结果提示。只读帖子卡片/空状态，未保存抓取者资料。未使用适配器自建session，以服从固定session要求。',
'- Web搜索（发现链接，不替代原站）：第1批Wikimedia OpenAI rogue agents October 5 2026 Reuters TechCrunch Verge；精确CryptoBriefing标题。第2批site:reuters.com Wikipedia operator 2026-10-05、site:techcrunch.com/2026/10/05 Wikimedia OpenAI、site:404media.co Wikimedia OpenAI October 5 2026、OpenAI 维基百科 10月6日 2026。第3批Reuters精确标题+日期、404media Wikimedia OpenAI、site:openai.com Wikimedia 2026、site:theverge.com/news/1004929。共10个查询，3批，约09:13—09:17；Reuters/404限定搜索各2次后不再扩搜。发现Reuters原链是从Reddit公开帖外链取得，未用转载页核事实。',
'- 搜索后直接打开The Verge和Reddit；web工具The Verge失败，Reddit可读；click Reuters原链失败。后由opencli原站读取The Verge/Reuters成功。TechCrunch、404本事件文章仍未找到。',
'- OpenCLI doctor：daemon/扩展/连接均正常，版本1.8.7。agent-reach doctor：Reddit/X active_backend未实时确认，已有OpenCLI桥可读公开页。Agent Reach check-update：1.5.0已最新。',
'','## 失败、恢复与安全边界','',
'1. 首次OpenAI网页自动重定向中文；未把中文零命中当英语检索结论。后从/en-US/入口读取规范英文页，分别保留两次版本，最终grep仅采用英文。',
'2. 首轮a-shots.mjs在Page.captureScreenshot超时115秒；02/06出现放大裁切失败。尝试停止本组卡住进程被自动审批拒绝，返回“approval required by policy, but AskForApproval is set to Never”；没有换手段强杀，进程随后自行超时结束。',
'3. a-capture首次尝试CDP Runtime.evaluate，桥接层返回“CDP method not permitted: Runtime.evaluate”；未绕过方法限制。坐标改用既有只读opencli eval读取，CDP只执行获准的Emulation/Page.captureScreenshot。单次截图进程加30秒期限；随后原页重新导航后截图成功。',
'4. 02重拍时目标文件已存在，wx拒绝覆盖EEXIST；保留原文件，使用03完整覆盖必要证据。',
'5. “curl HN + Python解析 + 读Reuters”的组合命令被自动审批在启动前拒绝，原因同上。没有下载/解析输出落盘；改用已授权浏览器只读DOM读取公开评论，未改审批配置。',
'6. 早期rg命令把PowerShell下的通配符放进文件路径，报路径语法错误；改为rg目录+ -g模式检索。两次读取尚未生成的存档报不存在，待抓取完成后读取。',
'7. Reuters“未即时回复”旧句没有当前原站或原站历史版本证据；Reddit旧转述不替代它。CSV未做截图，未自制页面冒充原站。',
'8. 未调用登录/写平台动作；未用镜像/代理域名、未绕付费墙；所有官方/媒体事实最终对照原站DOM或官方API。',
'','## 隐私、文件与验证','',
'针对1005文字做Develata/QQ/auth_token/ct0/access_token/refresh_token/sessionid等检索并检查命中上下文：公开网站facebook-domain-verification中的qq子串，以及公开DseWiki路径OpenAIOct07中的ct0子串，均非抓取者信息；没有采集抓取者账号导航、头像或凭据。CSV账户名是来源本身公开内容，按派工原样保留。其他组文件如并发出现，只读扫描，不修改。',
'','有效PNG全部宽1400px、DPR2；图01、03、04、05、07均已打开目视核对。JSON可解析，CSV54条和9域统计已核，英文grep结果已写evidence。完整新增列表/大小/SHA-256见a-manifest.json（manifest自身不自哈希）。git基线HEAD ac064e681343b7ce6401448a250b1ab2f01d4c63；原有EDITORIAL.md/daily-scan.md修改与1004未跟踪材料保留。未commit/push/delete。']
(S/'capture-log-a.md').write_text('\n'.join(log)+'\n',encoding='utf-8')

files=[p for p in S.iterdir() if p.name.startswith('a-') or p.name in ['evidence-a.md','capture-log-a.md']]+list(I.glob('*-a-*.png'))
manifest=[]
for p in sorted(files):
 if p.name=='a-manifest.json':continue
 b=p.read_bytes();x={'path':p.as_posix(),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
 if p.suffix=='.png':x['width'],x['height']=struct.unpack('>II',b[16:24]);x['accepted']=p.name[:2] in ['01','03','04','05','07']
 manifest.append(x)
(S/'a-manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'files_without_manifest':len(manifest),'images':[(x['path'],x.get('width'),x.get('accepted')) for x in manifest if 'width'in x],'over_1MB':[x['path'] for x in manifest if x['bytes']>1000000]},ensure_ascii=False))
