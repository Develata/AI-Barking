import subprocess,runpy,pathlib
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P'];im=p.parent/'images'
shots=[('02-meta-probability','Probability: The Strict Threshold',1000),('03-meta-pde','Differential Equations:',1050),('04-meta-group','Group Theory:',1050),('05-meta-optimization','Optimization:',1000),('06-meta-arithmetic','Arithmetic Physics:',1000),('07-meta-algebra','Non-Associative Algebra:',1050)]
for name,needle,h in shots:
 r=subprocess.run(['bun',str(p/'d-shot.mjs'),'ev1003a',str(im/(name+'.png')),needle,str(h)],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=90);m['log']('https://research.meta.ai/blog/solving-open-research-problems-together','OpenCLI CDP DPR2 screenshot',r.stdout+r.stderr,name+'.png');print(name,r.returncode,flush=True)
