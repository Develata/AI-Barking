import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,shots='[]']=process.argv.slice(2);
if(!/^d-[a-z0-9-]+$/.test(name))throw Error('D prefix required');
const base='docs/2610/1006/';
const record={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:'anonymous headless Chrome via Playwright Core; CSS 700; DPR 2'};
let browser;
try{
 browser=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1006-d-chrome-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:700,height:1200},deviceScaleFactor:2,args:['--no-first-run','--no-default-browser-check']});
 const page=await browser.newPage();
 await page.goto(url,{waitUntil:'domcontentloaded',timeout:45000});await new Promise(r=>setTimeout(r,7000));
 const data=await page.evaluate(()=>({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time')].map(e=>e.outerHTML),jsonld:[...document.querySelectorAll('script[type="application/ld+json"]')].map(e=>e.textContent),links:[...document.querySelectorAll('a[href]')].map(e=>({text:e.innerText,url:e.href})),layout:[...document.querySelectorAll('h1,h2,h3,p,table')].map(e=>{const r=e.getBoundingClientRect();return {tag:e.tagName,text:e.innerText.slice(0,180),y:Math.round(r.y+scrollY),height:Math.round(r.height)}})}));
 data.captured_bj=record.time_bj;
 fs.writeFileSync(base+'sources/'+name+'.json',JSON.stringify(data,null,2),{flag:'wx'});
 fs.writeFileSync(base+'sources/'+name+'.txt',data.text,{flag:'wx'});
 record.resolved_shots=[];
 for(const [file,requestedY,requestedHeight] of JSON.parse(shots)){
  if(!/^(2[5-9]|3[0-4])-d-[a-z0-9-]+\.png$/.test(file))throw Error('D image range');
  if(fs.existsSync(base+'images/'+file))throw Error('Existing image');
  const area=typeof requestedY==='string'?await page.evaluate(s=>{const e=[...document.querySelectorAll('p')].find(e=>e.innerText.startsWith(s));if(!e)throw Error('Paragraph missing');const r=e.getBoundingClientRect();return {y:Math.max(0,Math.floor(r.y+scrollY)-18),height:Math.ceil(r.height)+36};},requestedY):{y:requestedY,height:requestedHeight};
  await page.screenshot({path:base+'images/'+file,clip:{x:0,y:area.y,width:700,height:area.height},fullPage:true});
  record.resolved_shots.push({file,...area});
 }
 record.result='saved; QA pending';record.chars=data.text.length;record.shots=JSON.parse(shots);
}catch(e){record.result='FAILED '+e.message.slice(0,400);}
finally{if(browser)await browser.close();fs.appendFileSync(base+'sources/d-browser-records.jsonl',JSON.stringify(record)+'\n');console.log(JSON.stringify(record));}
