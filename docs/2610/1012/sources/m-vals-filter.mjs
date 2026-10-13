// 1013：Vals 榜单按模型名过滤后截图（匿名 headless Chrome，全新临时 profile、不登录、DPR 2、700 CSS px）。
// 环境变量 HEAD：榜单卡片标题里的一段文字（向上找到包含它的容器作为截图范围）。
// 用法：node docs/2610/1013/sources/m-vals-filter.mjs <url> <filterText> <out.png> [probe]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,filt,out,probe]=process.argv.slice(2);
const base='docs/2610/1013/';
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; DPR 2; 700 CSS px; table search filter',url,filter:filt,out};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-v-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',viewport:{width:+(process.env.VW||700),height:1400},deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});
 await new Promise(r=>setTimeout(r,8000));
 if(probe){console.log(await p.evaluate(()=>[...document.querySelectorAll('input,button,[role=searchbox]')].slice(0,40).map(e=>e.tagName+'|'+(e.getAttribute('placeholder')||'')+'|'+(e.getAttribute('aria-label')||'')+'|'+e.textContent.trim().slice(0,30)).join('\n')));}
 else{
  const inp=p.locator('input[placeholder*="earch" i]').first();
  if(!(await inp.isVisible().catch(()=>false))){await p.locator('button[aria-label*="earch" i]').first().click();}
  await inp.fill(filt);await new Promise(r=>setTimeout(r,2500));
  await p.mouse.move(1,1);
  const box=await p.evaluate(head=>{let c=document.querySelector('input[placeholder*="earch" i]');while(c&&!c.textContent.includes(head))c=c.parentElement;const r=c.getBoundingClientRect();return {x:r.left,y:r.top+scrollY,width:r.width,height:r.height};},process.env.HEAD);
  const dst=base+'images/'+out;if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists '+dst);
  await p.screenshot({path:dst,clip:box,fullPage:true});rec.area=box;rec.result='saved; QA pending';
 }
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();if(!probe)fs.appendFileSync(base+'sources/m-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
