// Arena WebDev leaderboard: the list scrolls inside an inner container, so scroll the mistral-large-4 row into view and take a viewport screenshot.
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const base='docs/2610/1007/';const [out,W='700',H='700',wide]=process.argv.slice(2);
if(!/^(1[5-9]|2[0-4])-b-[a-z0-9-]+\.png$/.test(out))throw Error('B image range');
if(fs.existsSync(base+'images/'+out)&&!process.env.OVERWRITE)throw Error('exists');
const time_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-arena-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:Number(W),height:Number(H)},deviceScaleFactor:Number(process.env.DPR||2)});
const p=await b.newPage();await p.goto('https://arena.ai/leaderboard/code/webdev/overall',{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,10000));
const info=await p.evaluate(()=>{const e=[...document.querySelectorAll('span')].find(s=>s.innerText.trim()==='mistral-large-4');if(!e)return null;e.scrollIntoView({block:'center'});return true});
await new Promise(r=>setTimeout(r,1500));
const rowtxt=await p.evaluate(()=>{const e=[...document.querySelectorAll('span')].find(s=>s.innerText.trim()==='mistral-large-4');if(!e)return null;let r=e;for(let i=0;i<6&&r.parentElement;i++){r=r.parentElement;if(r.tagName==='TR')break;}return {row:r.innerText.replace(/\n/g,' | '),y:Math.round(e.getBoundingClientRect().y),tz:Intl.DateTimeFormat().resolvedOptions().timeZone}});
console.log(JSON.stringify({info,rowtxt}));
await p.screenshot({path:base+'images/'+out});
fs.appendFileSync(base+'sources/b-browser-records.jsonl',JSON.stringify({time_bj,url:'https://arena.ai/leaderboard/code/webdev/overall',name:'b-arena-row-shot',tool:`anonymous headless Chrome via Playwright Core; viewport ${W}x${H}, DPR ${process.env.DPR||2}; inner list scrolled so the mistral-large-4 row is centred`,row:rowtxt,resolved_shots:[{file:out}],result:'saved; QA pending'})+'\n');
await b.close();
