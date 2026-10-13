// 1013 C 组：匿名 headless Chrome（Playwright Core，全新临时 profile，不登录）。改写自 1012 a-browser.mjs。
// 用法（在仓库根目录运行）：node docs/2610/1013/sources/c-browser.mjs <name> <url> <cssWidth> @<shots.json> [waitMs]
// shots.json: [{"file":"25-x.png","a":"起始文本","b":"结束文本|null","pt":18,"pb":18}] 或 {"file":..,"y":y,"h":h}；{"file":..,"full":true} 截整页。
// 环境变量 NOARCHIVE=1 不写文本存档；OVERWRITE=1 覆盖；HIDE=1 隐藏 fixed/sticky 元素；SCHEME=dark 深色。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,width='700',shotsArg='[]',waitArg='6000']=process.argv.slice(2);
if(!/^c-[a-z0-9-]+$/.test(name))throw Error('C prefix required');
const base='docs/2610/1013/';const W=Number(width);
const record={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:`anonymous headless Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR 2`};
let browser;
try{
 browser=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-c-chrome-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1200},locale:process.env.LOCALE||'en-US',timezoneId:'Asia/Shanghai',colorScheme:process.env.SCHEME||'light',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',deviceScaleFactor:2,args:['--no-first-run','--no-default-browser-check']});
 const page=await browser.newPage();
 await page.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 if(process.env.HIDE){await page.evaluate(()=>{for(const e of document.querySelectorAll('body *')){const s=getComputedStyle(e);if(s.position==='fixed'||s.position==='sticky')e.style.setProperty('display','none','important');}});}
 const data=await page.evaluate(()=>({url:location.href,title:document.title,text:document.body.innerText,html:document.documentElement.outerHTML}));
 record.final_url=data.url;record.title=data.title;
 if(!process.env.NOARCHIVE){fs.writeFileSync(base+'sources/'+name+'.txt',data.text,{flag:process.env.OVERWRITE?'w':'wx'});fs.writeFileSync(base+'sources/'+name+'.rendered.html',data.html,{flag:process.env.OVERWRITE?'w':'wx'});}
 record.chars=data.text.length;record.resolved_shots=[];
 const shots=shotsArg.startsWith('@')?JSON.parse(fs.readFileSync(shotsArg.slice(1),'utf8')):JSON.parse(shotsArg);
 for(const s of shots){
  const file=s.file;
  if(!/^(2[5-9]|3[0-6])-[a-z0-9-]+\.png$/.test(file))throw Error('C image range 25-36 only: '+file);
  const dst=base+'images/'+file;
  if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('Existing '+dst);
  let area;
  if(s.full){area=await page.evaluate(()=>({x:0,y:0,width:innerWidth,height:document.documentElement.scrollHeight}));}
  else if(s.y!==undefined)area={y:s.y,height:s.h,x:0,width:W};
  else area=await page.evaluate(([a,b,pt,pb])=>{
   const find=(t,last)=>{const c=[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,img,table,section,blockquote')].filter(e=>(e.innerText||e.alt||'').trim().startsWith(t)).sort((p,q)=>p.getBoundingClientRect().height-q.getBoundingClientRect().height);return c[0];};
   const e1=find(a);if(!e1)throw Error('start missing: '+a);const e2=b?find(b):e1;if(!e2)throw Error('end missing: '+b);
   const r1=e1.getBoundingClientRect(),r2=e2.getBoundingClientRect();
   const top=Math.max(0,Math.floor(r1.y+scrollY)-pt),bot=Math.ceil(r2.y+scrollY+r2.height)+pb;
   return {y:top,height:bot-top,x:0,width:innerWidth};
  },[s.a,s.b||null,s.pt??18,s.pb??18]);
  await page.screenshot({path:dst,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  record.resolved_shots.push({file,...area});
 }
 record.result='saved; QA pending';
}catch(e){record.result='FAILED '+String(e.message).slice(0,400);}
finally{if(browser)await browser.close();fs.appendFileSync(base+'sources/c-browser-records.jsonl',JSON.stringify(record)+'\n');console.log(JSON.stringify(record));}
