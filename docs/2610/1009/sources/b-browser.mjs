// 1009 B group anonymous (adapted from 1007 b-browser.mjs) headless Chrome capture (Playwright Core, no login, fresh temp profile).
// Usage: node b-browser.mjs <name> <url> <cssWidth> <shotsJSON> [waitMs] [initJS]
// shots item: [file, startText, endText|null, padTop, padBottom]  (clip from top of element whose text starts with startText
//   to bottom of element whose text starts with endText (or the start element)); or [file, "y", y, height].
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,width='700',shotsArg='[]',waitArg='7000',initJS='']=process.argv.slice(2);
if(!/^b-[a-z0-9-]+$/.test(name))throw Error('B prefix required');
const base='docs/2610/1009/';
const W=Number(width);
const record={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:`anonymous headless Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR ${process.env.DPR||2}`};
let browser;
try{
 browser=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1009-b-chrome-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1200},deviceScaleFactor:Number(process.env.DPR||2),args:['--no-first-run','--no-default-browser-check']});
 const page=await browser.newPage();
 await page.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 if(initJS){await page.evaluate(initJS);await new Promise(r=>setTimeout(r,1500));}
 const data=await page.evaluate(()=>({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time')].map(e=>e.outerHTML),links:[...document.querySelectorAll('a[href]')].map(e=>({text:e.innerText.slice(0,100),url:e.href})),layout:[...document.querySelectorAll('h1,h2,h3,p,table,img,figure,del,ins')].map(e=>{const r=e.getBoundingClientRect();const cs=getComputedStyle(e);return {tag:e.tagName,text:(e.innerText||e.alt||e.currentSrc||'').slice(0,160),y:Math.round(r.y+scrollY),x:Math.round(r.x),w:Math.round(r.width),h:Math.round(r.height),vis:cs.display!=='none'&&cs.visibility!=='hidden'}})}));
 data.captured_bj=record.time_bj;
 if(!process.env.NOARCHIVE){fs.writeFileSync(base+'sources/'+name+'.json',JSON.stringify(data,null,2),{flag:'wx'});
 fs.writeFileSync(base+'sources/'+name+'.txt',data.text,{flag:'wx'});}
 record.chars=data.text.length;record.resolved_shots=[];
 for(const s of JSON.parse(shotsArg)){
  const file=s[0];
  if(!/^(1[5-9]|2[0-4])-[a-z0-9-]+\.png$/.test(file))throw Error('B image range 15-24 only: '+file);
  if(fs.existsSync(base+'images/'+file)&&!process.env.OVERWRITE)throw Error('Existing image '+file);
  let area;
  if(s[1]==='y')area={y:s[2],height:s[3],x:0,width:W};
  else area=await page.evaluate(([a,b,pt,pb])=>{
   const find=t=>[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,img,table,section')].filter(e=>(e.innerText||e.alt||'').trim().startsWith(t)).sort((p,q)=>p.getBoundingClientRect().height-q.getBoundingClientRect().height)[0];
   const e1=find(a);if(!e1)throw Error('start missing: '+a);const e2=b?find(b):e1;if(!e2)throw Error('end missing: '+b);
   const r1=e1.getBoundingClientRect(),r2=e2.getBoundingClientRect();
   const top=Math.max(0,Math.floor(r1.y+scrollY)-pt),bot=Math.ceil(r2.y+scrollY+r2.height)+pb;
   return {y:top,height:bot-top,x:0,width:innerWidth};
  },[s[1],s[2],s[3]??18,s[4]??18]);
  await page.screenshot({path:base+'images/'+file,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  record.resolved_shots.push({file,...area});
 }
 record.result='saved; QA pending';
}catch(e){record.result='FAILED '+String(e.message).slice(0,400);}
finally{if(browser)await browser.close();fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify(record)+'\n');console.log(JSON.stringify(record));}
