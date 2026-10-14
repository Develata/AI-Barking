// 1013 B 组（改写自 a-browser.mjs）：匿名 headless Chrome 渲染网页并按文字锚点裁切；截图前把当事人姓名与国籍词用黑条遮盖（DOM 层，text 节点包进黑底黑字 span，不改其他像素）。
// 用法（仓库根目录）：node docs/2610/1013/sources/b-browser.mjs <url> <cssWidth> @shots.json [waitMs]
// shots.json: [{"file":"13-x.png","a":"起始文本","b":"结束文本|null","pt":18,"pb":18}] ；a/b 取以该文本开头的最小元素（先按原文匹配，再遮盖）。
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,width='900',shotsArg='[]',waitArg='7000']=process.argv.slice(2);
const base='docs/2610/1013/';const W=Number(width);
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url,tool:`anonymous headless Chrome via Playwright Core; fresh temp profile; CSS ${W}; DPR 2; name/nationality blackout`};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1013-bb-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:W,height:1200},locale:'en-US',timezoneId:'Asia/Shanghai',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36',deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(waitArg)));
 // 去掉粘性层与广告遮挡（只隐藏 fixed/sticky 元素，不改正文）
 await p.addStyleTag({content:'*{scroll-behavior:auto!important}'});
 await p.evaluate(()=>{for(const e of document.querySelectorAll('*')){const c=getComputedStyle(e).position;if(c==='fixed'||c==='sticky')e.style.display='none';}});
 const shots=JSON.parse(fs.readFileSync(shotsArg.slice(1),'utf8'));
 const NAME=/abdul\s?rahman\s+al[\s-]?f[ou]+zan|al[\s-]?f[ou]+zan|abdul\s?rahman|saudi\s+national|kingdom\s+of\s+saudi\s+arabia|saudi\s+arabia|saudi/ig;
 rec.resolved=[];
 for(const s of shots){
  const dst=base+'images/'+s.file;
  if(!/^(1[3-9]|2[0-4])-[a-z0-9-]+\.png$/.test(s.file))throw Error('B image range 13-24 only: '+s.file);
  if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('Existing '+dst);
  const area=await p.evaluate(([a,bb,pt,pb,pat,sa])=>{
   const find=(t,sel)=>[...document.querySelectorAll(sel||'h1,h2,h3,h4,p,li,td,th,div,span,figure,blockquote')].filter(e=>(e.innerText||'').trim().startsWith(t)).sort((x,y)=>x.getBoundingClientRect().height-y.getBoundingClientRect().height)[0];
   const e1=find(a,sa);if(!e1)throw Error('start missing: '+a);const e2=bb?find(bb):e1;if(!e2)throw Error('end missing: '+bb);
   e1.scrollIntoView({block:'start'});
   // 遮盖
   const re=new RegExp(pat,'ig');
   const roots=[e1,e2];
   const walker=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);const nodes=[];let n;while(n=walker.nextNode())nodes.push(n);
   for(const t of nodes){ if(!t.parentElement||t.parentElement.dataset&&t.parentElement.dataset.bk)continue; if(!/script|style/i.test(t.parentElement.tagName)&&re.test(t.nodeValue)){re.lastIndex=0;
     const frag=document.createDocumentFragment();let last=0,v=t.nodeValue,m;
     while((m=re.exec(v))){ if(m.index>last)frag.appendChild(document.createTextNode(v.slice(last,m.index)));const sp=document.createElement('span');sp.dataset.bk='1';sp.style.cssText='background:#000;color:#000;-webkit-text-fill-color:#000;text-decoration:none';sp.textContent=m[0];frag.appendChild(sp);last=m.index+m[0].length;}
     if(last<v.length)frag.appendChild(document.createTextNode(v.slice(last)));t.parentNode.replaceChild(frag,t);} re.lastIndex=0;}
   const r1=e1.getBoundingClientRect(),r2=e2.getBoundingClientRect();
   const top=Math.max(0,Math.floor(r1.y+scrollY)-pt),bot=Math.ceil(r2.y+scrollY+r2.height)+pb;
   return {y:top,height:bot-top,x:0,width:innerWidth};
  },[s.a,s.b||null,s.pt??16,s.pb??16,NAME.source,s.sa||null]);
  await p.screenshot({path:dst,clip:{x:area.x,y:area.y,width:area.width,height:area.height},fullPage:true});
  rec.resolved.push({file:s.file,...area});
 }
 rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,400);}
finally{if(b)await b.close();fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
