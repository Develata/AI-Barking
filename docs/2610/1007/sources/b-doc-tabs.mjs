// Open docs model page, click WEIGHTS and USAGE tabs (read-only UI toggles), dump text. Writes b-doc-tabs.json/txt.
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const base='docs/2610/1007/sources/';
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-tabs-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:1400,height:1000}});
const p=await b.newPage();await p.goto('https://docs.mistral.ai/models/mistral-large-4-0',{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,8000));
const out={captured_bj:time_bj,tabs:{}};
for(const t of ['Features','Weights','Usage']){
 try{await p.getByRole('tab',{name:new RegExp('^'+t+'$','i')}).first().click({timeout:4000});await new Promise(r=>setTimeout(r,1500));
  out.tabs[t]=await p.evaluate(()=>{const e=document.querySelector('[role="tabpanel"][data-state="active"]')||document.body;return e.innerText.slice(0,4000)});}
 catch(e){out.tabs[t]='ERR '+String(e.message).slice(0,120);}
}
fs.writeFileSync(base+'b-doc-tabs.json',JSON.stringify(out,null,1),{flag:'wx'});
console.log(JSON.stringify(out,null,1).slice(0,3500));
await b.close();
