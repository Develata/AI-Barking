from pathlib import Path
import json,re,datetime
P=Path('docs/2610/1002/sources');I=P.parent/'images'
# Final evidence corrections, no change to captured source content.
p=P/'c-evidence.md';s=p.read_text(encoding='utf-8');s=s.replace('宽上限按任务指定模板的CSS像素口径（1100 CSS px对应2200物理px','宽上限按任务指定模板的CSS像素口径（最大1400 CSS px对应2800物理px，常用1100 CSS px对应2200物理px');s=s.replace('两处皆存，不擅选一个消除差异。','两处皆存，不擅选一个消除差异。图44为Space README归属，图45为应用标题；榜单数据全文已存，但完整榜表截图三次CDP超时，缺该配图。');s=s.replace('图片44','图片44');p.write_text(s,encoding='utf-8')
for p in P.glob('*.mjs'):
 s=p.read_text(encoding='utf-8-sig');old=next(iter(re.findall(r"import \{sendCommand\} from 'file:///[^']+/daemon-client\.js';",s)), '')
 if old and old in s:
  s=s.replace(old,"import {homedir} from 'node:os';\nimport {pathToFileURL} from 'node:url';\nimport {join} from 'node:path';\nconst {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);")
  p.write_text(s,encoding='utf-8')
# Join A/B/C reports, preserving verbatim excerpts and group provenance.
head='''# 1002 取证事实清单

仅取证，不是发布正文或最终编辑结论。A1–A10、B1–B10、C1–C11共31条。状态仅使用已找到/部分支持/与说法不符/未找到一手来源；媒体与社交级别不因存档成功提升。文内URL是出处；本地文件与截图见文件清单，抓取时间见capture-log.md。

假设与边界：日期均以北京时间说明，官方仅日期时不臆造时刻；截图尺寸依指定templates/codex-evidence.md按CSS像素计，DPR2，最大1400CSS（2800物理像素）。没有模型实测、训练复现或统计抽样代表性声明。HTTP403/错误页面只作为失败记录，不是成功证据。全文抓取与截图完成度分别标记。

'''
parts=[]
for g in 'abc':
 s=(P/(g+'-evidence.md')).read_text(encoding='utf-8');s=re.sub(r'^# .+\n','## '+g.upper()+'组\n',s,count=1);parts.append(s)
scan='''
## 扫描说法勘误（集中索引）

| 扫描说法 | 本次独立存档复核 |
|---|---|
| 在轨运行Gemini推理 | 未获Google/Planet支持。Google仅联系成功、后续数周采集；Planet为will run，NPR为Gemma且will run。Cocoloop实际转述见A10。 |
| 五年等效辐射在轨通过 | 与Google原文不符，明确UC Davis地面质子束；Joule剂量不是在轨五年实测。 |
| 路透100+与OpenAI数十家数字冲突 | 不能由两数字不同直接成立；本次同页保留旧dozens、9/30更新as of9/26 over100。见B1/B2。 |
| 8/26报告dozens也是机构数 | 与原文不符，指Hugging Face servers。 |
| 加州是美国首次针对失控agent执法 | Guardian该句紧跟FTC调查，指FTC；加州稿说ongoing investigation。FTC本次原始公开文件未找到，不能自行确认为6(b)或法律性质。 |
| AISI页面没有日期 | 与当前页面Oct1,2026不符；官方未给时刻不等于没给日期。 |
| Jev Decision Index是CF自建基准 | 与HF Space作者、README及CF demo说明不符；社区作者multimodalart，CF自家demo另站。 |
| CF对比表数字全是CF自跑 | 需进一步限缩：CF两款自报，上游非CF数据沿用社区9/28快照（C2/C8）。 |

## 交付缺口与失败集中说明

- NPR四TPU/15分钟/Gemma/冰箱参数段没有逐项署名出处；未找到Google/Planet/Joule相应一手句，保持L4。
- Joule HTML全文已取得，但PDF403，页码未取得，以节与文本行定位。SpaceX Starmind2027Q4未找到一手。
- FTC本次调查公开原始文件、加州9月独立官宣稿、OpenAI9/30独立文章URL未找到；不把旧6(b)研究混入。
- B7未凑到两家符合指定误读的实际文章；C11未找到指定强夸大实例。详见逐稿抽查，不把搜索摘要当文章全文。
- Google/Google Research官方X与本次OpenAI更新X未取得；C组X搜索失败。Reddit搜索失败后公开原帖取得，计数有时点。
- Clef延迟具体GPU/并发/网络边界仍未找到；社区榜明确Jev含HTTPS，CF明确own serving stack不可直接比较。
- C4完整榜表截图多次原生CDP超时，保留作者README和应用标题截图及完整DOM数据；不把不全的横向图冒充完整榜图。C2主质量表10行、工作流4行、延迟2行全部截图与抄录已齐。
- 未精确即时记录的手工抓取以文件mtime补记，日志明确区分；不伪造秒级抓取时刻。截图所有引用均检查实际文件。
'''
(P/'evidence.md').write_text(head+'\n\n'.join(parts)+scan,encoding='utf-8')
# Continuous filenames for newly created assets only; preserve all contents, no deletion.
mapping={}
for i,p in enumerate(sorted(I.glob('*.png')),1):
 new=f'{i:02d}-'+p.name.split('-',1)[1]
 if new!=p.name:
  q=I/new
  assert p.resolve().parent==I.resolve() and q.resolve().parent==I.resolve() and not q.exists()
  mapping[p.name]=new;p.rename(q)
for p in P.iterdir():
 if p.suffix in ['.md','.jsonl','.py','.mjs']:
  s=p.read_text(encoding='utf-8-sig')
  if any(k in s for k in mapping):
   s=re.sub('|'.join(map(re.escape,mapping)),lambda m:mapping[m[0]],s);p.write_text(s,encoding='utf-8')
(P/'d-image-renumbering.json').write_text(json.dumps(mapping,ensure_ascii=False,indent=2),encoding='utf-8')
print('31 rows assembled; images',len(list(I.glob('*.png'))))
