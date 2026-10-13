// 1013 B 组：匿名 headless Chrome 渲染 Anthropic 文章内嵌图表页（assets.claude.ai 的 -light.html），导出渲染后文本。
// 用法（仓库根目录）：node docs/2610/1013/sources/b-figtext.mjs
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const dir='docs/2610/1013/sources/b-figs/';
const files=fs.readdirSync(dir).filter(f=>f.endsWith('.html'));
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-bfig-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:1000,height:900},deviceScaleFactor:2});
for(const f of files){
 const p=await b.newPage();
 await p.goto(pathToFileURL(path.resolve(dir+f)).href,{waitUntil:'load'});await new Promise(r=>setTimeout(r,1500));
 const t=await p.evaluate(()=>document.body.innerText);
 fs.writeFileSync(dir+f.replace('.html','.txt'),t);console.log(f,t.length);await p.close();
}
await b.close();
