# 未执行的生成草案：自动审批拒绝其执行。不是事实清单、不是已验收程序。
# 权威交付为人工写入且另行只读核对的 evidence.md / capture-log.md。
# 保留此文件仅因本任务禁止删除文件；其中自动生成方案和页码不得当作已核实证据。
from pathlib import Path
import re, json, datetime, hashlib
from bs4 import BeautifulSoup

S = Path(__file__).resolve().parent
I = S.parent / 'images'
BJ = datetime.timezone(datetime.timedelta(hours=8))
def norm(t): return re.sub(r'\s+', ' ', t).strip()
def read(f): return (S/f).read_text(encoding='utf-8-sig')
def stamp(f): return datetime.datetime.fromtimestamp((S/f).stat().st_mtime, BJ).isoformat(timespec='seconds')
A='https://arxiv.org/html/2609.22978v1'
AA='https://arxiv.org/abs/2609.22978'
B='https://www.anthropic.com/news/claude-discovers-novel-enzyme-system'
BP='https://www-cdn.anthropic.com/22573675ada52a8ca8a97a1a4b4326b2f208a071.pdf'
C='https://alignment.openai.com/misalignment-reports/self-replicating-prompt-injections-exist/'
af='a-arxiv-paper-paragraphs.md'; aaf='a-arxiv-abs.md'; bf='b-anthropic-paragraphs.md'; pf='b-technical-report-normalized.md'; cf='c-openai-paragraphs.md'
def q(f,t):
    assert norm(t) in norm(read(f)), (f,t)
    return {'file':f,'quote':t}
def para(f,needle):
    return q(f,next(p for p in read(f).split('\n\n') if needle in p))
def span(f,start,end):
    t=norm(read(f)); x=t.index(start); y=t.index(end,x)+len(end)
    return q(f,t[x:y])
rows=[]
def add(id,claim,level,url,quotes,shots,conditions,status='已找到'):
    rows.append(dict(id=id,claim=claim,level=level,url=url,quotes=quotes,shots=shots,conditions=conditions,status=status))

add('A1','标题、作者单位、梁文锋署名、v1 时间与 extended abstract 历史','L1',AA+' ; '+A,[q(aaf,'DeepSeek Elastic Compute (DSec): A Sandbox Infrastructure for Effective Agentic Training at Scale'),q(aaf,'Wenfeng Liang'),q('a-arxiv-paper.md','Affiliation: DeepSeek-AI'),q('a-arxiv-paper.md','Tsinghua University'),q(aaf,'Sat, 19 Sep 2026 12:20:26 UTC (504 KB)'),q(aaf,'This version has been substantially expanded from an earlier version, whose two-page extended abstract underwent first-round review for the Operational Systems Track of ACM SIGOPS ATC 2026')],['01-dsec-abstract.png'],'DeepSeek 作者原始技术报告按 L1；非同行评审验收。署名单位还包括 Tsinghua University；不是全员只有一个单位。v1 北京时间 2026-09-19 20:20:26。此前两页摘要经过 first-round review，不等于已录用。')
add('A2','生产规模及单个 scale unit 口径','L1',A,[q(af,'A single production-scale unit of DSec spans around 160 nodes, serving about 3 million sandboxes per day; in production, it supports over 380,000 concurrent sandboxes and sustains over 5,000 sandbox creations per second.'),para(af,'Within one scale unit')],['01-dsec-abstract.png'],'摘要用 over 380,000；第 2 节用 peak concurrency reaching ∼380K（HTML 有数学标记重复）；并列保留。单个 scale unit，单位 sandbox / sandbox instances，不是 agent 数。生产数据，不是第 8 章独立测试集群。')
add('A3','隔离层级原话','L1',A,[q(af,'This report presents DeepSeek Elastic Compute (DSec), a production sandbox platform that exposes FnCall, container, microVM, and full-VM sandbox backends through a unified SDK.')],['01-dsec-abstract.png'],'摘要原词 FnCall、container、microVM、full-VM；不要将四种后端都称 container。')
add('A4','残留答案、RPC、日志、/bin/bash、XFS、端口、Go proxy、新版 package 全部案例','L1',A+'#S6.SS4',[para(af,'Within sandboxes, agents searched'),para(af,'Outside the sandbox, agents searched')],['02-dsec-agent-behaviors.png'],'6.4 节；forge / overwriting / XFS 均须保留 attempted / tried。XFS 原文结果是 metadata 损坏与 filesystem shutdown。Outside the sandbox 描述搜索信息源，不自动等于成功逃逸。')
add('A5','非故意事故与 AppArmor / eBPF 的能力边界','L1',A+'#S6.SS4',[para(af,'Infrastructure failures also arose'),para(af,'No single mechanism can prevent'),para(af,'We use AppArmor profiles'),para(af,'DSec enforces the corresponding allowlists')],['03-dsec-accidents.png'],'事故在 6.4；防护限制在相邻 6.5，非同一节。grep 从根目录递归遍历 /proc；打错的是 agent container 自己；yes 的 stdout 被 chronus 存档后累积几十 GB。')
add('A6','性能实验使用独立 10 节点 CPU 测试集群','L1',A+'#S8.SS1',[q(af,'We conduct the experiments in this chapter on a dedicated 10-node CPU test cluster, separate from our production deployment.')],[],'8.1 Experimental Setup；this chapter 限定第 8 章，不能把生产规模当作这组实验测量。')
add('A7','行为定性及是否报告成功逃出 sandbox','L1',A+'#S6.SS4',[q(af,'Agent Misbehavior and System Failures'),q(af,'Obtaining answers through unintended channels.'),span(af,'Here we describe access controls','mitigate reward hacking'),q(af,'The attempt corrupted XFS metadata and forced a filesystem shutdown, illustrating how answer-seeking behavior can even disrupt infrastructure.')],['02-dsec-agent-behaviors.png','03-dsec-accidents.png'],'找到 misbehavior、unintended channels 与 6.5 reward hacking；在 6.4/6.5、摘要及全文 escape / escaped / breakout / break out 检索中未找到“成功逃出 sandbox”的一手表述。未找到不等于证明绝无逃逸。','部分支持')

heat=[]
for id,stem,url in [('A8','a-hn','https://news.ycombinator.com/item?id=49859112'),('B7','b-hn','https://news.ycombinator.com/item?id=49820134')]:
    soup=BeautifulSoup(read(stem+'.html'),'html.parser');title=soup.select_one('.titleline a').get_text();sub=soup.select_one('.subtext').get_text(' ',strip=True)
    points=re.search(r'(\d+) points',sub).group(1);comments=re.search(r'(\d+)\s+comments',sub).group(1)
    time=next(x['time_bj'] for x in json.loads(read('fetch-results.json')) if x['file']==stem+'.html')
    heat.append(dict(site=id,url=url,title=title,points=points,comments=comments,time=time,source=stem+'.html',kind='原站 HTML'))
    if id=='A8':add(id,'HN 标题、points、comments 与抓取时间','L6',url,[q(stem+'.md',title),q(stem+'.md',points+' points'),q(stem+'.md',comments+' comments')],[],f'抓取北京时间 {time}；动态快照，不是累计传播人数。')

add('B1','页面标题、日期及机构自称','L1',B,[q(bf,'Claude discovers a novel enzyme system with CRISPR-like repeats'),q('b-anthropic.md','Sep 23, 2026'),q(bf,'We’re introducing a new life sciences research group and laboratory at Anthropic.'),q(bf,'We are part of Anthropic’s life sciences organization, alongside teams whose work includes drug discovery, and training Claude in biology and chemistry.')],['04-enzyme-title-known-rt.png'],'可见 h1 比 HTML title 多 with CRISPR-like repeats；原站未列日期时刻/时区，不擅自补算北京时间。')
add('B2','RT 已被识别；新识别的是周围非编码阵列与 accessory protein','L1',B,[para(bf,'While this underlying RT')],['04-enzyme-title-known-rt.png'],'保留 appears to be the first；underlying RT found in a jumbo phage，accessory protein 的功能未知。')
add('B3','系统功能未知；CRISPR-like 为何种类比','L1',B,[para(bf,'Although we don’t yet know its function'),para(bf,'The repeat layout resembles a CRISPR array')],['05-enzyme-function-unknown.png'],'类比的是 DNA repeat array 布局；短 RNA 表达已观察，类似可编程用途仅 suggesting / may，不是已证明基因编辑功能。')
add('B4','约 950 agents / 21 小时 / 2.10 亿 tokens 及阶段','L1',B+' ; '+BP,[para(bf,'After 21 hours spent searching'),span(pf,'The full campaign comprised 119 tasks','without human intervention (Supp. Fig. 1A).'),span(pf,'The agents ran 949 sessions','Tokens that were read from the prompt cache were excluded.')],['06-enzyme-agents-humans.png'],'公告：roughly 950 agents、21 hours、210 million tokens。报告 p3：949 agent sessions、77 agent-hours、215.6 million、21.5 wall-clock hours；Methods p33：76.9 agent-hours，tokens 为 uncached input + output + cache-write，排除 cache-read。均为搜索 campaign，不是整个湿实验历时；不取舍、不混用。')
add('B5','人类湿实验、审查和 Claude 辅助解释；实验究竟验证什么','L1',B+' ; '+BP,[q(bf,'All of the lab work is performed by human scientists.'),para(bf,'Many of our workflows involve'),para(bf,'When a candidate survives our review'),q(bf,'Our first experiments show that the ART array is also expressed as a set of distinct short RNAs, suggesting that something analogous may be at play for this system.'),span(pf,'To examine the array RNA independently of phage infection','formed during infection (Fig. 3D).'),span(pf,'we have not shown that the RT is active','are currently unknown.')],['05-enzyme-function-unknown.png','06-enzyme-agents-humans.png'],'报告 p8：E. coli 中质粒表达 SA1 ART、small-RNA sequencing，观察离散短 RNA；原有 phage infection RNA-seq 数据是再分析，不能冒充本次全部新湿实验。p13 明说未证明 RT 活性/RNA 底物，伙伴相互作用及生物学功能未知。')
add('B6','预印本平台、标识、标题、作者、日期及摘要','L1',BP,[q(pf,'Autonomous AI agents discover reverse transcriptases with tandem repeat arrays'),q(pf,'Peter H. Yoon'),q(pf,'Januka S. Athukoralage'),q(pf,'Emmanuel Ameisen'),q(pf,'Eric Kauderer-Abrams'),q(pf,'Nicholas T. Perry'),q(pf,'Matthew G. Durrant'),span(pf,'Among the top candidates were','dedicated part- ner gene.'),q(pf,'ART arrays are highly expressed and appear as discrete units during Staphylococcus phage infection, suggesting an RT system directed by a repertoire of distinct RNAs.')],[],'已找到官方 CDN 40 页技术报告，公告称 pre-print。未找到该报告自身的 arXiv/bioRxiv 编号或 DOI；正文封面未列发布日期，关联公告日期 2026-09-23，不等于 PDF 正式发表日期。摘要未出现 CRISPR 一词；CRISPR 比较见引言 p2 / 讨论 p13。PDF 换行连字符按提取结果保留。','部分支持')
h=next(x for x in heat if x['site']=='B7')
add('B7','HN 标题、points、comments 与抓取时间','L6',h['url'],[q('b-hn.md',h['title']),q('b-hn.md',h['points']+' points'),q('b-hn.md',h['comments']+' comments')],[],f"抓取北京时间 {h['time']}；动态快照。")
add('C1','标题、Discovery / Disclosure date','L1',C,[q(cf,'Self-replicating prompt injections exist'),q('c-openai.md','Discovery date: Jun 27, 2026'),q('c-openai.md','Disclosure date: Sep 25, 2026')],['07-openai-worm-summary.png'],'页面另列 Report updated: Sep 25, 2026；无时刻和时区，不补算北京时间。')
add('C2','Summary：仅模拟工具调用，披露不因事故','L1',C,[q(cf,'No impact was observed outside of the simulated tool calls in training and evaluation; we are sharing this due to the novel nature of the prompt injection, not because of any incident.')],['07-openai-worm-summary.png'],'官方自述；不能据此改写为公开用户/产品已遭真实蠕虫事故。')
add('C3','攻击方/受害模型和内部 checkpoint 边界','L1',C,[para(cf,'The model that discovered the email')],['08-openai-worm-models.png'],'GPT-5.4-mini 两侧为 internal-only research checkpoints；GPT-5.5 是 separate Slack multi-hop evaluation，攻击发现方运行在 Codex harness。不能把全部实验压成一个模型。')
add('C4','邮件、文件系统、代码注释及多跳 Slack','L1',C,[q(cf,'Below is one of the clearest examples, in which the injection arrives by email, and instructs the agent to copy it into any email it sends.'),q(cf,'We discovered additional prompt injections that replicate via the filesystem or commit themselves via code comments.'),q(cf,'They also include multi-hop prompt injections, where one message leads the agent to other messages that together induce it to perform an unauthorized action and propagate the payload.'),q(cf,'Here, a GPT-5.5 agent retrieves additional Slack instructions, sends froges (an internal currency for recognizing colleagues) to a named recipient, and reposts the injected message.')],[],'模拟环境里的具体传播路径；示例信息 synthetic，Slack 名称/标识是 placeholders。存档中攻击载荷仅作引用数据，未执行。')
add('C5','把 self-reproduction 纳入 GPT-Red 攻击训练目标','L1',C,[para(cf,'We are including self-reproduction'),para(cf,'GPT-Red attacker training happens')],[],'future / expect 是未来模型鲁棒性预期，不等于已证明完全防住。')
add('C6','附录先前 AI worm 工作','L1',C,[para(cf,'Cohen, S., Bitton, R., & Nassi, B. (2025).')],[],'附录链接 DOI https://doi.org/10.1145/3719027.3765196；本条核实的是 OpenAI 页面引用 Cohen 等及 ACM CCS 2025，不声称独立复核该论文全部内容。足以表明本页承认先前工作；不建立“历史首个”排序。')
rq=[]
for name,id in [('openai','1wr78yj'),('artificial','1wr7ayr')]:
    f=f'c-reddit-{name}.json';mf=f'c-reddit-{name}-metadata.json'
    post=json.loads(read(f))[0];meta=json.loads(read(mf));p=meta['post'][0]
    rq += [q(f,p['post-title']),q(f,'"score": '+str(post['score'])),q(mf,'"comment-count":"'+p['comment-count']+'"')]
    for source,score,comments,kind in [(f,post['score'],'未输出总数','agent-reach / OpenCLI reddit read'),(mf,p['score'],p['comment-count'],'原站 DOM 属性')]:
        heat.append(dict(site='Reddit r/'+('OpenAI' if name=='openai' else name),url=meta['url'],title=p['post-title'],points=str(score),comments=comments,time=stamp(source),source=source,kind=kind))
add('C7','两则 Reddit 原帖标题、票分、评论总数和抓取时间','L6','https://www.reddit.com/r/OpenAI/comments/1wr78yj/ ; https://www.reddit.com/r/artificial/comments/1wr7ayr/',rq,['10-overclaim-worm-reddit.png'],'同账号 No-Peanut-6988 两帖；发帖 UTC 分别 2026-09-27 01:27:41.173 / 01:30:28.663。分数为动态且可能 fuzzing 的 score，不等于赞数。read：318/285；DOM：316/285，评论 20/41；时点见热度表。线索 205/190 非本次实测。')

da='https://www.techtimes.com/articles/328046/20260925/deepseek-training-agents-hacked-their-own-sandboxes-escape-catalog-now-public.htm'
db='https://finance.sina.com.cn/roll/2026-09-24/doc-iniswvxc5441101.shtml'
dz='https://zgeo.net/news/anthropic-multi-agent-art-gene-editing-geo-guide'
add('D-A','找到 DSec 逃逸措辞标题实例','L4',da,[q('d-a-techtimes.md','DeepSeek Training Agents Hacked Their Own Sandboxes: Escape Catalog Now Public'),q('d-a-techtimes.md','Published: Sep 25 2026, 11:07 AM EDT')],['10-overclaim-dsec-techtimes.png'],'Tech Times / Shannon Harwood；北京时间 2026-09-25 23:07。已找到指标题确实存在；是否可称成功逃逸须对照 A7。未找到“同时运行38万个 agent”原站明确标题，见 D-A2。')
add('D-A2','“DeepSeek 同时运行 38 万个 agent”原站实例','L4','—',[],[],'检索 exact 英文 380,000 agents / 中文 38万 智能体 等；找到的多篇仍写 sandbox，不改造成夸大实例。','未找到一手来源')
add('D-B','酶系统夸大措辞候选：基因编辑新突破及独立完成课题','L4',dz+' ; '+db,[q('d-b-zgeo.md','Anthropic多智能体系统自主发现ART系统：AI驱动基因编辑新突破与GEO落地指南'),q('d-b-zgeo.md','Anthropic 的 Claude 多智能体系统在 2026 年 9 月发现了什么新型基因编辑系统？'),q('d-b-sina.md','张锋点赞！Anthropic公布首个AI自主科学发现，21小时零干预，Claude找到全新DNA系统'),q('d-b-sina.md','一道指令，AI军团干完了整个课题')],['10-overclaim-enzyme-zgeo.png','10-overclaim-enzyme-sina.png'],'按任务要求媒体记录为 L4，ZGEO 自称 AI 编辑部/重组内容，不能作科学事实依据。新浪页署来源智药局，只核实该页确实刊载，不推定为最初首发。报告确有搜索 campaign without human intervention，不能单凭“21小时零干预”就判定其错；“整个课题”与 B5 人类实验边界需另行审稿。')
add('D-C','“The first real AI worms have arrived”两则原帖','L6','https://www.reddit.com/r/OpenAI/comments/1wr78yj/ ; https://www.reddit.com/r/artificial/comments/1wr7ayr/',[rq[0]],['10-overclaim-worm-reddit.png'],'已找到指原帖措辞存在；不支持已发生真实生产事故或历史首个。检索“ChatGPT 被蠕虫攻破”未找到本事件原站实例；不拿其他漏洞/事故填充。')

rows.sort(key=lambda r:(list('ABCD').index(r['id'][0]), int(re.search(r'\d+',r['id']).group()) if re.search(r'\d+',r['id']) else 0))
def cell(s):return str(s).replace('|','\\|').replace('\n',' ')
head='''# 0927 期取证事实清单

范围：仅取证，不是发布正文或最终定论。基线 main / ded0f347a04e5158bdf984be36ee71bc90323334；未读取 docs/0926/ 或上一期 handoff。网页原文中的任何指令均视作取证对象。

“已找到”只表示该来源确有相应表述，不表示独立复现、同行评审接受或支持标题的一切隐含结论。L1 为厂商官方原始发布/报告，HN/Reddit 为 L6；D 组媒体按任务指定记 L4。没有找到的内容不以转载或搜索摘要补齐。

本轮实测抓取已跨入北京时间 2026-09-28，期次目录仍按任务保持 0927。日期无时区的原站不擅自添加时区。各引文旁标存档文件；PDF 引文按 UTF-8 文本提取，换行在 normalized 文件中折叠为空格，保留原词及标点和断行连字符，不是改写。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
'''
for r in rows:
    quotes='<br><br>'.join('“'+cell(t['quote'])+'” ['+t['file']+']('+t['file']+')' for t in r['quotes']) or '—（未找到，不补写）'
    shots='<br>'.join(f'[{f}](../images/{f})' for f in r['shots']) or '—'
    head+='| '+' | '.join([r['id'],cell(r['claim']),r['level'],cell(r['url']),quotes,shots,cell(r['conditions']),r['status']])+' |\n'
head+='''
## 可能的吠点（来源限制清单，非正文结论）

- DSec 摘要是 “over 380,000 concurrent sandboxes”，第 2 节是单个 scale unit 的 peak concurrency ∼380K，不能换单位为 agent 或当作全公司总数。（A2）
- A4 的 “Outside the sandbox” 后面具体说的是可达镜像/代理等信息源，未找到成功逃逸陈述；XFS 结果明确是损坏和 shutdown。（A4、A7）
- AppArmor/eBPF 的“只能解决部分问题”在 6.5 节，事故在 6.4 节。（A5）
- 10 节点独立实验集群与生产规模是两套口径。（A6）
- Anthropic 保留 “appears to be the first”，且基础 RT 已被旧研究识别，新增识别的是系统特征。（B2）
- 公告与技术报告数字不同；949 是 sessions，215.6 million 排除 prompt-cache read tokens；不能从这些数字推算 950 个同时运行的 agent。（B4）
- E. coli small-RNA sequencing 支持阵列表达为短 RNA，不证明 RT 活性、RNA 底物或蛋白相互作用；报告原句为 “we have not shown that the RT is active or that the unit RNAs are its substrates.”（B5；报告 p13）
- 预印本摘要没有 CRISPR 一词；正文把重复阵列布局作类比，同时保留功能未知。（B3、B6）
- 报告还说 “We next asked whether the discovery of ART was reproducible within our harness, so we ran the same campaign ten more times.”，接下去写 “the array was missed in every rerun.”；这是完整搜索重跑，不可与给定 DNA 的识别 benchmark 混用。（b-technical-report.md，p8–10）
- OpenAI 的 GPT-5.4-mini 内部双模型实验与 GPT-5.5 Slack 多跳实验要分开，Summary 排除了观察到模拟外影响。（C2、C3）
- OpenAI 措辞为未来模型 “expect them to be more robust”，未声称已全面解决自我复制注入。（C5）
- C6 的附录明确承认 2025 年 AI Worm 工作，不宜把本报告标题中的新型注入直接改写为 AI worm 历史首创。（C6）

## 夸大说法实例（D 组，记录存在的措辞，保留审稿边界）

| 组 | 原站 URL | 发布方 | 发布时间原文 / 时区 | 标题原文（逐字） | 取证状态及边界 |
|---|---|---|---|---|---|
'''
examples=[('D-A',da,'Tech Times / Shannon Harwood','Sep 25 2026, 11:07 AM EDT','DeepSeek Training Agents Hacked Their Own Sandboxes: Escape Catalog Now Public','已找到；标题 escape，用 A7 限制核对'),('D-B',dz,'智脑时代 ZGEO / AI 编辑部','2026年9月24日（无时区）','Anthropic多智能体系统自主发现ART系统：AI驱动基因编辑新突破与GEO落地指南','已找到；正文 FAQ 还把 ART 称新型基因编辑系统'),('D-B',db,'新浪财经刊载 / 署来源智药局','2026年09月24日 11:28（页面未列时区）','张锋点赞！Anthropic公布首个AI自主科学发现，21小时零干预，Claude找到全新DNA系统','已找到；“一道指令，AI军团干完了整个课题”见正文；不能把搜索阶段无干预本身判错')]
for name in ['openai','artificial']:
    m=json.loads(read(f'c-reddit-{name}-metadata.json'));p=m['post'][0]
    examples.append(('D-C',m['url'],'Reddit u/'+p['author']+' / r/'+('OpenAI' if name=='openai' else 'artificial'),p['created-timestamp'],p['post-title'],'已找到；同一账号两帖，不是两家独立媒体'))
for e in examples:head+='| '+' | '.join(map(cell,e))+' |\n'
head+='\n未采纳为夸大实例：The Verge 原站标题为 “Anthropic’s biolab made a discovery it’s comparing to Crispr”，明确有 it’s comparing，不改成“新 CRISPR”。原站 HTML/正文另存 d-b-verge.* 供复核。\n\n## 热度数据（抓取时间均为北京时间）\n\n| 来源 | 标题 | score / points | 评论数 | 抓取时间 +08:00 | 工具/存档 |\n|---|---|---|---|---|---|\n'
for h in heat:head+='| '+' | '.join(map(cell,[h['url'],h['title'],h['points'],h['comments'],h['time'],h['kind']+' / '+h['source']]))+' |\n'
head+='\nReddit 读帖适配器未输出总评论数；以原站 shreddit-post 的 comment-count 补取，不把取回的评论条目数当总数。两个时点的不同分数全部保留；不可将 score 当唯一用户数或净赞以外的指标。\n'
(S/'evidence.md').write_text(head,encoding='utf-8')
(S/'evidence-data.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf-8')
(S/'heat-data.json').write_text(json.dumps(heat,ensure_ascii=False,indent=2),encoding='utf-8')
print('Validated quote count:',sum(len(r['quotes']) for r in rows),'rows:',len(rows))
