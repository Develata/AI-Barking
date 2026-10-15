// 1014 B 组（改写自 1013 b-browser.mjs）：匿名 headless Chrome 渲染网页并按文字锚点裁切；只隐藏 fixed/sticky 层（弹窗、遮罩），不改正文。
// 用法（仓库根目录）：node docs/2610/1014/sources/b-browser.mjs <url> <cssWidth> @shots.json [waitMs]
// shots.json: [{"file":"16-x.png","a":"起始文本","b":"结束文本|null","pt":18,"pb":18}]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,width='700',shotsArg='[]',waitArg='7000']=process.argv.slice(2);
const base='docs/2610/1014/';const W=Number(width);
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,tool:`anonymous headless Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR 2; fixed/sticky layers hidden`};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1014-bb-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1200},locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 await p.addStyleTag({content:'*{scroll-behavior:auto!important}'});
 await p.evaluate(()=>{for(const e of document.querySelectorAll('*')){const c=getComputedStyle(e).position;if(c==='fixed'||c==='sticky')e.style.display='none';} document.documentElement.style.overflow='visible';document.body.style.overflow='visible';document.body.style.filter='none';});
 const shots=JSON.parse(fs.readFileSync(shotsArg.slice(1),'utf8'));
 rec.resolved=[];
 for(const s of shots){
  const dst=base+'images/'+s.file;
  if(!/^(1[3-9]|2[0-4])-[a-z0-9-]+\.png$/.test(s.file))throw Error('B image range 13-24 only: '+s.file);
  if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('Existing '+dst);
  const area=await p.evaluate(([a,bb,pt,pb])=>{
   const find=(t)=>[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,blockquote')].filter(e=>(e.innerText||'').trim().startsWith(t)).sort((x,y)=>x.getBoundingClientRect().height-y.getBoundingClientRect().height)[0];
   const e1=find(a);if(!e1)throw Error('start missing: '+a);const e2=bb?find(bb):e1;if(!e2)throw Error('end missing: '+bb);
   const r1=e1.getBoundingClientRect(),r2=e2.getBoundingClientRect();
   const top=Math.max(0,Math.floor(r1.y+scrollY)-pt),bot=Math.ceil(r2.y+scrollY+r2.height)+pb;
   return {y:top,height:bot-top,x:0,width:innerWidth};
  },[s.a,s.b||null,s.pt??16,s.pb??16]);
  await p.screenshot({path:dst,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  rec.resolved.push({file:s.file,...area});
 }
 rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,400);}
finally{if(b)await b.close();fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
