// B group HTTP archiver: anonymous curl, ordinary User-Agent. Usage: node b-http.mjs <name> <url> [ext]
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const base='docs/2610/1007/sources/';
const [name,url,ext='html']=process.argv.slice(2);
if(!/^b-[a-z0-9-]+$/.test(name))throw Error('B prefix required');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
let rec={time_bj,url,tool:'curl.exe original host; anonymous; ordinary User-Agent',name};
try{
 const raw=execFileSync('curl.exe',['-L','--max-time','60','-sS','-A','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36','-w','\nHTTP_STATUS:%{http_code}\nFINAL_URL:%{url_effective}',url],{encoding:'utf8',maxBuffer:60000000,timeout:70000});
 const end=raw.lastIndexOf('\nHTTP_STATUS:');
 const body=raw.slice(0,end);rec.transport=raw.slice(end).trim().replace(/\n/g,' | ');
 fs.writeFileSync(base+name+'.'+ext,body,{flag:'wx'});
 if(ext==='html'){
  let plain=body.replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi,'').replace(/<style\b[^>]*>[\s\S]*?<\/style>/gi,'').replace(/<\/(p|div|h[1-6]|li|tr|section|article)>/gi,'\n').replace(/<[^>]+>/g,' ').replace(/&nbsp;/g,' ').replace(/&amp;/g,'&').replace(/&quot;/g,'"').replace(/&#x27;|&#39;/g,"'").replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/[ \t]+/g,' ').replace(/\n\s*\n/g,'\n');
  fs.writeFileSync(base+name+'.txt',plain,{flag:'wx'});
 }
 rec.bytes=Buffer.byteLength(body);rec.result='saved; content requires manual validation';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,400);}
fs.appendFileSync(base+'b-http-records.jsonl',JSON.stringify(rec)+'\n');
console.log(JSON.stringify(rec));
