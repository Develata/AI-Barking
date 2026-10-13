// 1013 C 组：点开 Vals 榜单“VIEW N MORE MODELS”后按页面 CSS 像素区间截图（匿名 headless Chrome，DPR 2，700 CSS px）。
// 用法：node docs/2610/1013/sources/c-expand-shot.mjs <url> <out.png> <y> <h> [waitMs]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,y,h,wait='10000']=process.argv.slice(2);
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,out,tool:'c-expand-shot.mjs (anonymous headless Chrome; clicks "VIEW N MORE MODELS"; DPR 2; CSS 700)'};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-ce-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:700,height:1200},locale:'en-US',timezoneId:'Asia/Shanghai',deviceScaleFactor:2});
 const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,+wait));
 await p.evaluate(()=>{for(const e of document.querySelectorAll('body *')){const s=getComputedStyle(e);if(s.position==='fixed'||s.position==='sticky')e.style.setProperty('display','none','important');}});
 const btn=p.locator('button',{hasText:/VIEW \d+ MORE MODELS/i}).first();rec.button=await btn.innerText();await btn.click();await new Promise(r=>setTimeout(r,3000));
 const dst='docs/2610/1013/images/'+out;if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists');
 await p.screenshot({path:dst,fullPage:true,clip:{x:0,y:+y,width:700,height:+h}});
 rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();fs.appendFileSync('docs/2610/1013/sources/c-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
