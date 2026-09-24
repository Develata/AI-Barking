import pathlib,hashlib,json,subprocess,datetime,os,time
root=pathlib.Path(r'E:\gitclone\AI-Barking'); out=root/'0924/sources/usage'
base={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in root.rglob('*') if p.is_file() and '.git' not in p.parts and out not in p.parents}
(out/'baseline.json').write_text(json.dumps(base,indent=2,ensure_ascii=False),encoding='utf-8')
queries=['"Opus 5.5" Astra','"Opus 5.5" vs','"Opus 5.5" coding','"Opus 5.5" 体验','"Opus 5.5" 用了','"GPT-6 Sol"','"GPT-6 Luna"','Astra "Opus 5.5" 编程']
end=datetime.datetime.now(datetime.timezone.utc); env=os.environ.copy(); env['OPENCLI_BROWSER_COMMAND_TIMEOUT']='150'
(out/'protocol.json').write_text(json.dumps({'start_utc':'2026-09-22T16:00:00+00:00','end_utc':end.isoformat(),'queries':queries,'sorts':['live','top'],'requested_limit':100,'interval_seconds':4,'seed':20260924},ensure_ascii=False,indent=2),encoding='utf-8')
for i,q in enumerate(queries,1):
 for sort in ['live','top']:
  query=q+' since:2026-09-22 until:2026-09-25'
  t=datetime.datetime.now(datetime.timezone.utc).isoformat()
  p=subprocess.run(['opencli','twitter','search',query,'--product',sort,'--limit','100','-f','json'],env=env,capture_output=True,encoding='utf-8',errors='replace',timeout=200)
  (out/f'x_search_{i:02}_{sort}.json').write_text(p.stdout,encoding='utf-8')
  (out/f'x_search_{i:02}_{sort}.stderr.txt').write_text(p.stderr,encoding='utf-8')
  try: count=len(json.loads(p.stdout))
  except: count=None
  record={'site':'x','query':query,'base_query':q,'sort':sort,'started_at':t,'finished_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'requested_limit':100,'count':count,'returncode':p.returncode,'file':f'x_search_{i:02}_{sort}.json'}
  with (out/'search_ledger.jsonl').open('a',encoding='utf-8') as f: f.write(json.dumps(record,ensure_ascii=False)+'\n')
  print(json.dumps(record,ensure_ascii=False),flush=True)
  if 'not connected' in (p.stdout+p.stderr).lower(): raise SystemExit('X stopped: browser not connected')
  time.sleep(4)
