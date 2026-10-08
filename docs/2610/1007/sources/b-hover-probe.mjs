import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-hover-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:1400,height:1000}});
const p=await b.newPage();await p.goto('https://docs.mistral.ai/models/mistral-large-4-0',{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,8000));
const out=[];
for(const sel of ['span:has-text("Sale price")','[data-slot="tooltip-trigger"]']){
 const els=await p.$$(sel);out.push({sel,n:els.length});
 for(let i=0;i<Math.min(els.length,14);i++){
  try{await els[i].hover({timeout:2000});await new Promise(r=>setTimeout(r,700));
   const t=await p.evaluate(()=>[...document.querySelectorAll('[role="tooltip"],[data-radix-popper-content-wrapper]')].map(e=>e.innerText).join(' | '));
   const lbl=await els[i].evaluate(e=>e.innerText.slice(0,40)+' / '+(e.parentElement?.innerText||'').slice(0,60));
   out.push({i,lbl,tip:t});}catch(e){out.push({i,err:String(e.message).slice(0,80)});}
 }
}
console.log(JSON.stringify(out,null,1));
await b.close();
