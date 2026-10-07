import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const base='docs/2610/1006/sources/';
const [name,url]=process.argv.slice(2);
if(!/^d-[a-z0-9-]+$/.test(name))throw Error('D prefix required');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
let rec={time_bj,url,tool:'curl.exe original host; anonymous; ordinary User-Agent',name};
try{
 const raw=execFileSync('curl.exe',['-L','--max-time','45','-sS','-A','Mozilla/5.0','-w','\nHTTP_STATUS:%{http_code}\nFINAL_URL:%{url_effective}',url],{encoding:'utf8',maxBuffer:18000000,timeout:50000});
 const end=raw.lastIndexOf('\nHTTP_STATUS:');
 const body=raw.slice(0,end);rec.transport=raw.slice(end).trim();
 fs.writeFileSync(base+name+'.html',body,{flag:'wx'});
 // Plain text is an extraction, not a substitute for visible-browser verification.
 let plain=body.replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi,'').replace(/<style\b[^>]*>[\s\S]*?<\/style>/gi,'').replace(/<\/(p|div|h[1-6]|li|tr|section|article)>/gi,'\n').replace(/<[^>]+>/g,' ').replace(/&nbsp;/g,' ').replace(/&amp;/g,'&').replace(/&quot;/g,'"').replace(/&#x27;|&#39;/g,"'").replace(/[ \t]+/g,' ').replace(/\n\s*\n/g,'\n');
 fs.writeFileSync(base+name+'.txt',plain,{flag:'wx'});
 rec.bytes=Buffer.byteLength(body);rec.result='saved; content requires manual validation';console.log(JSON.stringify(rec));
}catch(e){rec.result='FAILED '+e.message.slice(0,400);console.log(JSON.stringify(rec));}
fs.appendFileSync(base+'d-http-records.jsonl',JSON.stringify(rec)+'\n');
