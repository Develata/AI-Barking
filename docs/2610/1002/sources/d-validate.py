from pathlib import Path
import json,re,datetime,hashlib
from PIL import Image
P=Path('docs/2610/1002/sources');I=P.parent/'images';tz=datetime.timezone(datetime.timedelta(hours=8));now=datetime.datetime.now(tz).isoformat()
for fn in ['evidence.md','b-evidence.md','c-evidence.md']:
 p=P/fn;s=p.read_text(encoding='utf-8');s=s.replace('20–27已逐张','08–15已逐张').replace('官方页面截图42、43','官方页面截图18、19').replace('官方页面截图42、43','官方页面截图18、19').replace('图44为Space README归属，图45为应用标题','图20为Space README归属，图21为应用标题').replace('完整榜表截图三次CDP超时，缺该配图','完整榜表截图多次CDP超时/拼块重复，缺合格配图；失败原图移存sources/c-failed-jev-ranking.png，不能用于发布').replace('保留作者README和应用标题截图及完整DOM数据','保留作者README和应用标题截图及完整DOM数据；失败拼块图c-failed-jev-ranking.png只供排障')
 p.write_text(s,encoding='utf-8')
# Note all final C screenshots from actual filesystem after visual QA.
u={16:'https://blog.cloudflare.com/clef-decision-models/',17:'https://blog.cloudflare.com/clef-decision-models/',18:'https://blog.cloudflare.com/clef-decision-models/',19:'https://blog.cloudflare.com/clef-decision-models/',20:'https://huggingface.co/spaces/multimodalart/jev-decision-index/blob/main/README.md',21:'https://multimodalart-jev-decision-index.static.hf.space/index.html',22:'https://huggingface.co/Cloudflare/clef/blob/main/LICENSE',23:'https://huggingface.co/Cloudflare/clef-flash',24:'https://developers.cloudflare.com/workers-ai/models/clef/',25:'https://developers.cloudflare.com/workers-ai/models/clef-flash/',26:'https://clef-evals.workers-ai-mle.workers.dev/',27:'https://clef-evals.workers-ai-mle.workers.dev/#methodology',28:'https://huggingface.co/Cloudflare/clef',29:'https://typesafe.ai/blog/introducing-system-one-models-and-jev'}
with (P/'c-capture-records.jsonl').open('a',encoding='utf-8') as f:
 for p in I.glob('*.png'):
  n=int(p.name[:2])
  if n in u:f.write(json.dumps(dict(time=datetime.datetime.fromtimestamp(p.stat().st_mtime,tz).isoformat(),url=u[n],tool='OpenCLI browser/CDP DPR2',result='最终图视觉检查通过；mtime补记；已去账户顶栏',file=p.name),ensure_ascii=False)+'\n')
 f.write(json.dumps(dict(time=now,url=u[21],tool='OpenCLI native screenshot',result='FAIL 原生CDP多次超时，最终图存在重复拼块/截列，不可作证据；保留失败文件不放候选图目录',file='c-failed-jev-ranking.png'),ensure_ascii=False)+'\n')
# Consolidate per-operation logs, retain failures.
records=[]
for p in P.glob('*.jsonl'):
 if 'capture' not in p.name:continue
 for line in p.read_text(encoding='utf-8-sig').splitlines():
  if not line.strip():continue
  try:d=json.loads(line)
  except:continue
  records.append((str(d.get('time',d.get('time_bjt','未记录'))),str(d.get('url','未记录')),str(d.get('tool','见原始日志')),str(d.get('result',d.get('error','见原始日志'))),str(d.get('file','')),p.name))
records.sort()
def cell(s):return s.replace('|','\\|').replace('\n','<br>').replace('\r','')
lines=['# 1002 抓取日志','','时间均北京时间UTC+8；即时日志与mtime补记在结果栏区分。补记精度不表示网络完成时刻。URL查询参数仅保留检索/内容定位必需项；未使用镜像站。重试和失败都保留，HTTP200不单独作为内容成功判据。','','| 时间（北京） | URL | 工具 | 结果 | 文件 | 原始日志 |','|---|---|---|---|---|---|']
lines+=['| '+' | '.join(map(cell,r))+' |' for r in records]
lines+=['','检索词/覆盖范围见d-search-summary.md及各组检索存档。手工web查找仅用于发现原站，未逐项保留毫秒时间。初始状态main@0b2a422；原先仅派工单未跟踪。agent-reach conda示例入口失败，PATH入口成功，OpenCLI doctor连接正常。','','C组通用浏览器存档脚本部分执行遭自动审批拒绝（approval required但环境不允许请求审批）；改为范围更窄的公开正文/表格只读调用成功。该拒绝不代表页面不存在。未改权限、未改工具适配器。']
(P/'capture-log.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
# Mechanical checks.
s=(P/'evidence.md').read_text(encoding='utf-8');ids=re.findall(r'^\| ([ABC]\d+)\b',s,re.M);expected=[f'{g}{n}' for g,end in [('A',10),('B',10),('C',11)] for n in range(1,end+1)]
assert sorted(ids)==sorted(expected),(ids,expected)
valid={'已找到','部分支持','与说法不符','未找到一手来源'}
for line in s.splitlines():
 if re.match(r'^\| [ABC]\d+\b',line):assert line.split('|')[-2].strip() in valid,line
refs=set(re.findall(r'\b\d{2}-[a-z0-9-]+\.png',s));missing=[n for n in refs if not (I/n).exists()];assert not missing,missing
pngs=sorted(I.glob('*.png'));assert [int(p.name[:2]) for p in pngs]==list(range(1,len(pngs)+1))
invalid=[]
for p in P.glob('*.json'):
 try:json.loads(p.read_text(encoding='utf-8-sig'))
 except Exception as e:invalid.append((p.name,str(e)))
print('IDs',len(ids),'screenshots',len(pngs),'refs',len(refs),'invalid_json',invalid)
report=['# 交付自检','','- 31个ID各一行；状态全部符合四选一。','- 截图编号01–29连续；evidence.md中所有截图引用实际存在。','- C2三张官方表：10×6、4×3、2×6已完整抄录，主质量表与工作流/延迟表原站截图已查看。','- A/B代理逐图视觉核验；C核心表、方法、价格、Space归属与模型卡已主代理查看。','- 30号失败拼块榜图移入sources/c-failed-jev-ranking.png，未删除，不列候选。','- DPR2；宽为640–1400CSS、1280–2800物理像素；依模板CSS口径执行。','- C原始HTTP无cookies；HF浏览器档只取公开main/article，最终截图排除顶部当前会话头像。A/B社交存档脱敏见各组日志；社交截图未纳入候选。','- 不写发布稿、images/README.md、fact-check.md；不提交、不推送。','- JSON解析异常：'+repr(invalid),'','## 截图尺寸','']
for p in pngs:report.append('- '+p.name+': '+str(Image.open(p).size))
(P/'d-delivery-check.md').write_text('\n'.join(report)+'\n',encoding='utf-8')
# Complete inventory includes generated check and inventory itself, no archive copies.
inv=P/'d-file-list.md';inv.touch(exist_ok=True)
files=sorted(p for p in P.parent.rglob('*') if p.is_file())
ls=['# 新增文件清单','','全部位于docs/2610/1002/；HTML/PDF/图片等原件未提交。SHA256见d-artifact-manifest.json（不对manifest自身作自引用哈希）。','','| 文件 | 字节 | 说明 |','|---|---|---|']
manifest=[]
for p in files:
 rel=p.relative_to(P.parent).as_posix();note='失败截图，仅排障' if p.name=='c-failed-jev-ranking.png' else '超过1 MB；本轮不入库/不提交' if p.stat().st_size>1000000 else ''
 ls.append(f'| [{rel}]({rel.removeprefix("sources/") if rel.startswith("sources/") else "../"+rel}) | {p.stat().st_size} | {note} |')
 if p.name not in ['d-file-list.md','d-artifact-manifest.json']:manifest.append(dict(file=rel,bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
inv.write_text('\n'.join(ls)+'\n',encoding='utf-8');(P/'d-artifact-manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2),encoding='utf-8')
print('files',len(files)+1,'capture operations',len(records))
