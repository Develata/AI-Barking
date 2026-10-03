import subprocess,pathlib,sys,json,datetime,time
P=pathlib.Path(__file__).parent
for n,u in json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8-sig')).items():
 results=[]
 for args in [('open',u),('state',),('extract','--chunk-size','100000')]:
  r=subprocess.run(['opencli.exe','browser','evidence1002b',*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100);results.append(r.stdout);time.sleep(2 if args[0]=='open' else 0)
 (P/(n+'-extract.json')).write_text(results[-1],encoding='utf-8')
 with (P/'b-capture-log.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps({'url':u,'time_bjt':datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(),'tool':'opencli browser open/state/extract','result':'inspect '+n+'-extract.json'},ensure_ascii=False)+'\n')
 print(n,results[-1][:180],flush=True)
