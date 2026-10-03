from pathlib import Path
p=Path(__file__).parent
for f in sorted(p.glob('a-paper-*-full.txt')):
 t=f.read_text(encoding='utf8');print('\nFILE',f.name, '\n',t[:700])
 pos=t.find('Statement of AI Use',1000)
 if pos<0:pos=t.find('Statement of AI use',1000)
 print(t[pos:pos+2600])
