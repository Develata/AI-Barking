// 1013 A 组：匿名 headless Chrome 截 GitHub 固定 commit 的网页（不登录、全新临时 profile、DPR 2、UA 去掉 Headless 标记）。
// 用法（仓库根目录）：node docs/2610/1013/sources/a-gh-shot.mjs <url> <out.png|probe:x.png> <x> <y> <w> <h> [waitMs] [scrollTo]
//   x/y/w/h 为导航并等待后“视口”坐标（CSS px，视口 1100 宽）；out 以 probe: 开头则写到 $SCR（试截，不入 images/）。
//   scrollTo 可选：页面滚动到的 CSS px y 值（用于锚点不自动滚动的页面）。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,out,x,y,w,h,wait='9000',scrollTo]=process.argv.slice(2);
const probe=out.startsWith('probe:');
const dst=probe?path.join(process.env.SCR||os.tmpdir(),out.slice(6)):'docs/2610/1013/images/'+out;
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; viewport 1100; DPR 2',url,out:probe?'(probe)':out,clip:[x,y,w,h]};
let b;
try{
 if(!probe&&fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists '+dst);
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-ghs-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',viewport:{width:1100,height:1400},deviceScaleFactor:2});
 const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(wait)));
 if(scrollTo){await p.evaluate(v=>window.scrollTo(0,Number(v)),scrollTo);await new Promise(r=>setTimeout(r,1500));}
 rec.scrollY=await p.evaluate(()=>scrollY);rec.title=await p.title();
 await p.screenshot({path:dst,clip:{x:Number(x),y:Number(y),width:Number(w),height:Number(h)},fullPage:false});
 rec.result=probe?'probe saved':'saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();if(!probe)fs.appendFileSync('docs/2610/1013/sources/a-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
