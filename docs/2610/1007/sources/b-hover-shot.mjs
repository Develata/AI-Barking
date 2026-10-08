// Hover the price "sale" line on the docs page (desktop layout) and screenshot the card with the official tooltip visible.
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const base='docs/2610/1007/';const out=process.argv[2];const probe=process.argv[3]==='probe';
if(!probe&&!/^(1[5-9]|2[0-4])-b-[a-z0-9-]+\.png$/.test(out))throw Error('B image range');
if(!probe&&fs.existsSync(base+'images/'+out)&&!process.env.OVERWRITE)throw Error('exists');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-hs-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:1400,height:1000},deviceScaleFactor:Number(process.env.DPR||2)});
const p=await b.newPage();await p.goto('https://docs.mistral.ai/models/mistral-large-4-0',{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,8000));
const cards=await p.evaluate(()=>[...document.querySelectorAll('div.rounded-md.border')].filter(e=>e.getBoundingClientRect().height>0&&e.innerText.toUpperCase().includes('PRICE')).map(e=>{const r=e.getBoundingClientRect();return {x:r.x,y:r.y+scrollY,w:r.width,h:r.height,t:e.innerText.slice(0,60)}}));
console.log(JSON.stringify(cards));
const trig=await p.$$('[data-slot="tooltip-trigger"]');
await trig[8].hover();await new Promise(r=>setTimeout(r,900));
const tip=await p.evaluate(()=>[...document.querySelectorAll('[role="tooltip"]')].map(e=>e.innerText).join(' | '));
const tb=await p.evaluate(()=>{const e=document.querySelector('[data-radix-popper-content-wrapper]');if(!e)return null;const r=e.getBoundingClientRect();return {x:r.x,y:r.y+scrollY,w:r.width,h:r.height}});
console.log('tooltip',tip,JSON.stringify(tb));
if(!probe){
 const c=cards.sort((a,b)=>a.h-b.h)[0];
 const x=Math.max(0,Math.floor(c.x)-20);const y=Math.max(0,Math.floor(Math.min(c.y,tb?tb.y:c.y))-20);
 const width=Math.ceil(c.w)+40;const height=Math.ceil(Math.max(c.y+c.h,tb?tb.y+tb.h:0)-y)+20;
 await p.screenshot({path:base+'images/'+out,clip:{x,y,width,height},fullPage:true});
 fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify({time_bj,url:'https://docs.mistral.ai/models/mistral-large-4-0',name:'b-hover-shot',tool:'anonymous headless Chrome via Playwright Core; viewport 1400, DPR '+(process.env.DPR||2)+'; hover on sale-price line',tooltip:tip,resolved_shots:[{file:out,x,y,width,height}],result:'saved; QA pending'})+'\n');
}
await b.close();
