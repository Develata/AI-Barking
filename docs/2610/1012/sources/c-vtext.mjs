// 1013 C 组：匿名 headless Chrome 渲染后，按文档顺序导出全部文本节点（含链接 href），用于 Vals 这类 innerText 取不全的页面。
// 用法：node docs/2610/1013/sources/c-vtext.mjs <url> <out.txt> [waitMs] [width]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,wait='8000',width='1280']=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-cv-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:+width,height:1000},locale:'en-US',timezoneId:'Asia/Shanghai'});
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,out,tool:'c-vtext.mjs (anonymous headless Chrome)'};
try{
const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,+wait));
const t=await p.evaluate(()=>{const o=[];const w=document.createTreeWalker(document.body,NodeFilter.SHOW_ELEMENT|NodeFilter.SHOW_TEXT);let n;while(n=w.nextNode()){if(n.nodeType===3){if(n.parentElement&&/^(SCRIPT|STYLE|NOSCRIPT)$/.test(n.parentElement.tagName))continue;const s=n.textContent.trim();if(s)o.push(s);}else if(/^(P|DIV|LI|H\d|TR|SECTION|BR|TABLE)$/.test(n.tagName)){o.push('\n');}if(n.tagName==='A'&&n.href)o.push('['+n.href+']');}return o.join(' ').replace(/ ?\n ?/g,'\n').replace(/\n+/g,'\n');});
fs.writeFileSync(out,t);rec.chars=t.length;rec.title=await p.title();rec.result='saved';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{await b.close();fs.appendFileSync('docs/2610/1013/sources/c-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
