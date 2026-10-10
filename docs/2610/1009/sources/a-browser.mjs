// 1009 A 组：匿名 headless Chrome（Playwright Core，全新临时 profile，不登录）。改写自 1007 b-browser.mjs。
// 用法（在仓库根目录运行）：node docs/2610/1009/sources/a-browser.mjs <name> <url> <cssWidth> @<shots.json> [waitMs]
// shots.json: [{"file":"01-x.png","a":"起始文本","b":"结束文本|null","pt":18,"pb":18,"tmp":false}]
//   元素按 innerText 以 a / b 开头、取最小高度者；从 a 元素顶部到 b 元素底部裁切。tmp=true 时存到 $SCR（供拼接），否则存到 images/。
//   也可 {"file":..,"y":y,"h":h} 按页面 CSS 像素区间裁。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [name,url,width='700',shotsArg='[]',waitArg='6000']=process.argv.slice(2);
if(!/^a-[a-z0-9-]+$/.test(name))throw Error('A prefix required');
const base='docs/2610/1009/';const W=Number(width);const SCR=process.env.SCR||os.tmpdir();
const record={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,name,tool:`anonymous headless Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR 2`};
let browser;
try{
 browser=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1009-a-chrome-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1200},deviceScaleFactor:2,args:['--no-first-run','--no-default-browser-check']});
 const page=await browser.newPage();
 await page.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 const data=await page.evaluate(()=>({url:location.href,title:document.title,text:document.body.innerText}));
 data.captured_bj=record.time_bj;
 if(!process.env.NOARCHIVE){fs.writeFileSync(base+'sources/'+name+'.txt',data.text,{flag:'wx'});}
 record.chars=data.text.length;record.resolved_shots=[];
 const shots=shotsArg.startsWith('@')?JSON.parse(fs.readFileSync(shotsArg.slice(1),'utf8')):JSON.parse(shotsArg);
 for(const s of shots){
  const file=s.file;
  if(!s.tmp&&!/^(0[1-9]|1[0-4])-a-?[a-z0-9-]+\.png$/.test(file)&&!/^(0[1-9]|1[0-4])-[a-z0-9-]+\.png$/.test(file))throw Error('A image range 01-14 only: '+file);
  const dst=s.tmp?path.join(SCR,file):base+'images/'+file;
  if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('Existing '+dst);
  let area;
  if(s.y!==undefined)area={y:s.y,height:s.h,x:0,width:W};
  else area=await page.evaluate(([a,b,pt,pb])=>{
   const find=t=>[...document.querySelectorAll('h1,h2,h3,h4,p,li,td,th,div,span,figure,img,table,section,blockquote')].filter(e=>(e.innerText||e.alt||'').trim().startsWith(t)).sort((p,q)=>p.getBoundingClientRect().height-q.getBoundingClientRect().height)[0];
   const e1=find(a);if(!e1)throw Error('start missing: '+a);const e2=b?find(b):e1;if(!e2)throw Error('end missing: '+b);
   const r1=e1.getBoundingClientRect(),r2=e2.getBoundingClientRect();
   const top=Math.max(0,Math.floor(r1.y+scrollY)-pt),bot=Math.ceil(r2.y+scrollY+r2.height)+pb;
   return {y:top,height:bot-top,x:0,width:innerWidth};
  },[s.a,s.b||null,s.pt??18,s.pb??18]);
  await page.screenshot({path:dst,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  record.resolved_shots.push({file,tmp:!!s.tmp,...area});
 }
 record.result='saved; QA pending';
}catch(e){record.result='FAILED '+String(e.message).slice(0,400);}
finally{if(browser)await browser.close();fs.appendFileSync(base+'sources/a-browser-records.jsonl',JSON.stringify(record)+'\n');console.log(JSON.stringify(record));}
