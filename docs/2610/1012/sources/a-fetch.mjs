// 1013 A 组（改写自 1012 同名脚本）：匿名 headless Chrome（全新临时 profile，不登录，UA 去掉 Headless 标记）取渲染后文本。
// 用法：node docs/2610/1013/sources/a-fetch.mjs <name(a-xxx)> <url> [waitMs] [shot.png tmp]  → sources/<name>.txt（已存在则报错）；可选整页截图到 $SCR/<shot>
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,wait='8000',shot]=process.argv.slice(2);
if(!/^a-[a-z0-9-]+$/.test(name))throw Error('prefix');
const base='docs/2610/1013/';
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; UA override; en-US'};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-af-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',viewport:{width:1100,height:1400},deviceScaleFactor:2});
 const p=await b.newPage();
 const resp=await p.goto(url,{waitUntil:'commit',timeout:60000});
 rec.status=resp&&resp.status();
 await new Promise(r=>setTimeout(r,Number(wait)));
 const text=await p.evaluate(()=>document.body.innerText);
 rec.title=await p.title();rec.final_url=p.url();rec.chars=text.length;
 fs.writeFileSync(base+'sources/'+name+'.txt',text,{flag:'wx'});
 if(shot)await p.screenshot({path:path.join(process.env.SCR||os.tmpdir(),shot),fullPage:true});
 rec.result='saved';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();fs.appendFileSync(base+'sources/a-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
