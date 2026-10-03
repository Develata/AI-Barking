import json,pathlib
p=pathlib.Path(__file__).parent
for name in ['b-dayjob-abs-browser','c-griffin-browser']:
 d=json.loads((p/(name+'.json')).read_text(encoding='utf8'));print(name,[(a['text'],a['url']) for a in d['links'] if any(x in a['url'] for x in ['anc','src','prolific','paper','survey'])])
