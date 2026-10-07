import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import crypto from 'node:crypto';
import {execFileSync} from 'node:child_process';
const base='docs/2610/1006';
const walk=p=>fs.readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(path.join(p,e.name)):[path.join(p,e.name)]);
const all=walk(base),ours=all.filter(p=>/^d-|^(evidence-d|capture-log-d)\.md$|^(2[5-9]|3[0-4])-d-/.test(path.basename(p)));
const ev=fs.readFileSync(base+'/sources/evidence-d.md','utf8');
const refs=[...ev.matchAll(/\.\.\/images\/([\w-]+\.png)/g)].map(m=>m[1]);
const missing=refs.filter(f=>!fs.existsSync(base+'/images/'+f));
const rows=ev.split('\n').filter(s=>/^\| D[1-4]-\d/.test(s));
const invalid_status=rows.filter(s=>!/(已找到|部分支持|与说法不符|未找到一手来源) \|$/.test(s));
const images=ours.filter(p=>p.endsWith('.png')).map(p=>{const b=fs.readFileSync(p);return {file:p.replaceAll('\\','/'),width:b.readUInt32BE(16),height:b.readUInt32BE(20)};});
const cfg=fs.readFileSync(path.join(os.homedir(),'.agent-reach/config.yaml'),'utf8');
const secrets=['twitter_auth_token','twitter_ct0'].map(k=>cfg.match(new RegExp('^'+k+':\\s*(.+)$','m'))?.[1]?.trim().replace(/^['"]|['"]$/g,'')).filter(Boolean);
const leaks=[];
for(const p of all.filter(p=>/\.(md|txt|json|jsonl|html|mjs|py|tsv)$/.test(p))){const s=fs.readFileSync(p,'utf8');if(secrets.some(v=>s.includes(v)))leaks.push(p.replaceAll('\\','/'));}
const normal=s=>s.replace(/&gt;/g,'>').replace(/\s+/g,' ').trim();
const quoteChecks=[
 ['d-mistral-browser.txt','ML4 was trained from scratch on 3,800 NVIDIA Grace Blackwell GPUs'],
 ['d-doc-browser.txt','49B active parameters and 1.05T total parameters'],
 ['d-aa.txt','The model weights are not publicly available.'],
 ['d-liquid-browser.txt','Methodology. We ran each application once per model on October 5, 2026'],
 ['d-liquid-browser.txt','we plan to release open weights for upcoming models on Hugging Face soon.'],
 ['d-dust-browser.txt','We do not attempt to make it compute-efficient enough to replace backprop today.'],
 ['d-bloomberg.txt','DeepSeek is close to securing at least 80 billion yuan ($12 billion)'],
 ['d-tnw-deepseek.txt','Founder Liang Wenfeng was the largest investor in that first round'],
 ['d-cnbc-deepseek.txt','DeepSeek is seeking a valuation of about 500 billion yuan']
].map(([file,quote])=>({file,quote,found:normal(fs.readFileSync(base+'/sources/'+file,'utf8')).includes(normal(quote))}));
const result={checked_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),rows:rows.length,missing,invalid_status,images,configured_secret_leak_files:leaks,quoteChecks};
fs.writeFileSync(base+'/sources/d-validation.json',JSON.stringify(result,null,2));
const files=walk(base).filter(p=>/^d-|^(evidence-d|capture-log-d)\.md$|^(2[5-9]|3[0-4])-d-/.test(path.basename(p))).filter(p=>!p.endsWith('d-manifest.md'));
const entries=files.sort().map(p=>{const b=fs.readFileSync(p);let ignored=false;try{execFileSync('git',['check-ignore','-q',p],{stdio:'ignore'});ignored=true;}catch{}return {file:path.relative(base,p).replaceAll('\\','/'),bytes:b.length,sha256:crypto.createHash('sha256').update(b).digest('hex'),ignored};});
fs.writeFileSync(base+'/sources/d-manifest.md','# D组新增文件清单\n\n本清单列D组文件，不认领A/B文件；含失败存档与判废截图，使用范围以capture-log-d.md为准。清单自身不做自引用SHA。\n\n| 文件 | 字节 | SHA-256 | Git忽略 |\n|---|---:|---|---|\n'+entries.map(e=>`| ${e.file} | ${e.bytes} | ${e.sha256} | ${e.ignored?'是':'否'} |`).join('\n')+'\n');
console.log(JSON.stringify({rows:result.rows,files:entries.length+1,missing,invalid_status,images:images.length,wide_images:images.filter(i=>i.width>1400),quote_failures:quoteChecks.filter(q=>!q.found),configured_secret_leak_files:leaks,large_nonignored:entries.filter(e=>!e.ignored&&e.bytes>1000000)},null,2));
