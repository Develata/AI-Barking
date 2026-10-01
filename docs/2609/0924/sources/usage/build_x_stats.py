import json, pathlib, collections, math, random, re, email.utils, datetime
p=pathlib.Path(__file__).resolve().parent
raw=[json.loads(l) for l in (p/'x_raw.jsonl').open(encoding='utf-8')]
protocol=json.load((p/'protocol.json').open(encoding='utf-8'))
start=datetime.datetime.fromisoformat(protocol['start_utc']);end=datetime.datetime.fromisoformat(protocol['end_utc'])
names=dict(O='Claude Opus 5.5',A='GPT-6 Astra',S='GPT-6 Sol',L='GPT-6 Luna',F='Claude Fable 5.1')
sent={'+':'pos','-':'neg','m':'mixed','n':'na'}
verdict={'O':'prefer_opus','A':'prefer_astra','m':'mixed','n':'na'}
tasks=dict(c='coding_agent',w='writing',r='reasoning_math',h='chat',o='other',u='unspecified')
decisions={}
for l in (p/'coding_decisions.tsv').read_text().splitlines():
 if l and not l.startswith('#'):
  i,ms,v,o,s,lu,t=l.split();assert int(i) not in decisions
  decisions[int(i)]=(ms,v,o,s,lu,t)
anchors={159:'I used Opus 5.5 to formally verify the Claude Agent SDK using Lean.',209:"I'm VERY disappointed in GPT-6 Sol.\n\nI'm VERY pleased with GPT-6 Luna.",304:'刚用了一会儿 Claude Opus 5.5，你们知道我什么感觉吗？',273:'It looks like a huge degradation.',94:'First Opus 5.5 vs GPT-6 Astra test is 3D.',446:'试了一下网页端的Opus5.5',430:'GPT-6-Astra is still the frontier model by a comfortable margin.',1058:'I woke up today to a major mess by opus.',133:'以前我在小说创作辅助',213:'但是在事实核查和观点推倒上存在重大缺陷',993:"It's also noticeably much faster",117:'but still super slow',406:'It ignores rules and checklist.',656:'Kalite ✅ Coding ✅',267:'GPT-6 Luna is a clearer disappointment.',332:'Yeah, it took almost twice as long as Astra',436:'opus 5.5 测试过了',873:'We’ll need to run more tasks to compare Astra and Opus 5.5',1052:'Though Astra has the edge on nuance.'}
def quote(i,text):
 if i in anchors:
  a=anchors[i];assert a in text,(i,a);pos=text.index(a)
 else:
  # Select a short, verbatim evidence anchor. Full text remains the coding basis.
  chunks=list(re.finditer(r'[^\n。!?]+[。!?]?',text))
  pat=r'\b(I|my|we|our|used|using|tried|tested|testing|better|worse|good|bad|disappoint|impress|feels|prefer|cheap|expensive|slow|fast)\b|我|用|测|体感|感觉|额度|好|差|强|慢|快'
  match=max(chunks,key=lambda m:len(re.findall(pat,m.group(),re.I)),default=None)
  pos=match.start() if match else 0
 q=text[pos:].split('\n\n')[0].strip()
 if re.search('[\u4e00-\u9fff]',q):return q[:85]
 words=list(re.finditer(r'\S+',q));return q[:words[min(23,len(words)-1)].end()] if words else text[:80]
def dump(name,rows):
 (p/name).write_text(''.join(json.dumps(r,ensure_ascii=False)+'\n' for r in rows),encoding='utf-8')
coded=[]
for i,(ms,v,o,s,lu,t) in sorted(decisions.items()):
 r=raw[i];assert start<=email.utils.parsedate_to_datetime(r['created_at'])<=end,i
 assert v=='n' or ('O' in ms and 'A' in ms),i
 for model,val in [('O',o),('S',s),('L',lu)]:assert val=='n' or model in ms,i
 txt=r['text'];lang='other' if re.search('[\u3040-\u30ff]',txt) or i in [629,652,656] else ('zh' if re.search('[\u4e00-\u9fff]',txt) else 'en')
 coded.append({k:r[k] for k in ['id','author','url','text','created_at','likes','views']}|dict(raw_index=i,models=[names[m] for m in ms],verdict_opus_vs_astra=verdict[v],sentiment_opus55=sent[o],sentiment_sol=sent[s],sentiment_luna=sent[lu],task=tasks[t],lang=lang,employee=i==159,employee_status='confirmed_anthropic_official_site' if i==159 else 'unknown_bio_unavailable',evidence_quote=quote(i,txt),coding_basis='full_post_text; short quote is an anchor, not all supporting spans'))
dump('x_coded.jsonl',coded)
official={'openai','anthropicai','claudeai','claudedevs','openaidevs','chatgpt','openainewsroom'}
duplicates={34,47,279,331,351,383,519,558,603,636,661,785,792,804,887,1002,937,974,1043}
ads={126,150,155,189,247,269,275,298,302,310,366,374,397,424,453,482,497,509,537,539,559,562,671,676,746,847,861,878,940,947,1007,1040,1098}
excluded=[]
for i,r in enumerate(raw):
 if i in decisions:continue
 if not start<=email.utils.parsedate_to_datetime(r['created_at'])<=end:reason='outside_time_window'
 elif r['author'].lower() in official:reason='official_account'
 elif i in duplicates or 'localhost:3000' in r['text'] and 'localhost:8000' in r['text']:reason='duplicate_template_or_satire'
 elif r['author'].lower()=='grok':reason='bot_reply'
 elif i in ads or r['author'].lower() in ['higgsfield_ai','flowith','happycapyai','quadcode_ai'] or re.search('抽奖|带货|VPN寿司云|加飞机群|giveaway|affiliate|promo code',r['text'],re.I):reason='advertising_promotion_giveaway'
 elif re.search('^RT @',r['text']):reason='uncommented_repost'
 elif i in [18,34,52,355,587,727,891,1102]:reason='rumor_or_speculation_only'
 else:reason='no_clear_firsthand_model_experience' # includes news, other people's demos, vague praise, or unspecified model versions
 excluded.append(dict(id=r['id'],url=r['url'],raw_index=i,reason=reason))
dump('x_excluded.jsonl',excluded)
blind=random.Random(20260924).sample(coded,min(20,len(coded)))
dump('x_blind20.jsonl',[{k:r[k] for k in ['id','url','text']} for r in blind])
def wilson(k,n):
 if not n:return None
 z=1.959963984540054;ph=k/n;den=1+z*z/n
 center=(ph+z*z/(2*n))/den;half=z*math.sqrt(ph*(1-ph)/n+z*z/(4*n*n))/den
 return [max(0,center-half),min(1,center+half)]
def dist(rows,key,categories):
 c=collections.Counter(r[key] for r in rows);n=sum(c[x] for x in categories)
 return dict(n=n,na=c['na'],categories={x:dict(k=c[x],n=n,proportion=c[x]/n if n else None,wilson95=wilson(c[x],n)) for x in categories})
vs=['prefer_opus','prefer_astra','mixed'];ss=['pos','neg','mixed']
stats=dict(raw_returned=sum(r['count'] or 0 for r in map(json.loads,(p/'search_ledger.jsonl').open(encoding='utf-8'))),deduplicated=len(raw),included=len(coded),excluded=len(excluded),exclusions=dict(collections.Counter(r['reason'] for r in excluded)),unique_authors=len(set(r['author'].lower() for r in coded)),employee_true=sum(r['employee'] for r in coded),lang=dict(collections.Counter(r['lang'] for r in coded)),task=dict(collections.Counter(r['task'] for r in coded)),verdict=dist(coded,'verdict_opus_vs_astra',vs),sentiments={k:dist(coded,k,ss) for k in ['sentiment_opus55','sentiment_sol','sentiment_luna']},by_task={t:dist([r for r in coded if r['task']==t],'verdict_opus_vs_astra',vs) for t in tasks.values()},without_confirmed_employee=dist([r for r in coded if not r['employee']],'verdict_opus_vs_astra',vs),top5=[{k:r[k] for k in ['id','author','url','views','raw_index']} for r in sorted(coded,key=lambda r:int(r['views'] or 0),reverse=True)[:5]])
(p/'x_stats.json').write_text(json.dumps(stats,ensure_ascii=False,indent=2),encoding='utf-8')
assert len(coded)+len(excluded)==len(raw)
assert all(r['evidence_quote'] in r['text'] for r in coded)
print(json.dumps({k:stats[k] for k in ['raw_returned','deduplicated','included','excluded','unique_authors','exclusions','verdict','sentiments']},ensure_ascii=False,indent=2))
