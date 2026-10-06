import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const dir='docs/2610/1005/sources';
const urls=process.argv.slice(2);
const now=()=>new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00');
for(let i=0;i<urls.length;i+=2){
 const [name,url]=urls.slice(i,i+2); const at=now();
 if(!/^a-[\w-]+$/.test(name)||fs.existsSync(`${dir}/${name}.json`))throw Error('invalid/existing destination');
 try{
  execFileSync('opencli.exe',['browser','1005-a','open',url],{encoding:'utf8',timeout:90000,stdio:['ignore','pipe','pipe']});
  const js=`JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time')].map(x=>x.outerHTML),links:[...document.querySelectorAll('main a,article a,#mw-content-text a')].map(x=>({text:x.innerText,url:x.href}))})`;
  const raw=execFileSync('opencli.exe',['browser','1005-a','eval',js],{encoding:'utf8',timeout:90000,stdio:['ignore','pipe','pipe']}).split('\n  Update available:')[0].trim();
  const data=JSON.parse(raw); data.captured_bj=at;
  data.links=data.links.filter(x=>!/(login|logout|gift|subscribe|share=)/i.test(x.url)).map(x=>{try{let u=new URL(x.url);for(const k of [...u.searchParams.keys()])if(k.startsWith('utm_'))u.searchParams.delete(k);return {...x,url:u.href};}catch{return x;}});
  fs.writeFileSync(`${dir}/${name}.json`,JSON.stringify(data,null,2));fs.writeFileSync(`${dir}/${name}.txt`,data.text);
  const rec={at,url,tool:'opencli browser 1005-a open/eval',file:name,result:'captured; content requires review',chars:data.text.length,title:data.title};
  fs.appendFileSync(`${dir}/a-fetch-log.jsonl`,JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));
 }catch(e){const rec={at,url,tool:'opencli browser 1005-a open/eval',file:name,result:'failed',error:String(e.message).slice(0,700)};fs.appendFileSync(`${dir}/a-fetch-log.jsonl`,JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
}
