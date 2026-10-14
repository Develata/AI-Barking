// 1013 C 组：匿名 headless Chrome（全新临时 profile，不登录）渲染页面，可选点击展开按钮（如 blog.google 的 "More Information" 脚注折叠区），整页截图到指定 png（中间图，不入库）。
// 用法（在仓库根目录）：node docs/2610/1013/sources/c-expand-shot.mjs <url> <out.png> <width> [clickText] [waitMs]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,width='900',clickText='',wait='6000']=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-cx-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:+width,height:1000},deviceScaleFactor:2,locale:'en-US',timezoneId:'Asia/Shanghai'});
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,out:path.basename(out),clickText,tool:'c-expand-shot.mjs (anonymous headless Chrome, fresh profile)'};
try{
 const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,+wait));
 if(clickText){const el=p.getByText(clickText,{exact:true}).first();await el.scrollIntoViewIfNeeded();await el.click();await new Promise(r=>setTimeout(r,1500));rec.clicked=true;}
 // scroll through to trigger lazy images
 const H=await p.evaluate(()=>document.documentElement.scrollHeight);for(let y=0;y<H;y+=800){await p.evaluate(y=>window.scrollTo(0,y),y);await new Promise(r=>setTimeout(r,150));}
 await p.evaluate(()=>window.scrollTo(0,0));await new Promise(r=>setTimeout(r,800));
 await p.screenshot({path:out,fullPage:true});rec.result='saved';rec.height=await p.evaluate(()=>document.documentElement.scrollHeight);
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{await b.close();fs.appendFileSync('docs/2610/1013/sources/c-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
