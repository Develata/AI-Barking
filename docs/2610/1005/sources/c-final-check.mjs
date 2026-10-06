import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
const root = path.resolve('docs/2610/1005');
const src = path.join(root, 'sources');
function clean(s) {
  return s.replace(/https?:\/\/[^\s<>"\\]+/g, raw => {
    try { const u = new URL(raw); for (const k of [...u.searchParams.keys()]) if (/^utm_|^ref_src$|^src$/.test(k)) u.searchParams.delete(k); return u.href; } catch { return raw; }
  });
}
function walk(v) {
  if (typeof v === 'string') return clean(v);
  if (Array.isArray(v)) return v.map(walk);
  if (v && typeof v === 'object') return Object.fromEntries(Object.entries(v).map(([k,x])=>[k,walk(x)]));
  return v;
}
for (const n of fs.readdirSync(src).filter(n=>n.startsWith('c-') && /\.jsonl?$/.test(n))) {
  const p=path.join(src,n), t=fs.readFileSync(p,'utf8');
  try {
    const out=n.endsWith('.jsonl') ? t.trim().split('\n').map(x=>JSON.stringify(walk(JSON.parse(x)))).join('\n') : JSON.stringify(walk(JSON.parse(t)),null,2);
    fs.writeFileSync(p,out+'\n');
  } catch(e) { throw new Error(`${n}: ${e.message}`); }
}
const ev=fs.readFileSync(path.join(src,'evidence-c.md'),'utf8');
const refs=[...new Set(ev.match(/\b(?:1[5-9]|2[0-4])-c-[a-z-]+\.png/g))];
const images=refs.map(n=>{
  const b=fs.readFileSync(path.join(root,'images',n));
  const w=b.readUInt32BE(16),h=b.readUInt32BE(20);
  if(w!==1400)throw new Error(`${n}: width ${w}`);
  return {name:n,width:w,height:h};
});
if(images.length!==10)throw new Error('Expected 10 screenshots');
const caveats=fs.readFileSync(path.join(src,'c-caveats-verbatim.md'),'utf8');
if((caveats.match(/^\d+\. \*\*/gm)||[]).length!==17)throw new Error('Expected 17 caveats');
const own=[];
for(const dir of ['sources','images'])for(const n of fs.readdirSync(path.join(root,dir)))if(n.startsWith('c-')||/^(evidence-c|capture-log-c)\.md$/.test(n)||/^(1[5-9]|2[0-4])-c-/.test(n))own.push(`${dir}/${n}`);
const states = new Set(['已找到','部分支持','与说法不符','未找到一手来源']);
for (const row of ev.split('\n').filter(x=>x.startsWith('| C'))) if(!states.has(row.split('|').at(-2).trim()))throw new Error('Invalid state');
const sensitive=/Develata|C:[/\\]Users[/\\]QQ|auth_token|sessionid|access_token|octolytics-actor|user-login/i;
for(const n of own.filter(n=>/\.(jsonl?|md|txt|html)$/.test(n)))if(sensitive.test(fs.readFileSync(path.join(root,n),'utf8')))throw new Error(`Privacy check: ${n}`);
console.log(JSON.stringify({images,caveat_count:17,reference_check:'PASS',state_check:'PASS',collector_pattern_check:'PASS',files:own.length},null,2));
const lines=['path\tbytes\tsha256'];
for(const n of own.sort().filter(n=>n!=='sources/c-file-manifest.tsv')) { const b=fs.readFileSync(path.join(root,n));lines.push(`${n}\t${b.length}\t${crypto.createHash('sha256').update(b).digest('hex')}`); }
fs.writeFileSync(path.join(src,'c-file-manifest.tsv'),lines.join('\n')+'\n');
