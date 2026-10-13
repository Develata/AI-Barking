// 1013 B 组：探测 Anthropic 文章页布局（匿名 headless Chrome）。列出标题/段落/iframe/img/video 的位置，供截图定界。
// 用法（仓库根目录）：node docs/2610/1013/sources/b-probe.mjs <cssWidth>
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const W=Number(process.argv[2]||'1000');
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-bp-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1000},locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',deviceScaleFactor:2});
const p=await b.newPage();
await p.goto('https://www.anthropic.com/research/self-replicating-ai',{waitUntil:'domcontentloaded',timeout:60000});
await new Promise(r=>setTimeout(r,4000));
const H=await p.evaluate(()=>document.documentElement.scrollHeight);
for(let y=0;y<H;y+=600){await p.evaluate(y=>scrollTo(0,y),y);await new Promise(r=>setTimeout(r,350));}
await p.evaluate(()=>scrollTo(0,0));await new Promise(r=>setTimeout(r,1500));
const rows=await p.evaluate(()=>{
 const out=[];const art=document.querySelector('article')||document.body;
 for(const e of art.querySelectorAll('h1,h2,h3,h4,p,iframe,img,video,figure,figcaption,table,li')){
  const r=e.getBoundingClientRect();if(r.height<2)continue;
  out.push({tag:e.tagName,y:Math.round(r.y+scrollY),h:Math.round(r.height),x:Math.round(r.x),w:Math.round(r.width),t:(e.innerText||e.alt||e.src||'').trim().slice(0,70).replace(/\n/g,' ')});
 }
 return out;});
console.log('scrollHeight',H);
for(const r of rows)console.log(`${r.tag}\ty=${r.y}\th=${r.h}\tx=${r.x}\tw=${r.w}\t${r.t}`);
await b.close();
