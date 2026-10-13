// 1012 A 组：匿名 headless Chrome 渲染 x.com 帖子页（不登录、全新临时 profile、DPR 2、en-US），按文本行边界截取帖子卡片（article）分段。
// 用法：node docs/2610/1012/sources/a-xpage.mjs <tweetId> <shots.json>
// shots.json: [{"file":"01-x.png","from":null|"文本","to":null|"文本"}]  from=null 表示 article 顶部；to=null 表示 article 底部；
//   裁切区间 = [from 所在行顶部 - pad, to 所在行顶部 - pad)，即 to 文本所在行不入图（边界落在行间）。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [id,shotsFile]=process.argv.slice(2);
const base='docs/2610/1012/';
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; DPR 2; x.com status page (UA override, en-US, tz Asia/Shanghai)',id};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1012-xp-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',viewport:{width:700,height:1400},deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto('https://x.com/karpathy/status/'+id+'?lang=en',{waitUntil:'commit',timeout:60000});
 await p.waitForSelector('article',{timeout:30000});
 await new Promise(r=>setTimeout(r,6000));
 const shots=JSON.parse(fs.readFileSync(shotsFile,'utf8'));
 rec.shots=[];
 for(const s of shots){
  const dst=base+'images/'+s.file;
  if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists '+dst);
  const area=await p.evaluate(([from,to,pad])=>{
   const art=document.querySelector('article');const r=art.getBoundingClientRect();
   const lineTop=t=>{const w=document.createTreeWalker(art,NodeFilter.SHOW_TEXT);let n;while(n=w.nextNode()){const i=n.textContent.indexOf(t);if(i>=0){const rg=document.createRange();rg.setStart(n,i);rg.setEnd(n,i+1);return rg.getBoundingClientRect().top+scrollY;}}throw Error('missing '+t);};
   const top=from?lineTop(from)-pad:r.top+scrollY-10;const bot=to?lineTop(to)-pad:r.bottom+scrollY;
   return {x:r.left-10,width:r.width+20,y:top,height:bot-top};
  },[s.from||null,s.to||null,s.pad??6]);
  await p.screenshot({path:dst,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  rec.shots.push({file:s.file,...area});
 }
 rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();fs.appendFileSync(base+'sources/a-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
