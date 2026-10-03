"""Build the evidence index from public-source captures; does not fetch or publish."""
import csv, json, pathlib, re, datetime, hashlib
from bs4 import BeautifulSoup
P=pathlib.Path(__file__).parent
def read(n): return (P/n).read_text(encoding='utf-8')
def norm(s): return re.sub(r'\s+', ' ', s).strip()
def line(n, needle):
    return next((norm(x) for x in read(n).splitlines() if needle in x), '【未提取到：请检查存档】')
def para(n, needle):
    return next((norm(x) for x in read(n).split('\n\n') if needle in x), '【未提取到：请检查存档】')
def excerpt(n, start, end):
    s=read(n); a=s.index(start); b=s.index(end,a); return norm(s[a:b])
def img(n):
    f=next((P.parent/'images').glob(f'{n:02d}-*.png')); return f'[{f.name}](../images/{f.name})'
def link(u): return f'<{u}>'
META='https://research.meta.ai/blog/solving-open-research-problems-together'
DAY='https://arxiv.org/abs/2610.01306'
GR='https://www.tavus.io/griffin'
VF='https://research.nvidia.com/labs/amri/projects/video-fdb/'
TX='https://x.com/tavus/status/2105704169009246248'
MX='https://x.com/AIatMeta/status/2106099776035152231'
rows=[]
def row(code,claim,level,url,quote,shots,conditions,status):
    cells=[code+' '+claim,level,url,quote,shots,conditions,status]
    rows.append('| '+' | '.join(str(x).replace('|','\\|').replace('\n','<br>') for x in cells)+' |')
aq=lambda k:line('a-meta-blog.txt',k)
cq=lambda k:line('c-griffin.txt',k)
row('A1','博文标题、日期、模型与四条原则','L1',link(META),'Solving Open Research Problems Together；October 2, 2026；'+ '<br>'.join(aq(k) for k in ['They used both','A team of mathematicians','A second group','Each paper clearly','Each paper gives','Five present','After completing']),img(1),'官方仅日期，无时刻/时区。引文保留原语言；空白合并。','已找到')
row('A2','六篇原件、作者、日期及 AI/Human 标注','L1','见下方 A2 六篇原件表','margin markers identify human-drafted (Human) and AI-assisted material (AI) separately.','；'.join(img(n) for n in range(8,14)),'六个官方摘要页和六个官方 PDF 已存档。官方发布日期均 2026-10-02；未找到可核对的六篇 arXiv 提交历史，不能把发布日期写成提交日期。','部分支持')
row('A3','概率篇三组并行工作与严格阈值边界','L1',link(META),aq('We also acknowledge three independent'),img(2),'三个 arXiv 原始摘要页及 v1 时刻见 A7；exactly at the threshold remains unresolved。','已找到')
row('A4','PDE 人类选题和思路、2015 问题','L1',link(META)+'；https://arxiv.org/abs/1503.01741',aq('while Dinh chose'),img(3),'旧论文 v2 PDF p.8 原文见原始问题表；二维及以上、径向、负能量等条件不能删。','已找到')
row('A5','384 阶、GAP 与 Nilradical','L1',link(META)+'；https://nilradical.ai/results/kourovka-21-68/',aq('Muse Spark generated the search'),img(4),'Nilradical 自述及固定 commit 原件已存档；其反例为 2592 阶，运营归属见 A7。未重跑 GAP/Lean。','已找到')
row('A6','Hu–Wen 并行工作；算术物理起草及 review 名单','L1',link(META)+'；https://arxiv.org/abs/2609.25023',aq('We also acknowledge independent work by Hu')+'<br>'+aq('drafted three core technical'),img(6)+'；'+img(7),'Hu–Wen v1 日期见 A7。算术物理作者行无 Review by，其他五项有；不能据此断言无人 review。','已找到')
row('A7','并行工作的公开记录时间线','L1','见下方 A7 时间线','以各 arXiv Submission history、Nilradical 项目记录及固定 Git commit 为准。','—','仅日期与来源；commit 时间不等于首次公开可见时间；不评判先后贡献。','已找到')
row('A8','Meta 热度及数学圈讨论','L1/L5/L6',link(MX)+'；https://aihot.news/；https://news.ycombinator.com/item?id=49942159','AIHOT 页面标签：AI评分；Meta 项目显示 73。','—','Meta X 显示 264K views、955 likes、153 reposts；抓取北京时间见热度表。HN 两帖各 1 分。Threads、Tao/mathstodon 未找到可核对的目标帖。','部分支持')
row('A9','实际传播中的“独立/首次/六大问题”措辞','L2/L5/L6','见下方传播实例表','毕树超官宣：Meta AI连破6大世界猜想，数学界AlphaGo时刻来了！','—','已存原站传播页面，但该新浪页面自标来源新智元，未回溯其原发页；未找到符合要求的逐字“AI 独立解决 5 个数学难题”实例。','部分支持')
babs=para('b-dayjob-html.txt','Professional work often starts')
row('B1','摘要与 submission history','L1',link(DAY),babs,img(14),'v1 Thu, 1 Oct 2026 08:39:10 UTC＝北京时间 2026-10-01 16:39:10。','已找到')
row('B2','主结果全表、30 配置和软阈值','L1','https://arxiv.org/html/2610.01306v1；https://arxiv.org/pdf/2610.01306','Table 3；strict pass@1；见下方逐行抄录。',img(15),'PDF p.6 全页保留表头/effort/全部 30 行/表注；另存原始 ancillary CSV，软阈值见表。','已找到')
row('B3','pass、criteria、judge、尝试与一致率','L1',link(DAY),'The headline metric is strict pass@1, τ = 1.；We evaluate 30 configurations from 13 developers with five attempts per task.','—','PDF §§4.2–4.4 p.5：每题 5 次；错误尝试排除；judge Claude Opus 4.8/OpenCode 1.18.31/rewardkit 0.1.7；criteria 中位数 47.5/57.5。未找到本研究 judge 对人工一致率。','部分支持')
row('B4','13.6/16.6 小时如何得到','L1',link(DAY),'The tasks are estimated to take a professional 13.6 hours on average in healthcare and 16.6 in finance.','—','PDF §3.4 p.4：医疗由任务作者估时分箱，中点求平均，>40 取 40；金融 16.6 来自 dataset card，未给同等详细估法。不是已测人类耗时。','部分支持')
row('B5','错误输入贯穿分析的案例','L1',link(DAY),'见下方 §6 原文摘录。',img(16),'PDF p.8；论文作者个案描述，不是本次重测。','已找到')
row('B6','作者、机构、任务来源及公开资源','L1',link(DAY)+'；https://github.com/surge-ai/dayjob','We release all healthcare tasks, 50 of the 80 finance tasks, the evaluation harness, and the leaderboard.','—','15 位作者均列 Surge AI；公开医疗 50、金融 50/80；harness Apache-2.0，数据 MIT。专业人员人数未披露；未作全作者任职背景审计。','部分支持')
row('B7','DAYJOB 热度及“24% 工作”实例','L5/L6','https://aihot.news/all?q=DAYJOB&tab=relevance；https://news.ycombinator.com/','未找到可逐字引用且符合指定含义的“AI 只能做 24% 的工作”原站文章。','—','AIHOT 两种搜索为空；HN/X 有限检索未取得可归属的有效热度；不等于全网无人讨论。','未找到一手来源')
row('C1','Tavus 研究方法、结果、对照及可用性','L1',link(GR),'<br>'.join(cq(k) for k in ['Participants were told','26 of 54','On Phoenix-4.5','Griffin-Lite will not']), '；'.join(img(n) for n in [17,18,19]),'一分种视频通话，54 人，26 人；Phoenix-4.5 n=41。公司自述；问题原始问卷、平台名称、CI、显著性检验未披露。详见方法节。','部分支持')
row('C2','VideoFDB 两表、方法、日期、作者','L3',link(VF)+'；https://arxiv.org/abs/2605.30256',line('c-videofdb.txt','Current models remain')+'<br>'+line('c-videofdb.txt','No evaluated')+'<br>'+line('c-videofdb.txt','Three rubric axes'),img(20)+'；'+img(21),'当前网页两表均截全；PDF v1 北京 2026-05-29；当前网页 Griffin 行不能倒推已存在于 v1。名次按 Overall。','已找到')
row('C3','Tavus X 互动、Community Note 与 Protos','L1/L5',link(TX)+'；https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/','Yesterday, in an X post that garnered more than 13 million views, Tavus revealed footage of Griffin realistically talking to company founder, Hassaan Raza.',img(22),'原帖已读取；当前约 1983 万。Note 显示“读者已提议补充背景信息／正在收集评分”，不能写成已获认可的正式 Note。','部分支持')
row('C4','Turing 1950 imitation game 原始设置','L1','https://academic.oup.com/mind/article/LIX/236/433/986238','未取得可存档核对的机器替换段原文，不凭记忆补引文。','—','OUP HTTP 403/Cloudflare；Turing Archive 入口亦未成功读取；未使用转载或镜像绕过。','未找到一手来源')
row('C5','中英文图灵测试传播实例','L5/L6','见下方传播实例表','AI数字人工具横评2026：Tavus 通过视频图灵测试，HeyGen/Synthesia/D-ID 对比','—','中文标题有无条件“通过”；英文 Tech-ish 标题限定视频通话参与者且正文列限制，不能自动当成夸大实例。','部分支持')

out=['# 1003 期取证清单','',
'仅取证，不是发布稿或数学证明验收。抓取于北京时间 2026-10-03；动态计数以各快照为准。官方只给日期的，原样保留并注明未给时刻/时区，不擅自换日。英文引文仅合并网页换行和空白；PDF 抽取保留原文件，跨行断词及公式以 PDF 为准。L1–L7 按 EDITORIAL.md。','',
'| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |',
'|---|---|---|---|---|---|---|',*rows,'',
'## A2：六篇原件与标注','',
'六个官方摘要页均写 October 02, 2026，官方未给时刻/时区或提交历史；本次没有找到这六篇的 arXiv 版本。下载入口及实际官方 CDN 原件 URL 见 [a-paper-downloads.json](a-paper-downloads.json)，签名参数属于下载所需参数，不是 utm 跟踪参数。','',
'| 篇目 | 作者（PDF） | 官方原件入口／本地原件 | 标注示例（PDF 页码） | 页面列出的 review |',
'|---|---|---|---|---|']
slugs=['the-strict-threshold-for-gaussian-ellipsoid-fitting','finite-time-blow-up-of-radial-negative-energy-solutions-for-the-mass-critical-biharmonic-nonlinear-schrodinger-equation','semiabelian-groups-need-not-be-monomial','tightness-of-the-cycle-based-relaxation-for-completed-length-three-alpha-cycles','string-two-point-function-height-function-on-a-curve','on-solvable-evolution-algebras-and-a-conjecture-by-garcia-martinez-and-perez-rodriguez']
titles=['The Strict Threshold for Gaussian Ellipsoid Fitting','Finite-Time Blow-Up of Radial Negative-Energy Solutions for the Mass-Critical Biharmonic Nonlinear Schrödinger Equation','Semiabelian Groups Need Not Be Monomial','Tightness of the Cycle-Based Relaxation for Completed Length-Three Alpha-Cycles','String Two-Point Function = Height Function on a Curve','On Solvable Evolution Algebras and a Conjecture by García-Martínez and Pérez-Rodríguez']
authors=['Aykut Arslan','Leonard Dinh','Joseph Phillip Brennan; Milana Golich','Aykut Arslan','Anindya Dey; Gabriel Herczeg; An Huang; Nicolas Jaramillo Torres; Jacob H. Swenberg','Andres Barei（官方摘要 Written by 为 Andres Barei Bueno）']
reviews=['Babak Modami; Alexander Roitershtein; Mark Sepanski; Grigory Sokolov','Fazel Hadadifard; Salem Selim','Andres Barei; John Portin','Kien Trung Le','未单列 reviewer；致谢反馈不能自动补成正式 reviewer','Nicolás Jaramillo Torres']
for i,(slug,title,author,review,page) in enumerate(zip(slugs,titles,authors,reviews,[5,1,4,3,2,2]),1):
    out.append(f'| {i}. {title} | {author} | [官方摘要](https://ai.meta.com/research/publications/{slug}/)；[PDF](a-paper-{i}-full.pdf)；[摘要存档](a-paper-{i}.html) | p.{page}，页边 Human / AI；{img(i+7)} | {review} |')
out+=['','六篇 Statement of AI Use 均说明：','', '> This paper was developed through collaboration between researchers and Muse Spark (AI model) via the meta.ai chat interface. The model helped explore ideas, develop candidate arguments, and draft material. Researchers guided the work, checked and corrected the mathematics, and take ownership for the final manuscript. Following [Sch25], margin markers identify human-drafted (Human) and AI-assisted material (AI) separately.','','AI 标记的定义是 **AI-assisted material**，不能直接换成“未经人类修改的 AI 原稿”。每篇的并行工作原句如下（从 PDF 抽取，断词保留；完整比较段见各 PDF）：','']
for i in range(1,7):
    s=read(f'a-paper-{i}-full.txt'); m=re.search(r'Concurrent [Ww]ork',s); assert m
    tail=s[m.end():]; blocks=[b for b in re.split(r'\n\s*\n',tail) if b.strip()]
    q=norm(blocks[0]); page=s[:m.start()].count('\f')+1
    out += [f'**篇 {i}，PDF p.{page}：**','', '> '+q,'']
out+=['## A7：日期时间线（不作优先权判断）','','| 记录 | 北京日期 | 原记录日期与时区 | 一手来源 |','|---|---|---|---|',
'| Misiakiewicz–Wen v1 | 2026-08-11 | 2026-08-10 UTC | https://arxiv.org/abs/2608.10184 |',
'| de la Cerda–Potechin–Tulsiani–Xu v1 | 2026-08-12 | 2026-08-12 UTC | https://arxiv.org/abs/2608.12415 |',
'| Hu–Wen v1 | 2026-08-12 | 2026-08-12 UTC | https://arxiv.org/abs/2609.25023 |',
'| Koehler–Sohn v1 | 2026-08-28 | 2026-08-27 UTC | https://arxiv.org/abs/2608.27372 |',
'| Nilradical statement accepted | 官方未给时区，不能换算 | 2026-09-16，未给时刻 | https://nilradical.ai/results/kourovka-21-68/ |',
'| Nilradical 固定 Git commit | 2026-09-16 | 2026-09-16 UTC | https://github.com/alunik/kourovka-lean/commit/5a6b2c18e326b7b0281f00b64cade629acbfe1f5 |',
'| Meta 六篇官方公开页面 | 官方未给时区，不能换算 | 2026-10-02，官方未给时刻 | 上方六个官方摘要页 |','',
'时间核对附记：上述四个 arXiv v1 依次为北京时间 08-11 03:55:11（08-10 19:55:11 UTC）、08-12 12:09:06（04:09:06 UTC）、08-12 21:38:05（13:38:05 UTC）、08-28 01:09:38（08-27 17:09:38 UTC）。Hu–Wen 的 2609 编号与页面所列 8 月 submission history 不同，照录页面，不用编号推算月份。Git commit 为北京时间 09-16 18:37:31（10:37:31 UTC），这不是首次公开可见性的证明。','',
'四篇作者：Theodor Misiakiewicz / Garrett G. Wen；Sofia de la Cerda / Aaron Potechin / Madhur Tulsiani / Jeff Xu；Xing-Yu Hu / Ran Wen；Frederic Koehler / Youngtak Sohn。Nilradical 网站标注 “A project by Aluna Rizzoli”；GitHub alunik 用户 API 与之吻合。其结果页反例阶数为 2592，Meta 为 384；此处不复核数学或最小性。','',
'## 原始问题与“五篇”的范围','','| 项目 | 已取得的出处与原句/位置 | 尚缺或限制 |','|---|---|---|',
'| 概率 | 新论文 p.2 追溯 Saunderson, Parrilo, Willsky [SPW13] 的 n=d²/4 猜想；原始参考文献见 PDF 文末 | 本次未另取得 2013 原文；不可把新文献转述当作独立核对原问题 |',
'| PDE | [Boulenger–Lenzmann](https://arxiv.org/abs/1503.01741)，2015 预印本 v2 p.8：“Furthermore, it seems natural to conjecture that finite-time blowup always occurs in the setting of Theorem 3, at least in sufficiently high dimensions.” | 2017 期刊年不是问题首次预印本年；新结果需保留径向、负能量等条件 |',
'| 群论 | Kida, On semiabelian groups，DOI 10.1515/jgth-2024-0010；Crossref published-online 2024-11-09；新论文 Conj.1.2 引 Kida Conj.1.3 | 原出版社正文 HTTP 202 空响应；2025 期刊引用与 2024 online 年份可并存。原问题原页未取得 |',
'| 优化 | [Del Pia–Khajavirad](https://arxiv.org/abs/2507.12831)，v2 §5.2 p.28：“We leave open the question of whether the generalized triangle inequalities, together with the complete edge relaxation, characterize the multilinear polytope of the support hypergraph of an α-cycle of length three.” | v1 2025-07-17、v2 2026-06-10；v1 未匹配到同句，不能据论文初版年份直接判“2026 提出”错误 |',
'| 非结合代数 | García-Martínez–Pérez-Rodríguez, A note on complete evolution algebras，DOI 10.1007/s00013-026-02251-0；Crossref online 2026-04-22；新论文 Conj.1.1：复数域有限维 evolution algebra 可解 iff 无非零幂等元 | 出版社返回 challenge 页，未取得原文猜想页。不是对所有域任意代数的陈述 |',
'| 算术物理 | 原件 Introduction：从 Tate curve 推广到 p-adic local field 上有 semistable reduction 的曲线；参考 [Man87] 是 1987 方向性工作 | 未找到官方逐项标明哪一篇不计入 five 的句子 |','',
'**待确认的映射：** 博文的概率、PDE、群论、优化、非结合代数五节分别使用 answers/disproves a question/conjecture；算术物理节描述推广和连接。由此可以提出“五篇＝前述五项”的候选解释，但官方没有显式给出排除名单，不能把这项推断当成已核实；算术物理原件也有自己的研究问题。','',
'## B2：DAYJOB 主结果表（PDF Table 3，p.6）','',
'原表表注和数值逐行从官方 HTML 表抄录；截图使用官方 PDF 整页。请保留 effort 括号；空白档位不补造。','']
tables=json.loads(read('b-dayjob-tables.json'))
t=next(x for x in tables if 'Table 3' in x['text'])
soup=BeautifulSoup(t['html'],'html.parser')
cap=soup.find('figcaption');out += ['> '+norm(cap.get_text(' ',strip=True)),'']
trs=[]
for tr in soup.select('tr'):
    vals=[norm(c.get_text(' ',strip=True)) for c in tr.find_all(['td','th'],recursive=False)]
    if len(vals)==6: trs.append(vals)
out+=['| Configuration / effort | Developer | Healthcare strict pass@1 % | Rank | Finance strict pass@1 % | Rank |','|---|---|---:|---:|---:|---:|']
for vals in trs:
    if vals[0] not in ['Configuration','Model']:out.append('| '+' | '.join(vals)+' |')
out += ['','### 原始 ancillary CSV：软阈值（均为通过率 %，不是平均完成工作比例）','','医疗与金融顺序为 H/F；N−1 允许最多一项失败；95%、90% 是单次尝试满足 criteria 比例的门槛。以下直接抄录原始 CSV 精度，可能比论文一位小数多。','',
'| Configuration | Strict H/F | N−1 H/F | ≥95% H/F | ≥90% H/F |','|---|---:|---:|---:|---:|']
hc=list(csv.DictReader(read('b-leaderboard_healthcare.csv').splitlines()));fc={r['configuration']:r for r in csv.DictReader(read('b-leaderboard_finance.csv').splitlines())}
for h in hc:
    f=fc[h['configuration']];out.append('| '+h['configuration']+' | '+' | '.join(h[k]+' / '+f[k] for k in ['strict_pct','n_minus_1_pct','pass95_pct','pass90_pct'])+' |')
out+=['','来源：[医疗 CSV](https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_healthcare.csv)、[金融 CSV](https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_finance.csv)。论文 §5.2 给 leader 的 90% 门槛 62.4%/58.3%（CSV 金融为 58.25），median 6.4%/10.4%；N−1 leader 37.8%/32.9%。没有找到所有 criteria 的平均满足比例或可直接解释为“完成了多少工作”的指标。','',
'## B3–B6：方法、来源与案例','',
'- §4.2 p.5：每模型配置每任务五次；provider rate-limit error 最多 rerun 三次；timeout/provider failure 等 error 从该题平均值排除。strict pass@1 是先题内尝试平均，再题间等权；不是五次里成功一次的 pass@5。','- §4.3 p.5：OpenCode 1.18.31 / rewardkit 0.1.7 / Claude Opus 4.8。judge 可以读文件、运行代码、读 trajectory；每项二元判断附证据；每 criterion 90 秒、至少 40 分钟。没有找到此 benchmark 自身的人工一致率实验。','- §4.4 p.5：criterion 不按重要性加权；所有项都满足才算 strict pass。医疗/金融中位 criterion 数 47.5/57.5。','- §4.1：OpenHands SDK 1.43.1、Harbor 0.22；最多 1000 turns、6 小时，每模型调用 600 秒；网络仅 provider API。这不是无约束代理工作环境。','- §3.4 p.4：医疗 task creator 从八个时间区间选档，用中点计算，>40h 截为 40h。[原始分箱 CSV](b-human_time_ranges.csv)；金融 16.6h 引 dataset card，未见与医疗同样详细的估计程序。','- §3.5：任务来自有一线相关工作经验的专业人员并经过多层审核；未披露独立专业人员总数。','- PDF p.1：Stephanie Finley, Liudas Panavas, Thomas Mikkelson, Cam Hinton, Stacey Ganss, Bradley Monton, Emily Kendall, Michelle Spradlin, Lydia Bye, Michael O’Brien, Lauren Ylvisaker, Derek Ray, Suhaas Garre, Sushant Mehta, Edwin Chen；统一机构 Surge AI。作者页未列 Anthropic 等被测厂商；不能据此排除一切过去/兼职关系。','- 数据：[healthcare](https://huggingface.co/datasets/surgeai/DAYJOB-healthcare)、[finance](https://huggingface.co/datasets/surgeai/DAYJOB-finance)，MIT；[harness](https://github.com/surge-ai/dayjob)，Apache-2.0；医疗 50 全公开，金融只公开 50/80，其余需请求。','',
'§6，PDF p.8 个案原文（仅据作者记录，未复跑）：','']
case=para('b-dayjob-html.txt','2,838')
out+=['> '+case,'',
'**另一官方口径差异：** [Surge 博客](https://surgehq.ai/blog/dayjob) 的 JSON-LD datePublished 是 2026-09-23（无时刻）；本次抓取正文写 finance 21.6h / healthcare 19.6h，与论文的 16.6h / 13.6h 不同。没有版本说明足以解释差异，两套数值均保留，不能混用。','',
'## C1：Tavus 方法细节与 Turing 用词全集','',
'页面日期 October 1st, 2026，官方未给时刻/时区。署名 Hassaan Raza、Ioannis Patras、Tavus Research Team。以下为页面方法段原文：','', '> '+cq('Participants were told'),'',
'已披露：一分种通话；谈论今年期待的事；独立研究平台招募但没有名称；先评自然、可信、交流体验，最后才问是否想过不是人，随后告知是 AI。页面未披露事先提示“可能是 AI”。Phoenix-4.5 是另一模型系统对照，n=41；未披露真人—真人对照。研究由 Tavus 以 we 叙述，未给独立执行机构或负责设计者名单。','',
'没有找到：判定问题的逐字问卷、随机化/盲法细节、招募平台名称、人口统计、显著性检验、置信区间。页面的 79%/81% 是受试者主观信心，不能当作 CI。七分量表 naturalness 5.4、trustworthiness 5.6、enjoyment 5.8、listening 5.5、flow 4.9，也不是真假分类准确率。','',
'页面正文含 Turing 的三段（逐段全文，搜索大小写不敏感）：','']
for l in read('c-griffin.txt').splitlines():
    if 'turing' in l.lower():out+=['> '+l,'']
out+=['## C2：VideoFDB 当前网页两张完整表','',
'作者：Amrita Mazumdar, Seonwook Park, Rajarshi Roy, Nikhil Srihari, Shengze Wang, Yuhao Zhou, Julia Wang, Koki Nagano, Shalini De Mello。页面机构 NVIDIA / David AI；Yuhao Zhou、Julia Wang 属 David AI。未列 Tavus 作者。PDF 致谢中未找到 Tavus；Tavus 自述 NVIDIA 为其评分，因此只能说未发现作者层面的 Tavus 署名，不能推成“无任何利益关系”。','',
'arXiv v1：北京时间 2026-05-29 01:20:01（2026-05-28 17:20:01 UTC）；v2：北京时间 2026-07-29 04:35:17（2026-07-28 20:35:17 UTC）。当前网页未给更新日期；Tavus 页称 2026 年 9 月评分。PDF 不含 Griffin/Tavus，不能用论文日期给新增榜单行定年。','']
for name,table in zip(['Perception','Generation'],json.loads(read('c-videofdb-tables.json'))):
    out+=['### '+name,'']
    soup=BeautifulSoup(table['html'],'html.parser'); rr=[]
    for tr in soup.select('tr'):
        v=[norm(c.get_text(' ',strip=True)) for c in tr.find_all(['td','th'],recursive=False)]
        if v:rr.append(v)
    width=max(map(len,rr)); head=next(v for v in rr if len(v)==width)
    out+=['| '+' | '.join(head)+' |','|'+'---|'*width]
    used=False
    for v in rr:
        if v==head and not used:used=True;continue
        if len(v)==1:v += ['—']*(width-1)
        out.append('| '+' | '.join(v)+' |')
    out+=['']
out+=['Perception Overall：Griffin 3.73 / Human 4.20；Generation Overall：Griffin 3.83 / Human 3.92。Griffin 在两张表按 Overall 均为 AI 模型第 1，含 human 则第 2；这不表示每个子项第一。Generation Affect 为 Griffin 4.40 / Human 4.14，不能只用 overall 代替分项。Timing 按页面同时报告比例与毫秒；不可只保留延迟。','',
'237 段真实双人视频通话、11 类动态；rubric-based LM-as-judge，0–5 分。77–89% within 1 point 是跨 LM judge 的一致性，不是人类真伪识别率或人工标注一致率。网页同时保留 “Current models remain well below human conversational naturalness.” / “No evaluated agent approaches the human reference on VideoFDB.”；这是页面措辞，不替作者对新增模型更新该结论。','',
'## 发布时刻与热度（动态快照）','',
'| 来源 | 北京时间（括号原时区） | 抓取量级/范围 |','|---|---|---|',
'| Meta X 官方 | 2026-10-03 03:11:30（10-02 19:11:30 UTC） | 卡片 264K views / 955 likes / 153 reposts / 64 replies / 308 bookmarks |',
'| Meta 博文及六篇 | 2026-10-02，官方未给时刻/时区 | 日期不能当成北京时间零点 |',
'| DAYJOB v1 | 2026-10-01 16:39:10（08:39:10 UTC） | 30 配置、13 developers |',
'| Tavus X 官方 | 2026-10-02 00:59:30（10-01 16:59:30 UTC） | 搜索快照 19,829,283 views / 38,984 likes；稍后卡片 1,983.4万 views / 38,983 likes / 9750 reposts / 3271 replies / 27595 bookmarks |',
'| Tavus 官页 | 2026-10-01，官方未给时刻/时区 | 不能由 X 时刻反推网页上线时刻 |',
'| Protos | 2026-10-02 21:31:45（13:31:45 UTC，article:published_time） | 可见页显示 2:31 PM，未给区名；采用带 offset 元数据 |',
'| AIHOT Meta | 见 d-aihot 快照及 capture-log 北京抓取时刻 | 博文项目 AI评分 73；另一社交项目 65；不是已证实“热度 75” |',
'| HN Meta | 见 d-hn-meta 快照 | item 49942159：1 分/1 评论；49937619：1 分/0 评论 |',
'| Reddit Griffin | 见 d-reddit 快照 | r/singularity 1wv7q40 搜索时 1327 分/372 评论；后读评论时 1323 分；标题写 44%，与原研究 48% 不同 |','',
'上述计数抓取时间精确到秒见 [capture-log.md](capture-log.md) 及相应 JSON；不同时间的数值不合并成一个虚假快照。Meta Threads 已开页面但没有取得可核对帖文；数学圈 Tao/mathstodon 未找到目标一手帖；DAYJOB 的 AIHOT/HN/X 有限检索未取得有效热度数字。','',
'### C3：Community Note 的准确状态','',
'> '+excerpt('c-tavus-x-card.json','The 48% figure','\\n\\ncellcog').replace('\\"','"'),'',
'卡片界面明确写“读者已提议补充背景信息”“正在收集评分”。这是可读到的提议 Note；没有取得 note ID 或最终 helpful 状态。它提及公司自身 n=54 一分种研究、未独立验证、非标准 protocol，也肯定 Griffin 在榜单领先；不要缩成“已证实造假”或“已被正式 Note 否定”。Protos 将之写为 community-noted 属媒体自己的表述。','',
'## 夸大说法实例（只存传播措辞，不代替事实裁决）','',
'| 项目/发布方 | 原站 URL | 原句逐字 | 时间（北京，原时区） | 限制 |','|---|---|---|---|',
'| Meta／新浪页面，文内标来源新智元 | https://k.sina.com.cn/article_5952915705_162d248f906703pv9i.html | 毕树超官宣：Meta AI连破6大世界猜想，数学界AlphaGo时刻来了！ | 2026-10-03 19:05:05（页面未标时区，按中国站点北京时区暂记） | 原站传播实例；不是新智元原发页；不能借它替代论文 |',
'| Meta／同上正文 | 同上 | 在过去短短6个月内，Meta的最新模型Muse Spark已经协助人类数学家连破6大数学领域的开放性难题。 | 同上 | 保留“协助”，不能转写成原文说“独立” |',
'| Meta／Alexandr Wang 公开帖 | https://x.com/i/status/2106149796121805099 | mathematicians and muse spark collaborated to solve 6 open problems in math: | 2026-10-03 06:30:16（10-02 22:30:16 UTC） | 搜索卡片 2550 likes/297326 views；账号个人发言，当前职衔未另核 |',
'| Meta／AI Primer | https://www.ai-primer.com/engineer/stories/meta-muse-assisted-math-results | Meta reports Muse Spark solutions to six open mathematics problems | 元数据 2026-10-02 08:00（00:00 UTC；整点可能是日期占位） | 标题 six；正文有 five，不能抹掉其限定 |',
'| Griffin／AI工具宝箱编辑组 | https://www.aitoollab.cn/articles/ai-digital-human-tools-2026/ | AI数字人工具横评2026：Tavus 通过视频图灵测试，HeyGen/Synthesia/D-ID 对比 | 2026-10-03，页面未给时刻/时区 | 正文也给 n=54 和公司条件；只存标题措辞 |',
'| Griffin／Tech-ish | https://tech-ish.com/2026/10/02/tavus-griffin/ | Nearly half of people on a video call with Tavus\'s Griffin AI thought they were talking to a human | 2026-10-02 05:25:59（00:25:59 +03:00） | 明确限定参与视频通话者；正文有方法限制，未把它判为夸大 |',
'| DAYJOB | 未找到符合目标措辞且已核到原站的页面 | — | — | 未把“25% 任务”自动替换成“24% 的工作”来凑例子 |','',
'没有找到满足全部要求的英文媒体“AI 已通过经典图灵测试”无条件断言实例；Tavus 自己的官方 X 明确宣称 first model to pass the video Turing test，属于厂商自述，已列 C3。','',
'### 社区质疑线索（L6，不能直接升级成事实）','',
'Reddit 原帖：https://www.reddit.com/r/singularity/comments/1wv7q40/ 。高赞评论 karl_mainz（抓取时 71 分）提及已有 £20m deepfake video call 骗局并担忧诈骗用途；该评论附有既有报道链接，但不证明 Griffin 已被用于该骗局。只取得线程/作者/得分，未取得该评论独立 permalink；见 d-reddit-comments.json。其他争论多为意见，不充当方法证据。HN 本次未取得满足“高赞且有可核证据”的目标评论。','',
'## 可能的吠点（条件与缺口，不是正文结论）','',
'1. Meta 官方说 six papers / five answers；“AI 独立解决六个”会同时抹掉篇数口径和人类指导/核查。（A1、A2）',
'2. 博文写 After completing；三篇论文并行工作段写 While completing；如实记录措辞差异，不能据此推断主观意图。（A1、A2）',
'3. 概率结果未解决恰在阈值处，且是高维概率陈述，不能改成有限维超过一点即绝对概率零。（A3）',
'4. 六篇都有 Human/AI 页边标记；AI 的定义是 assisted，原件并未说所有标记段落无需人工负责。（A2）',
'5. 算术物理没有公开列 reviewer 名单，不等于没有 review；“哪五篇”仍缺官方显式映射。（A6、问题表）',
'6. DAYJOB 的严格全条件通过率不是工作量完成比例；放宽到 90% criteria 后 leader 为 62.4%/58.3%。（B2–B3）',
'7. DAYJOB 时间是作者估计，judge 是模型且未找到人类一致率；error 尝试排除等规则必须随结果保留。（B3–B4）',
'8. DAYJOB 论文与官方博客的人类耗时不同；没有版本解释前不能混用。（B4）',
'9. Griffin 48% 来自特定一分种、末尾才询问是否为 AI 的公司研究；招募平台、原始问卷和 CI 未披露。（C1）',
'10. VideoFDB 的 LM rubric 分数、跨 judge 一致性，与人类真伪判断是不同指标；榜单第一不等于人类参考。（C2）',
'11. Tavus X 的 Note 是提议并征求评分状态；Protos 报道的 1300 万是其写作时历史数字。（C3）',
'12. Turing 1950 正文抓取受阻，暂不能用本次存档支持“原始审问者知道其中一方是机器”的逐字对照。（C4）','',
'## 扫描说法勘误','',
'| 扫描说法 | 是否被本次存档证实 | 证据与界限 |','|---|---|---|',
'| Generation 3.83 vs human 3.92 | 已找到 | 当前 VideoFDB Generation Overall；21 图 |',
'| Tavus 原帖超过 1300 万 views | 已找到 | Protos 逐字如此报道；本次 X 约 1983 万，保留时点差异 |',
'| 出现 Community Note 质疑图灵测试定义 | 部分支持 | 可见提议 Note，质疑自述/非标准 protocol；未见已获认可状态，也不是逐字讨论 Turing 1950 定义 |',
'| DAYJOB 共 30 个模型配置 | 已找到 | 摘要、Table 3、原始 CSV 均 30；不是 30 个厂商 |',
'| DAYJOB 提交时刻 08:39:10 UTC | 已找到 | Submission history v1；北京 10-01 16:39:10 |',
'| Meta AIHOT 热度约 75 | 部分支持 | 本次看到 AI评分 73；不是同一时点，也不是页面称为热度的指标；旧值未证实 |',
'| DAYJOB AIHOT 约 78 | 未找到一手来源 | 两种 DAYJOB 搜索为空，未找到对应历史条目/分数 |','',
'## 取证边界','',
'仅新增本期文件。没有验证六篇数学证明、复跑 GAP/Lean、复跑 benchmark、做新的统计推断，也没有确认首发优先权或全部利益关系。页面公开内容存档不含登录者导航/私信；X 截图只保留帖子卡片。未使用镜像、代理域名、付费墙绕过或自行登录。完整文件清单、哈希、尺寸与失败记录分别见 d-file-inventory.md / d-artifact-manifest.json / capture-log.md。']
(P/'evidence.md').write_text('\n'.join(out)+'\n',encoding='utf8')
print('wrote evidence.md; rows',len(rows),'main table candidate rows',len(trs),'soft rows',len(hc))
