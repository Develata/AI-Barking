// 1013 C 组：匿名 headless Chrome（全新临时 profile，不登录）打开页面，可先点击展开某按钮，再按“起止文字所在元素”的页面坐标裁出一段区域（2 倍像素比），存为中间 png（不入库）。
// 用法（仓库根目录）：node docs/2610/1013/sources/c-clip-shot.mjs <url> <out.png> <width> <startText> <endText> [clickText] [padTop] [padBottom]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,width,startText,endText,clickText='',padTop='24',padBottom='24']=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-cc-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:+width,height:1000},deviceScaleFactor:2,locale:'en-US',timezoneId:'Asia/Shanghai'});
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,out:path.basename(out),startText,endText,clickText,tool:'c-clip-shot.mjs (anonymous headless Chrome, fresh profile)'};
try{
 const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,6000));
 if(clickText){const el=p.getByText(clickText,{exact:true}).first();await el.scrollIntoViewIfNeeded();await el.click();await new Promise(r=>setTimeout(r,1500));rec.clicked=true;}
 const H=await p.evaluate(()=>document.documentElement.scrollHeight);for(let y=0;y<H;y+=700){await p.evaluate(y=>window.scrollTo(0,y),y);await new Promise(r=>setTimeout(r,120));}
 const box=async t=>{const l=p.getByText(t,{exact:false}).first();await l.scrollIntoViewIfNeeded();return await l.evaluate(e=>{const r=e.getBoundingClientRect();return {top:r.top+scrollY,bottom:r.bottom+scrollY,left:r.left+scrollX,right:r.right+scrollX,text:e.innerText.slice(0,120)}});};
 const s=await box(startText),e=await box(endText);rec.start=s;rec.end=e;
 const y0=Math.max(0,Math.floor(s.top-+padTop)),y1=Math.ceil(e.bottom+ +padBottom);
 await p.screenshot({path:out,clip:{x:0,y:y0,width:+width,height:y1-y0},fullPage:true});rec.clip={y0,y1};rec.result='saved';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{await b.close();fs.appendFileSync('docs/2610/1013/sources/c-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
