import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const dir='docs/2610/1005/sources';
const [mode,name,url]=process.argv.slice(2);
const bj=()=>new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const rec={time_bj:bj(),url,tool:mode};
try {
 let data;
 if(mode==='browser'){
  execFileSync('opencli.exe',['browser','1005-c','open',url],{encoding:'utf8',timeout:90000});
  const js='JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll("meta[name],meta[property],time")].map(x=>x.outerHTML),links:[...document.querySelectorAll("a")].map(x=>({text:x.innerText,url:x.href}))})';
  const out=execFileSync('opencli.exe',['browser','1005-c','eval',js],{encoding:'utf8',timeout:90000,maxBuffer:16000000});
  data=JSON.parse(out.split('\n')[0]);
  fs.writeFileSync(`${dir}/c-${name}.json`,JSON.stringify(data,null,2),{flag:'wx'});
  fs.writeFileSync(`${dir}/c-${name}.txt`,data.text,{flag:'wx'});
  rec.result=`Captured ${data.text.length} characters: ${data.title}`;
 } else {
  const out=execFileSync('curl.exe',['-L','--max-time','60','-sS','-A','Mozilla/5.0',url],{maxBuffer:24000000});
  fs.writeFileSync(`${dir}/c-${name}`,out,{flag:'wx'});
  rec.result=`Downloaded ${out.length} bytes; body requires inspection`;
 }
}catch(e){rec.result='FAILED '+e.message.slice(0,500)}
fs.appendFileSync(`${dir}/c-capture-records.jsonl`,JSON.stringify(rec)+'\n');
console.log(JSON.stringify(rec));
