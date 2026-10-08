// Click the WEIGHTS tab on the docs model page (UI toggle only) and screenshot tab row + table. Usage: node b-doc-weights-shot.mjs <outfile> [cssWidth]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const base='docs/2610/1007/';const [out,W='700']=process.argv.slice(2);
if(!/^(1[5-9]|2[0-4])-b-[a-z0-9-]+\.png$/.test(out))throw Error('B image range');
if(fs.existsSync(base+'images/'+out))throw Error('exists');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-wt-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:Number(W),height:1000},deviceScaleFactor:2});
const p=await b.newPage();await p.goto('https://docs.mistral.ai/models/mistral-large-4-0',{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,8000));
await p.getByRole('tab',{name:/^weights$/i}).first().click({timeout:4000});await new Promise(r=>setTimeout(r,1500));
const g=await p.evaluate(()=>{const tab=[...document.querySelectorAll('[role="tab"]')].find(e=>/weights/i.test(e.innerText));const panel=document.querySelector('[role="tabpanel"][data-state="active"]');const a=tab.getBoundingClientRect(),c=panel.getBoundingClientRect();return {y:a.y+scrollY,bottom:c.y+scrollY+c.height,txt:panel.innerText.replace(/\n/g,' | ').slice(0,300)}});
console.log(JSON.stringify(g));
const y=Math.max(0,Math.floor(g.y)-20),h=Math.ceil(g.bottom-g.y)+40;
await p.screenshot({path:base+'images/'+out,clip:{x:0,y,width:Number(W),height:h},fullPage:true});
fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify({time_bj,url:'https://docs.mistral.ai/models/mistral-large-4-0',name:'b-doc-weights-shot',tool:`anonymous headless Chrome via Playwright Core; CSS ${W}; DPR 2; clicked WEIGHTS tab`,panel:g.txt,resolved_shots:[{file:out,y,height:h}],result:'saved; QA pending'})+'\n');
await b.close();
