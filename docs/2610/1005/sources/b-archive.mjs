import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const dir='docs/2610/1005/sources/';
const [name,url]=process.argv.slice(2);
if(!/^b-[a-z0-9-]+$/.test(name)) throw Error('B prefix required');
const bj=()=>new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const started=bj();
const cli=(...args)=>execFileSync('opencli.exe',['browser','1005-b',...args],{encoding:'utf8',timeout:120000});
try {
 cli('open',url);
 await new Promise(r=>setTimeout(r,1500));
 const out=cli('eval',`JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time,link[rel=canonical]')].map(e=>e.outerHTML),links:[...document.querySelectorAll('main a,article a')].map(a=>({text:a.innerText,url:a.href}))})`);
 const data=JSON.parse(out.split('\n  Update available:')[0].trim());
 if(!data.text || data.text.length<100) throw Error('Empty/short body: '+JSON.stringify(data));
 data.captured_bj=bj();
 data.links=data.links.map(a=>{try{let u=new URL(a.url); for(const k of [...u.searchParams.keys()]) if(k.startsWith('utm_'))u.searchParams.delete(k);a.url=u.href;}catch{}return a;});
 fs.writeFileSync(dir+name+'.json',JSON.stringify(data,null,2));
 fs.writeFileSync(dir+name+'.txt',data.text);
 fs.appendFileSync(dir+'b-captures.jsonl',JSON.stringify({started_bj:started,url,tool:'opencli browser 1005-b open/eval',result:data.title,chars:data.text.length,file:name+'.json'})+'\n');
 console.log(name,data.title,data.text.length);
}catch(e){fs.appendFileSync(dir+'b-captures.jsonl',JSON.stringify({started_bj:started,url,tool:'opencli browser 1005-b open/eval',error:e.message.slice(0,1000)})+'\n');console.error(e.message);process.exitCode=1;}
