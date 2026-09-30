from pathlib import Path
import json,datetime,re,hashlib,collections,subprocess
from PIL import Image
P=Path('docs/0930/sources');R=P.parent; tz=datetime.timezone(datetime.timedelta(hours=8))
now=datetime.datetime.now(tz).isoformat()
records=[json.loads(s) for s in (P/'capture-records.jsonl').read_text(encoding='utf-8').splitlines() if s.strip()]
last={r.get('file'):i for i,r in enumerate(records) if r.get('file')}
def esc(x):return str(x).replace('|','\\|').replace('\n',' ')
log=['# 0930 抓取日志','',f'整理时间（北京时间）：{now}。请求日志时间为调用开始，截图为保存后记录；HTTP状态不是内容有效性判定。','', '所有引用URL均去除追踪查询参数；保留定位法规/API必需参数。失败响应原样作为诊断档，不作为事实证据。截图同名重截时文件为最后版本，早期成功仅表示写盘成功，以下标明已被替换。','', '| 北京时间 | URL / 最终URL | 工具 | 结果与存档标识 |','|---|---|---|---|']
for i,r in enumerate(records):
 st=r.get('status','失败');rid=r.get('id','')
 result=('成功（进程退出0）' if st==0 else '成功 HTTP200' if st==200 else '失败 HTTP'+str(st) if isinstance(st,int) else str(st))
 if rid=='c-law' and st==200:result='失败：HTTP200但重定向docnotfound，无条文'
 if rid=='b-zai-blog':result='部分失败：HTTP200仅598字节JS壳，未取得博客文章'
 if r.get('error'):result+='；'+r['error']
 if r.get('file') and last[r['file']]!=i:result+='；已被后续同名截图替换，不作为最终配图'
 result+='；'+rid
 if r.get('bytes') is not None:result+='；'+str(r['bytes'])+' bytes'
 u=r.get('url','—')
 if r.get('final_url') and r['final_url']!=u:u+=' → '+r['final_url']
 log.append('| '+' | '.join(map(esc,[r.get('time_bjt','未记录'),u,r.get('tool',''),result]))+' |')
log+=['','## 搜索留档与补记','', '以下搜索文件以文件修改时间作为本轮结果落盘时间，不冒充精确请求开始时间。搜索摘要只用作定位，原站另行抓取；原始查询字符串未全部单独留存，因此不声称搜索穷尽。','', '| 北京时间（结果文件mtime） | 服务/范围 | 工具 | 结果文件与结果 |','|---|---|---|---|']
for p in sorted(P.glob('*search*.txt')):
 txt=p.read_text(encoding='utf-8-sig'); ts=datetime.datetime.fromtimestamp(p.stat().st_mtime,tz).isoformat()
 urls=re.findall(r'^URL: (\S+)',txt,re.M)
 status='未返回结果' if 'No search results found' in txt else f'返回{len(urls)}个结果；仅作线索'
 log.append('| '+' | '.join(map(esc,[ts,'https://exa.ai/；'+('Z.ai官方X范围' if 'official-x' in p.name else '主题限定搜索'), 'agent-reach搜索路由 / Exa（CLI）',p.name+'；'+status]))+' |')
log+=['','## 未保存逐次机器时间的操作与失败','', '- 初始 opencli doctor 未连接浏览器扩展；本轮重启daemon后再次检查恢复。此诊断先于首批23:52:56 HTTP请求，未保存可核对的秒级时间。agent-reach doctor --json 已检查路由。', '- web.run 打开白宫行政令等页面时出现 Internal Error；另以原站 requests 成功存档。搜索工具失败不改用镜像。该批工具调用未另存秒级时间。', '- opencli 一次 Page.captureScreenshot 在60000ms后超时（约10/01 00:13–00:17北京时间）；改用本轮新浏览会话后恢复。未修改OpenCLI源码。', '- 白宫订阅弹窗在shadow root内；先前普通DOM按钮未能关闭。最终点击弹窗自身Close按钮，等待关闭动画后重截17/18/26；无订阅提交。', '- OpenAI/AA懒加载图表曾空白或文字被固定导航遮挡；滚动加载、展开Sol段并重截。最终图以最后同名日志及交付清单哈希为准。', '- OpenAI发布页/中文与英文DevDay HTTP403响应保留，事实取自原站浏览器JSON；官方API定价入口403，developers.openai.com官方文档成功。', '- d-b-gate HTTP403后原站浏览器成功；d-c-axios HTTP403本轮未恢复，未作为事实来源。', '- 美国法典granuleid查询返回docnotfound；改用同一官方站点title/section/edition查询成功。Z.ai博客仅JS壳，未取得正文。', '- 本地PDF检查先尝试pypdf但当前Python无法导入；改用已安装pdftotext提取PDF前4页，c-sp330-cover-text.txt含2019版及美国商务部署名。纯本地操作，不是再次网络抓取。', '- 截图06/07最终按比例缩至1400px宽，其他图宽1266px；13图裁掉图注以外底部重复渲染区域，保留完整图和图注。不重绘、不改数字。', '', '## 时间和重试说明', '', '本期0930是编辑期号，抓取实际跨北京时间9/30与10/01。发布日期只按页面/JSON-LD原有时区换算；无时区不自行补时区。热度值各自按日志时间使用，不把不同时间读数拼为同一快照。']
(P/'capture-log.md').write_text('\n'.join(log)+'\n',encoding='utf-8')
# Mechanical delivery validation.
t=(P/'evidence.md').read_text(encoding='utf-8'); ids=re.findall(r'^\| ([ABC]\d+) ',t,re.M)
expected=[f'{g}{i}' for g,n in [('A',9),('B',8),('C',7)] for i in range(1,n+1)]
assert ids==expected,(ids,expected)
rows=[s for s in t.splitlines() if re.match(r'^\| [ABC]\d+ ',s)]
assert all(re.search(r'\| (已找到|部分支持|与说法不符|未找到一手来源) \|$',s) for s in rows)
refs=re.findall(r'\]\(\.\./images/([^)]*)\)',t); assert all((R/'images'/x).is_file() for x in refs)
imgs=sorted((R/'images').glob('*.png'));assert [p.name[:2] for p in imgs]==[f'{i:02d}' for i in range(1,len(imgs)+1)]
assert all(Image.open(p).width<=1400 for p in imgs)
assert not (R/'images/README.md').exists() and not (P/'fact-check.md').exists()
status=subprocess.check_output(['git','status','--short'],text=True,encoding='utf-8')
assert set(status.splitlines())=={'?? .handoff/2026-09-30-0930-evidence.md','?? .workbuddy/','?? docs/0930/'},status
q=['# 0930 交付核验', '',f'核验时间：{now}', '',f'- A1–A9、B1–B8、C1–C7共{len(rows)}行，逐项状态均在约定词表内。',f'- {len(imgs)}张连续编号PNG，宽度均≤1400px；事实表全部截图链接存在。', '- 图表截图经视觉检查；White House遮挡及OpenAI/AA懒加载问题已重截处理。06/07仅等比缩小，13仅移除图注以外重复区域。','- 公开页面抓取；社交数值只保存公开帖子字段，不含个人账号导航/回复框。交付前rg对已知维护者handle、邮箱与本机用户路径无匹配；截图未出现抓取者头像、显示名、handle。', '- 本次未生成发布稿、images/README.md、fact-check.md或ZIP。检查时发现本轮其他进程/协作者新增根目录doc_0930_publish.txt（创建北京时间9/30 23:57:54、最后写入23:58:13），未修改，排除本次交付清单。未复现任何模型评测。', '- git status范围符合约定；基线已有派工单和.workbuddy保持不动；无commit/push。', '', '```text', status.rstrip(), '```', '', '未找到/部分支持项目、历史价格证据边界及搜索范围见evidence.md。截图成功不等于独立验证厂商能力。']
(P/'a-delivery-checks.md').write_text('\n'.join(q)+'\n',encoding='utf-8')
manifest=P/'a-delivery-manifest.md';manifest.touch(exist_ok=True)
files=sorted(p for folder in [P,R/'images'] for p in folder.rglob('*') if p.is_file())
out=['# 0930 新增文件清单','',f'共{len(files)}个文件（含本清单）；{len(imgs)}张PNG。所有路径均在docs/0930/sources或images内；排除非本任务生成的doc_0930_publish.txt。','', 'evidence.md为主事实清单；capture-log.md为抓取日志。HTTP403/软失败档与重试脚本为过程记录，不能拿它们当正文证据。HTML为抓取响应存档，并非可完全离线重放网页；浏览器JSON保存公开正文与元数据。','', '| 文件 | 字节 | SHA-256 |','|---|---:|---|']
for p in files:
 rel=p.relative_to(R).as_posix();link=('../'+rel)
 sha='自身不计算' if p==manifest else hashlib.sha256(p.read_bytes()).hexdigest()
 out.append(f'| [{rel}]({link}) | {p.stat().st_size if p!=manifest else "—"} | {sha} |')
manifest.write_text('\n'.join(out)+'\n',encoding='utf-8')
print(json.dumps({'evidence_rows':len(rows),'images':len(imgs),'files':len(files),'capture_records':len(records),'status':status},ensure_ascii=False))
