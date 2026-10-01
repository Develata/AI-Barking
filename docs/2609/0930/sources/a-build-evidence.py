import pathlib,json,re,datetime
P=pathlib.Path('docs/0930/sources')
def txt(n):return (P/(n+'.md')).read_text(encoding='utf-8-sig')
def bt(n):return json.loads((P/(n+'-browser.json')).read_text(encoding='utf-8'))['text']
def line(n,term):return next(x for x in txt(n).splitlines() if term in x)
def para(t,term):
 return next(x.strip() for x in t.split('\n\n') if term in x)
def q(s):return s.replace('|','\\|').replace('\n','<br>')
def imgs(*names):return '; '.join('[%s](../images/%s)'%(n,n) for n in names) or '—'
rows=[]
def row(id,claim,level,url,quote,image,notes,status='已找到'):
 rows.append([id+' '+claim,level,url,q(quote),image,notes,status])
OA='https://openai.com/index/introducing-gpt-6-1-sol/'
AN='https://www.anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities'
AA='https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence'
WH='https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/'
a=bt('a-release')
row('A1','发布日期、可用范围及标准价','L1',OA+' ; https://developers.openai.com/api/docs/changelog',para(a,'GPT‑6.1 Sol is available starting today'),imgs('01-sol-title.png','02-sol-price-availability.png'),'当前发布页首屏与正文未显示自身发布日期；不能把下方推荐卡片的 Sep 29, 2026 当成此页日期。官方 API changelog 的 Sep 29 / gpt-6.1-sol 条目支持 2026-09-29（无时区）；不据此推算准确北京时间。发布页正文与 9/30 01:44 北京时间旧存档一致，导航有变化。见 a-release-browser.json、a-changelog.md、a-release-comparison.md。','部分支持')
price=[]
for n in ['a-sol-model','a-sol61-model','a-astra-model']:
 t=txt(n);i=t.rfind('Text tokens');j=t.index('Modalities',i);price.append(n+':\n'+t[i:j].strip())
row('A2','三款模型当前 Standard 输入/缓存输入/输出单价','L1','https://developers.openai.com/api/docs/pricing ; https://developers.openai.com/api/docs/models/gpt-6-sol ; https://developers.openai.com/api/docs/models/gpt-6.1-sol ; https://developers.openai.com/api/docs/models/gpt-6-astra','\n\n'.join(price),imgs('08-standard-price-table.png','09-sol-old-price.png'),'美元/百万 token；短上下文输入 ≤272K，标准档。GPT-6 Sol 2/0.20/10；6.1 Sol 2/0.10/10；Astra 10/1/50。另保留 cache writes 原列。长上下文全请求输入/缓存 2 倍，输出 1.5 倍；Batch/Flex、Fast、区域加价不可混用。openai.com/api/pricing HTTP 403；platform.openai.com/docs/pricing 正常重定向至 developers 官方页。主价格页默认只列当前旗舰三款，旧 Sol 由专属模型页补证。')
old=json.loads(pathlib.Path('docs/0924/sources/openai-pricing-expanded-rows.json').read_text(encoding='utf-8-sig'))
# Preserve a small historical provenance excerpt separately, without changing the old archive.
oldrows=[]
def walk(x):
 if isinstance(x,str) and ('gpt-6-sol' in x or 'gpt-6-astra' in x):oldrows.append(x)
 elif isinstance(x,list):
  for z in x:walk(z)
 elif isinstance(x,dict):
  for z in x.values():walk(z)
walk(old)
(P/'a-history-price-excerpts.json').write_text(json.dumps({'source':'docs/0924/sources/openai-pricing-expanded-rows.json','rows':oldrows,'status':'historical local archive, not newly fetched'},ensure_ascii=False,indent=2),encoding='utf-8')
row('A3','GPT-6 Sol 历史价与现价、北京时间发布日期','L1（历史存档+当前）','https://developers.openai.com/api/docs/models/gpt-6-sol ; https://openai.com/index/introducing-gpt-6-sol-and-luna/ ; https://x.com/OpenAI/status/2102460975790137662','\n'.join(oldrows)+'\nHistorical official launch post datetime: 2026-09-22T18:12:13.000Z',imgs('09-sol-old-price.png'),'0924 标准表历史存档与当前 Sol 2/0.20/10 一致；两次快照不能证明中间从未改价。6.1 相对 6 Sol 输入/输出未降，缓存 0.20→0.10；两代输入/输出均为 Astra 的 1/5（算术核对）。旧官方发布帖时间换算为北京时间 9/23 02:12:13；发布页推荐卡 datetime=2026-09-22T18:00:00Z 对应 9/23 02:00，二者不是同一事件时刻。历史文件 docs/0924/sources/openai-launch-browser-time.json。','部分支持')
row('A4','三项按任务成本说法及图表','L1',OA,'\n\n'.join(para(a,x) for x in ['On DeepSWE v1.1','On OSWorld 2.0’s offline set','On Terminal-Bench Science 0.1'])+'\n\n'+para(a,'GPT‑6 Astra still achieves the highest'),imgs('03-sol-deepswe.png','04-sol-osworld.png','05-sol-science.png'),'OpenAI 自测；不是 token 单价。DeepSWE v1.1 roughly 1/5；OSWorld 2.0 offline partial reward、v2026.08.08、maximum effort，roughly 1/7；Science 0.1 max 平均每任务 $5.47 / Astra $23.80 / Opus 5.5 $23.21。完整图及图例、轴、图下注已截；默认交互散点未在每个点上静态显示 effort，不能把所有点都称 max。')
row('A5','评测环境与竞品数据脚注','L1',OA,para(a,'Evaluations of GPT were performed'),imgs('02-sol-price-availability.png'),'研究环境/API 与生产 ChatGPT 的系统提示、工具、effort 等不同；竞品数值来自公开报告。脚注整句见 a-release-browser.json。')
zh=bt('a-devday-zh');en=bt('a-devday-en')
row('A6','DevDay 中文“四分之一”是否仍存在','L1','https://openai.com/zh-Hans-CN/index/devday-2026-recap/ ; https://openai.com/index/devday-2026-recap/',para(zh,'它以仅为 Astra')+'\n\n'+para(en,'We’re introducing GPT‑6.1 Sol'),imgs('06-devday-chinese.png','07-devday-english.png'),'当前中文已改为五分之一，英文 a fifth；与 9/30 01:40 北京时间旧中文存档“四分之一”不同，修改发生在两次抓取之间，精确修改时间未找到。见 a-devday-zh-comparison.md。','与说法不符')
row('A7','AA 文章日期、指数、成本用词、token、编码 effort','L3',AA,'\n'.join([line('a-aa-article','At max effort, GPT-6.1 Sol costs'),line('a-aa-article','GPT-6.1 Sol uses ~10-30%'),line('a-aa-article','We observed the xhigh effort')]),imgs('22-aa-intelligence-cost.png','23-aa-coding-index.png','24-aa-token-efficiency.png'),'页面日期 September 29, 2026，无时区。文章首图 Intelligence Index v4.3.2：6.1 Sol max 52、Astra max 53；模型系列页 low/medium/high/xhigh/max=42/48/50/51/52。原文为 per Intelligence Index task，非“跑完整个指数 $0.72”。编码图 Coding Agent Index v1.5：Codex+6.1 Sol xhigh 63、max 60；Astra max 62；6 Sol max 57；低档散点没有逐点整数标签，不从图估算。输出 token 增幅是各 effort 与 6 Sol 对比。','与说法不符')
row('A8','AA 当前榜单/模型页指数与成本','L3','https://artificialanalysis.ai/leaderboards/models ; https://artificialanalysis.ai/models/gpt-6-astra ; https://artificialanalysis.ai/models/releases/gpt-6-1-sol','GPT-6.1 Sol (max)\n52\nGPT-6.1 Sol (xhigh)\n51\nGPT-6.1 Sol (high)\n50\nGPT-6.1 Sol (medium)\n48\nGPT-6.1 Sol (low)\n42\nPer Intelligence Index Task\nCost per Intelligence Index task',imgs('25-aa-sol-effort-levels.png'),'系列页低至高 effort 成本 $0.13/$0.21/$0.32/$0.39/$0.72；Astra max 指数53、每题$3.26。原始 md/html 与浏览器 json 均留存；首次 HTTP 抓取 9/30 23:53–23:55 北京时间，浏览器后续时点见日志。榜单动态值以具体快照为准，不和文章发布日期混用。')
row('A9','HN 与 Reddit 热度','L1（平台指标）；帖文 L6','https://news.ycombinator.com/item?id=49896586 ; https://www.reddit.com/r/OpenAI/comments/1wtg4f3/','GPT 6.1 Sol: Near-Astra intelligence for a fifth of the price\nOpenAI launches GPT-6.1 Sol', '—','HN Algolia 首次 1024 points/891 comments，官方 Firebase 后抓取1028/893；以 a-hn-item.json 与日志时点为准。Reddit read 抓取 score668；该适配器不返回评论总数，不能把读到的评论条数当总数。见 a-reddit-openai.json。','部分支持')
row('B1','Anthropic 报告发布时间、作者、主旨','L1（厂商自测）',AN,line('b-anthropic','But those models have now arrived.'),imgs('10-anthropic-title.png'),'署名 Andrew Fasano、Marius Fleischer、Cole McFaul、Robert Xiao、Tripp Gallagher。页面 Sep 29, 2026；JSON-LD datePublished=2026-09-29T15:46:00.000Z（北京时间9/29 23:46），dateModified=2026-09-30T08:46:41.000Z（北京时间9/30 16:46:41）。搜索引擎另报15:56，不采用其作为官方发布时间。')
row('B2','恶意请求四条件比例及指标定义','L1',AN,line('b-anthropic','Rate at which each model tried')+'\n\n'+line('b-anthropic','No model-generated code is ever executed'),imgs('12-engagement-full.png'),'0%/64%/92%/100% 是 direct/cover story/prefill/abliteration 下尝试连接目标的参与率；50 samples per cell=5 attack orders×2 targets×5 attempts。模拟工具、代码不执行、无外部交互；不是攻击成功率，也不是在真实网络的成功率。')
row('B3','ExploitBench 实际数字、其他模型与测试条件','L1',AN,line('b-anthropic','we focus on the models’ ability')+'\n\n'+line('b-anthropic','Exploitation capability versus output-token budget'),imgs('11-exploitbench-full.png'),'原文 50/410 vs 56/410；图四舍五入为12% vs14%。图2其他模型：Kimi K3 0.5%，DeepSeek-V4.1-Flash 0.2%，Opus4.6和GLM5.2均0%。只记录图示百分比，不倒推出未刊登的整数次数。41个V8已知漏洞，410 attempts；隔离沙箱、离线测试构建、无网络；图2 Claude Opus4.6/Mythos Preview关闭防护。Binary Exploitation另为随机100任务：GLM5.3 4%、Mythos Preview6%，其他0%。')
row('B4','$20.40/人工20分钟/模型8小时与浏览器未知漏洞是否同一实验','L1',AN,line('b-anthropic','In the first of these sessions')+'\n\n'+line('b-anthropic','This took 20 minutes of human attention'),imgs('14-flash-cost-context.png'),'两件实验：第一件 GLM-5.3+研究员、沙箱中的本地 Linux 浏览器、此前未知 JavaScript 引擎漏洞；研究者驱动且浏览器只提供 Linux 环境，其他平台可受影响是作者推测。第二件 GLM-5.3-Flash，给定已公开 CVE-2026-11645 与另一已知漏洞，ARM64 exploit chain、PAC；20分钟人工+8小时模型，$20.40是按智谱API价估算 would have cost，不是第一件未知漏洞实验账单。','与说法不符')
row('B5','Claude 哪些对照不适用、Mythos Preview 的发布方式','L1',AN+' ; https://www.anthropic.com/glasswing',line('b-anthropic','The Anthropic API provides would-be attackers')+'\n\n'+line('b-anthropic','In light of these considerations'),imgs('12-engagement-full.png','13-abliteration-full.png'),'prefill：API不提供预填thinking方式；abliteration：没有公开/可自定义权重；图中锁表示不适用/不可实施，不是实际测试出0%的同一实验。cover story可比条件下拒绝。Mythos Preview 经 Project Glasswing 向受信任防御者限量开放；当前更多模型访问变化不能反推此前全面开放。')
row('B6','CAISI 日期、基准、关键数字与防护条件','L1','https://www.nist.gov/news-events/news/2026/09/caisis-assessment-zais-glm-53-cyber-capabilities',line('b-nist','CAISI evaluated all models as agents')+'\n\n'+'SEC-Bench Pro\n40.4% (74/183)\nExploitBench\nScore reflects the best of three attempts per task.\n61.1% (9.8/16)\nExploitGym\n9.4% (47/498)\nOSS-Fuzz\n7.7% (23/297)',imgs('15-caisi-benchmark-definitions.png','16-caisi-results.png'),'9/17/2026（页面无时区）。四基准定义和全部模型列/95% Wilson CI完整存档。US frontier对应值90.2%、100%、44.4%、23.2%；PRC previous frontier27.3%、32.2%、2.6%、2.4%。最大推理，适用时美国模型关闭cyber safeguards；涵盖trusted-access release。SEC-Bench Pro 183任务是触发指定bug的crash；ExploitBench41任务16分量表best-of-three。不能与Anthropic50/410直接比较；ExploitGym实际GLM分母498，定义表为502，原样保留。')
row('B7','GLM-5.3 开放权重、许可证、发布日期、官方回应','L1；NIST对第三方发布事实为L3','https://huggingface.co/zai-org/GLM-5.3 ; https://huggingface.co/zai-org/GLM-5.3/raw/main/LICENSE ; https://docs.z.ai/guides/llm/glm-5.3','GLM-5.3 License\n'+line('b-hf-license','If the Licensee or any of its affiliates operates'), '—','官方Hugging Face模型与权重文件链接可访问，许可证为自定义 GLM-5.3 License，不能写成MIT或无限制。MaaS业务合并收入任意连续12个月超100亿美元的商业使用须Z.ai安全审查。NIST记载模型8/14发布、两周后公开权重；开发者本轮可见页面未找到精确开放权重日期。Z.ai博客HTTP200仅598字节JS壳；官方文档与限定官方域搜索未找到对9/29报告的直接回应，不等于没有回应；未穷尽所有官方社交帖。','部分支持')
row('B8','LocalLLaMA、singularity 与 HN 热度','L1（平台指标）；帖文L6','https://www.reddit.com/r/LocalLLaMA/comments/1wtg0vd/ ; https://www.reddit.com/r/singularity/comments/1wtkwkq/ ; https://news.ycombinator.com/item?id=49897075','GLM-5.3 and the Spread of Advanced Cyber Capabilities \\ Anthropic\n[ Removed by moderator ]','—','Reddit read 当前分数394与387；singularity主帖已被moderator移除，保留此状态，不能把评论引用当原帖全文。适配器没有返回评论总数。HN Algolia235/225，后续官方Firebase237/225。各时间见capture-log；缺失评论数不补猜。','部分支持')
row('C1','行政令标题、日期、第1–3节适用范围、定义与立法建议','L1',WH,'\n\n'.join(line('c-order',term) for term in ['The terminology used','To the maximum extent permitted by law, executive departments','Nothing in this section requires','For purposes of this order','Within 60 days of the date','an assessment of whether','any proposed conforming','recommendations for any additional']),imgs('17-whitehouse-section-one.png','18-whitehouse-sections-two-three.png'),'标题 Inaugurating The Era Of Super Intelligence；页面9/29/2026，meta发布时间2026-09-29T21:17:25+00:00=北京时间9/30 05:17:25。限制 to maximum extent permitted by law、executive branch、non-statutory documents；豁免已发法规、总统行动、合同、拨款和历史文件。第3(a)引用现行15 USC9401(3)；第3(b)是60天内提交建议，不是已经通过新法律定义。')
fs=txt('c-fact-sheet');start=fs.index('In July 2025');end=fs.index('In March 2026',start)
row('C2','Fact Sheet 的 AI Action Plan / SI race 整句','L1','https://www.whitehouse.gov/fact-sheets/2026/09/fact-sheet-president-donald-j-trump-inaugurates-the-era-of-super-intelligence/',fs[start:end].strip(),imgs('19-whitehouse-action-plan.png'),'原文同一句并用 AI Action Plan 与 SI race。文中多处原政策名改为SI，不据此推断技术达成新门槛。')
row('C3','行政令页导航 Lead the World in AI','L1',WH,'Lead the World in AI',imgs('26-whitehouse-ai-navigation.png'),'原站导航菜单 Priorities / Top Priorities 下；打开菜单截图，HTML也有同文字。仅记录抓取时点，不推断更新计划。')
cg=json.loads((P/'c-ai-gov-counts.json').read_text(encoding='utf-8'))
row('C4','ai.gov 标题、首屏及两术语出现次数','L1','https://www.ai.gov/',cg['title']+'\n'+bt('c-ai-gov')[:650],imgs('20-ai-gov-first-screen.png'),'计数范围 document.body.innerText，大小写不敏感的完整短语出现次数（非全文HTML/script、非全站）；artificial intelligence=11、Super Intelligence=0；北京时间 '+cg['time_bjt']+'。另保留完整HTML，使用不同计数范围会产生不同数字。')
law=txt('c-law-text');i=law.index('(3) Artificial intelligence');j=law.index('(4)',i)
row('C5','15 USC9401(3) 原文定义','L1','https://uscode.house.gov/view.xhtml?req=title:15%20section:9401%20edition:prelim',law[i:j].strip(),'—','用户granuleid线索URL返回docnotfound（HTTP200软失败），同一官方站点按title/section/edition查询成功；页面注明 laws in effect on September 29, 2026。不是第三方镜像。')
row('C6','SI 国际单位制、NIST SP330','L1','https://www.nist.gov/pml/special-publication-330 ; https://www.nist.gov/pml/special-publication-330/sp-330-version-history ; https://www.nist.gov/pml/owm/metric-si/si-units','The International System of Units (SI) 2019 Edition\nThis supersedes NIST Special Publication 330, 2008 Edition.',imgs('21-nist-si-title.png'),'官方SP330页面与version history当前仍列2019版，网页Updated August18,2025不等于SP330出版了2025版。2019 PDF已存档c-sp330.pdf；NIST所属美国商务部可由PDF封面机构署名核对。')
row('C7','9/22联合国讲话起源、国务院邮件','L1（官方节选）；L4（媒体邮件报道）','https://www.whitehouse.gov/releases/2026/09/president-trump-at-the-united-nations-while-others-have-talked-i-have-acted/ ; https://www.euronews.com/2026/09/24/artificial-is-out-trump-orders-officials-to-call-it-super-intelligence-instead/',line('c-un-release','The United States totally rejects')+'\n\n'+line('c-euronews','The order arrived in an email')+'\n\n'+line('c-euronews','The use of the word artificial makes intelligence fake'),'—','白宫9/22官方讲话节选确认 hereinafter officially called Super Intelligence，但本轮未找到含fake整句的白宫完整文字稿。fake整句和邮件标题仅由Euronews/AP报道支持，标L4；没有获得国务院邮件原件，不写成一手已核实。','部分支持')
header='''# 0930 取证事实清单

仅取证，不是发布正文或编辑结论。当前基线 main@62e6b4bfe04871d36e801429ad28b3073d85097a；保留既有 .workbuddy/ 与派工单。抓取跨北京时间 2026-09-30 / 2026-10-01，逐次时间以 capture-log.md 为准。

“已找到”表示本行来源与口径可核对，不表示独立复现厂商评测。“与说法不符”针对派工线索中的具体口径，详见条件栏；同一行其他子项仍可能得到支持。“部分支持”逐项标明缺口。引句保持原语言；HTML文本抽取换行在表内用 `<br>` 表示。当前浏览器 JSON 优先于403挑战页HTML，旧存档不冒充本次抓取。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
'''
(P/'evidence.md').write_text(header+'\n'.join('| '+' | '.join(r)+' |' for r in rows)+'\n',encoding='utf-8')
print('rows',len(rows))
