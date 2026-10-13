// 1012 C 组：探测页面尺寸/文本锚点位置（不写 images）。用法：node docs/2610/1012/sources/c-shot-probe.mjs <url> <cssWidth> <waitMs> [needle1|needle2...]
import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,w='700',wait='5000',needles='']=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1012-c-probe-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:Number(w),height:1200},deviceScaleFactor:2});
const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(wait)));
if(process.env.SCROLL){await p.evaluate(async()=>{for(let y=0;y<document.documentElement.scrollHeight;y+=500){window.scrollTo(0,y);await new Promise(r=>setTimeout(r,250));}window.scrollTo(0,0);await new Promise(r=>setTimeout(r,2500));});}
const r=await p.evaluate((needles)=>{const out={title:document.title,h:document.documentElement.scrollHeight,found:{}};
 for(const n of needles.split('|').filter(Boolean)){const els=[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,a,blockquote')].filter(e=>(e.innerText||'').includes(n)).filter(e=>e.getBoundingClientRect().height>0).sort((a,b)=>a.getBoundingClientRect().height-b.getBoundingClientRect().height);
  out.found[n]=els.slice(0,2).map(e=>{const r=e.getBoundingClientRect();return {y:Math.round(r.y+scrollY),h:Math.round(r.height),t:(e.innerText||'').slice(0,80)}});}
 return out;},needles);
console.log(JSON.stringify(r,null,1));await b.close();
