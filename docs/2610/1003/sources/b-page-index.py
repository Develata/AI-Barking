from pathlib import Path
p=Path(__file__).parent
for name in ['b-dayjob-pdf','a-original-pde-full','a-original-optimization-full']:
 pages=(p/(name+'.txt')).read_text(encoding='utf8').split('\f')
 for i,t in enumerate(pages):
  if any(x in t for x in ['Table 3: Launch','Unchallenged premises.','seems natural to conjecture','We leave open the question']):print(name,i+1,t[:180])
