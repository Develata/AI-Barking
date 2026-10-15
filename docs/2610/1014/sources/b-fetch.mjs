// 1014 B 组：匿名 headless Chrome（Playwright Core）取页面渲染后 HTML；用法：node b-fetch.mjs <url> <out.html> [waitMs]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,wait='5000']=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1014-bf-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36'});
try{const p=await b.newPage();const r=await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(wait)));fs.writeFileSync(out,await p.content());console.log(r.status(),(await p.title()),fs.statSync(out).size);}catch(e){console.log('FAILED',String(e.message).slice(0,200));}finally{await b.close();}
