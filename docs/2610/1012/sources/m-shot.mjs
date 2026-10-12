// 1012 批注底图重截：匿名 headless Chrome（全新临时 profile、不登录、DPR 2、700 CSS px 宽、en-US、Asia/Shanghai），
// 按文本行边界裁切，边界落在行间。改自 a-xpage.mjs。
// 环境变量 HIDE=1：截图前隐藏 position fixed/sticky 的浮层（文档站的 Ask AI / Docs agent 面板、页头）。
// 环境变量 VW：视口 CSS 宽（默认 700）；ROOTTEXT：在多个 root 中选含该文本的那个。
// 用法：node docs/2610/1012/sources/m-shot.mjs <url> <rootSelector> <out.png> <fromText|-> <toText|->
//   from=- 表示 root 顶部；to=- 表示 root 底部；to 文本所在行不入图。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,sel,out,from,to]=process.argv.slice(2);
const base='docs/2610/1012/';
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; DPR 2; 700 CSS px',url,out};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1012-m-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',viewport:{width:+(process.env.VW||700),height:1400},deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto(url,{waitUntil:'commit',timeout:60000});
 await p.waitForSelector(sel,{timeout:40000});
 await new Promise(r=>setTimeout(r,7000));
 if(process.env.HIDE){await p.evaluate(()=>{for(const e of document.querySelectorAll('body *')){const s=getComputedStyle(e);if(s.position==='fixed'||s.position==='sticky')e.style.setProperty('display','none','important');}});}
 if(from!=='-'){await p.evaluate(t=>{const w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);let n;while(n=w.nextNode()){if(n.textContent.includes(t)){n.parentElement.scrollIntoView({block:'center'});return;}}},from);await new Promise(r=>setTimeout(r,3000));}
 const dst=base+'images/'+out;
 if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists '+dst);
 const area=await p.evaluate(([sel,from,to,pad,rootText])=>{
  const roots=[...document.querySelectorAll(sel)];
  const lineTop=(root,t)=>{const w=document.createTreeWalker(root,NodeFilter.SHOW_TEXT);let n;while(n=w.nextNode()){const i=n.textContent.indexOf(t);if(i>=0){const rg=document.createRange();rg.setStart(n,i);rg.setEnd(n,i+1);return rg.getBoundingClientRect().top+scrollY;}}return null;};
  const root=roots.find(r=>(!rootText||lineTop(r,rootText)!==null)&&(!from||lineTop(r,from)!==null))||roots[0];
  const r=root.getBoundingClientRect();
  const top=from?lineTop(root,from)-pad:r.top+scrollY-10;const bot=to?lineTop(root,to)-pad:r.bottom+scrollY+6;
  if(top===null||Number.isNaN(top)||Number.isNaN(bot))throw Error('missing text');
  return {x:Math.max(0,r.left-10),width:Math.min(innerWidth,r.width+20),y:top,height:bot-top};
 },[sel,from==='-'?null:from,to==='-'?null:to,+(process.env.PAD||8),process.env.ROOTTEXT||null]);
 await p.screenshot({path:dst,clip:area,fullPage:true});
 rec.area=area;rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();fs.appendFileSync(base+'sources/m-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
