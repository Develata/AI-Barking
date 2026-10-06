import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const dir='docs/2610/1005/sources';
const rows=fs.readFileSync(`${dir}/a-wikimedia-edits.csv`,'utf8').trim().split(/\r?\n/).map((url,i)=>{const u=new URL(url);return {line:i+1,url,host:u.host,title:u.searchParams.get('title'),revid:Number(u.searchParams.get('oldid')||u.searchParams.get('diff'))};});
for(const host of [...new Set(rows.map(x=>x.host))]){
 const sub=rows.filter(x=>x.host===host); const u=new URL(`https://${host}/w/api.php`);Object.entries({action:'query',format:'json',prop:'revisions',revids:sub.map(x=>x.revid).join('|'),rvprop:'ids|timestamp'}).forEach(([k,v])=>u.searchParams.set(k,v));
 const at=new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00');
 try{const raw=execFileSync('curl.exe',['-sS','-L','--max-time','30','-A','Mozilla/5.0',u.href],{encoding:'utf8',timeout:35000});const data=JSON.parse(raw);fs.writeFileSync(`${dir}/a-revisions-${host}.json`,JSON.stringify(data,null,2));
 for(const p of Object.values(data.query?.pages||{}))for(const r of p.revisions||[]){const row=sub.find(x=>x.revid===r.revid);if(row)Object.assign(row,{resolved_title:p.title,ns:p.ns,timestamp:r.timestamp});}
 fs.appendFileSync(`${dir}/a-fetch-log.jsonl`,JSON.stringify({at,url:u.href,tool:'curl MediaWiki public API; rvprop ids|timestamp only',result:'JSON received',host})+'\n');
 }catch(e){fs.appendFileSync(`${dir}/a-fetch-log.jsonl`,JSON.stringify({at,url:u.href,tool:'curl MediaWiki public API',result:'failed',error:String(e.message).slice(0,250)})+'\n');}
 console.log(host,sub.length,sub.filter(x=>x.timestamp).length);
}
fs.writeFileSync(`${dir}/a-csv-statistics.json`,JSON.stringify({rows:rows.length,hosts:Object.fromEntries([...new Set(rows.map(x=>x.host))].map(h=>[h,rows.filter(x=>x.host===h).length])),resolved:rows.filter(x=>x.timestamp).length,records:rows},null,2));
