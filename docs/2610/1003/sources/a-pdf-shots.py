import pathlib,re,subprocess,runpy
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
for i in range(1,7):
 f=p/f'a-paper-{i}-full.txt';pages=f.read_text(encoding='utf8').split('\f');candidates=[n+1 for n,t in enumerate(pages) if re.search(r'^AI\s',t,re.M) and re.search(r'^Human\s',t,re.M)]
 page=candidates[0] if candidates else 1;print(i,'pages',len(pages),'labels',candidates,'chosen',page)
 out=p.parent/'images'/f'{i+7:02d}-meta-paper-{i}-labels'
 r=subprocess.run(['pdftoppm','-f',str(page),'-l',str(page),'-singlefile','-scale-to-x','1400','-scale-to-y','-1','-png',str(p/f'a-paper-{i}-full.pdf'),str(out)],capture_output=True,text=True)
 m['log']('见 a-paper-downloads.json 第 '+str(i)+' 项','Poppler PDF original page raster','page='+str(page)+'; width=1400; 700 CSS equivalent at 2x; '+str(r.returncode),out.name+'.png')
