// 1009 C group anonymous (adapted from 1007 b-browser.mjs) headless Chrome capture (Playwright Core, no login, fresh temp profile).
// Usage: node b-browser.mjs <name> <url> <cssWidth> <shotsJSON> [waitMs] [initJS]
// shots item: [file, startText, endText|null, padTop, padBottom]  (clip from top of element whose text starts with startText
//   to bottom of element whose text starts with endText (or the start element)); or [file, "y", y, height].
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,width='700',shotsArg='[]',waitArg='7000',initJS='']=process.argv.slice(2);
if(!/^c-[a-z0-9-]+$/.test(name))throw Error('C prefix required');
const base='docs/2610/1009/';
const W=Number(width);
const record={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:`anonymous ${process.env.HEADED?"headed":"headless"} Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR ${process.env.DPR||2}`};
let browser;
try{
 browser=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1009-c-chrome-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:process.env.HEADED?false:true,viewport:{width:W,height:1200},deviceScaleFactor:Number(process.env.DPR||2),args:['--no-first-run','--no-default-browser-check']});
 const page=await browser.newPage();
 await page.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 if(process.env.SCROLL){const H=await page.evaluate(()=>document.documentElement.scrollHeight);for(let y=0;y<H;y+=300){await page.evaluate(v=>window.scrollTo(0,v),y);await new Promise(r=>setTimeout(r,Number(process.env.SCROLL)>1?Number(process.env.SCROLL):250));}await page.evaluate(()=>window.scrollTo(0,0));await new Promise(r=>setTimeout(r,2500));}
 if(initJS){await page.evaluate(initJS);await new Promise(r=>setTimeout(r,1500));}
 const data=await page.evaluate(()=>({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time')].map(e=>e.outerHTML),links:[...document.querySelectorAll('a[href]')].map(e=>({text:e.innerText.slice(0,100),url:e.href})),layout:[...document.querySelectorAll('h1,h2,h3,p,table,img,figure,del,ins')].map(e=>{const r=e.getBoundingClientRect();const cs=getComputedStyle(e);return {tag:e.tagName,text:(e.innerText||e.alt||e.currentSrc||'').slice(0,160),y:Math.round(r.y+scrollY),x:Math.round(r.x),w:Math.round(r.width),h:Math.round(r.height),vis:cs.display!=='none'&&cs.visibility!=='hidden'}})}));
 data.captured_bj=record.time_bj;
 if(!process.env.NOARCHIVE){fs.writeFileSync(base+'sources/'+name+'.json',JSON.stringify(data,null,2),{flag:'wx'});
 fs.writeFileSync(base+'sources/'+name+'.txt',data.text,{flag:'wx'});}
 record.chars=data.text.length;record.resolved_shots=[];
 for(const s of JSON.parse(shotsArg)){
  const file=s[0];
  if(!/^(2[5-9]|3[0-4])-[a-z0-9-]+\.png$/.test(file))throw Error('C image range 25-34 only: '+file);
  if(fs.existsSync(base+'images/'+file)&&!process.env.OVERWRITE)throw Error('Existing image '+file);
  let area;
  if(process.env.JUMP){if(s[1]==='y')await page.evaluate(v=>window.scrollTo(0,v-200),s[2]);else await page.evaluate(a=>{const f=[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,img,table,section,summary')].filter(e=>(e.innerText||e.alt||'').trim().startsWith(a)).sort((p,q)=>p.getBoundingClientRect().height-q.getBoundingClientRect().height)[0];if(f)f.scrollIntoView({block:'start'});},s[1]);await new Promise(r=>setTimeout(r,6000));}
  if(s[1]==='y')area={y:s[2],height:s[3],x:0,width:W};
  else area=await page.evaluate(([a,b,pt,pb])=>{
   const find=t=>[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,img,table,section,summary')].filter(e=>(e.innerText||e.alt||'').trim().startsWith(t)).sort((p,q)=>p.getBoundingClientRect().height-q.getBoundingClientRect().height)[0];
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
finally{if(browser)await browser.close();fs.appendFileSync(base+'sources/c-browser-records.jsonl',JSON.stringify(record)+'\n');console.log(JSON.stringify(record));}
