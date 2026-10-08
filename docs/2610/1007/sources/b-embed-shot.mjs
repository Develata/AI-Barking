// Screenshot only the tweet card from X's own embed endpoint (anonymous, no login). Usage: node b-embed-shot.mjs <statusId> <outfile> [cssWidth]
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const base='docs/2610/1007/';const [id,out,W='550']=process.argv.slice(2);
if(!/^(1[5-9]|2[0-4])-b-[a-z0-9-]+\.png$/.test(out))throw Error('B image range');
if(fs.existsSync(base+'images/'+out)&&!process.env.OVERWRITE)throw Error('exists');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const url=`https://platform.twitter.com/embed/Tweet.html?id=${id}&lang=en&theme=light`;
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-embed-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:Number(W),height:1500},deviceScaleFactor:Number(process.env.DPR||2)});
const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,9000));
const info=await p.evaluate(()=>{const a=document.querySelector('article')||document.querySelector('[data-testid="tweet"]')||document.body;const r=a.getBoundingClientRect();return {tag:a.tagName,x:r.x,y:r.y,w:r.width,h:r.height,text:document.body.innerText.slice(0,900)}});
console.log(JSON.stringify(info));
const rec={time_bj,url:`https://x.com/arena/status/${id}`,name:'b-embed-shot',tool:`anonymous headless Chrome via Playwright Core; X embed endpoint ${url}; CSS ${W}; DPR ${process.env.DPR||2}; card only`,card:info,resolved_shots:[]};
if(info.h>50){await p.screenshot({path:base+'images/'+out,clip:{x:Math.max(0,info.x-4),y:Math.max(0,info.y-4),width:Math.min(Number(W),info.w+8),height:info.h+8},fullPage:true});rec.resolved_shots.push({file:out});rec.result='saved; QA pending';}
else rec.result='FAILED no card rendered';
fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify(rec)+'\n');
await b.close();
