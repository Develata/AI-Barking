import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
b.shot('17-whitehouse-section-one.png',370,1050)
b.shot('18-whitehouse-sections-two-three.png',1340,1080)
# Open the actual navigation using a observed native button; no text modifications.
print(b.call('eval','JSON.stringify([...document.querySelectorAll("button")].map(e=>({text:e.innerText,aria:e.getAttribute("aria-label")})))'),flush=True)
d=b.archive('c-fact-sheet','https://www.whitehouse.gov/fact-sheets/2026/09/fact-sheet-president-donald-j-trump-inaugurates-the-era-of-super-intelligence/')
y=b.call('eval','[...document.querySelectorAll("li,p")].find(e=>e.innerText.startsWith("In July 2025"))?.getBoundingClientRect().top+scrollY')
b.shot('19-whitehouse-action-plan.png',y-250,740)
d=b.archive('c-ai-gov','https://www.ai.gov/')
counts={x:d['text'].lower().count(x.lower()) for x in ['artificial intelligence','Super Intelligence']};print('AI_GOV_COUNTS',counts,flush=True)
import json
(b.P/'c-ai-gov-counts.json').write_text(json.dumps(dict(time_bjt=b.stamp(),url=d['url'],title=d['title'],scope='document.body.innerText, case-insensitive exact phrase',counts=counts),ensure_ascii=False,indent=2),encoding='utf-8')
b.shot('20-ai-gov-first-screen.png',0,850)
d=b.archive('c-sp330','https://www.nist.gov/pml/special-publication-330')
y=next((x['y'] for x in d['headings'] if 'International System' in x['text']),350);b.shot('21-nist-si-title.png',y-130,950)
