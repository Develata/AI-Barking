// 1012 A 组：匿名 headless Chrome 打开页面，输出 innerText 与全部 a[href]（不存档、不登录）。用法：node a-links.mjs <url> [waitMs] [textOut]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,wait='6000',out]=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1012-al-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',viewport:{width:1100,height:1400}});
try{const p=await b.newPage();await p.goto(url,{waitUntil:'commit',timeout:60000});await new Promise(r=>setTimeout(r,Number(wait)));
const d=await p.evaluate(()=>({text:document.body.innerText,links:[...document.querySelectorAll('a[href]')].map(a=>a.href).filter(h=>/\/comments\//.test(h))}));
if(out)fs.writeFileSync(out,d.text);console.log(JSON.stringify([...new Set(d.links)],null,0));if(!out)console.log(d.text.slice(0,3000));}catch(e){console.log('FAILED',e.message.slice(0,200));}finally{await b.close();}
