import json,pathlib,email.utils,datetime,re,collections
p=pathlib.Path('0924/sources/usage');rows=[json.loads(l) for l in (p/'x_raw.jsonl').read_text(encoding='utf-8').split('\n') if l]; proto=json.load((p/'protocol.json').open(encoding='utf-8'));start=datetime.datetime.fromisoformat(proto['start_utc']);end=datetime.datetime.fromisoformat(proto['end_utc'])
# Candidate screening only; manual review will determine inclusion and codes.
pat=re.compile(r'\b(i|my|we|our|used|using|tried|testing|tested|built|switched|experience|feels?|found|spent|working|workflow|prompted|asked|gave)\b|我|实测|试|体验|用了|用过|用下来|用起来|感受|感觉|干活|写代码|编程|跑了|测了|写了|帮我|让它|让他|给我|用着|好用|难用|手感|上手|反馈|对比|任务|烧|额度',re.I)
reasons={}; candidates=[]
for i,r in enumerate(rows):
 t=email.utils.parsedate_to_datetime(r['created_at'])
 if not(start<=t<=end): reasons[str(i)]='outside_time_window'
 elif r['author'].lower() in ['openai','anthropicai','claudeai','claudedevs','openaidevs','chatgpt','openainewsroom']: reasons[str(i)]='official_account'
 elif not pat.search(r['text']):reasons[str(i)]='no_firsthand_claim_screen'
 else:candidates.append(i)
(p/'screening.json').write_text(json.dumps({'reasons':reasons,'candidates':candidates},indent=2),encoding='utf-8')
print('screen',collections.Counter(reasons.values()),'candidates',len(candidates))
for i in candidates[:65]:
 r=rows[i];print(f'[{i}] @{r["author"]} v={r["views"]}\n'+r['text']+'\n')

