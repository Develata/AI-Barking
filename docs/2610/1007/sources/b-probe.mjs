// Probe element geometry by text (debug helper; writes nothing). Usage: node b-probe.mjs <url> <width> <waitMs> <text1> [text2...]
import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [url,width,wait,...texts]=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1007-b-probe-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:Number(width),height:1200},deviceScaleFactor:Number(process.env.DPR||2)});
const p=await b.newPage();await p.goto(url,{waitUntil:'domcontentloaded',timeout:60000});await new Promise(r=>setTimeout(r,Number(wait)));
for(const t of texts){const r=await p.evaluate(t=>[...document.querySelectorAll('body *')].filter(e=>(e.innerText||'').includes(t)&&![...e.children].some(c=>(c.innerText||'').includes(t))).slice(0,5).map(e=>{const r=e.getBoundingClientRect();return {tag:e.tagName,cls:(e.className||'').toString().slice(0,60),y:Math.round(r.y+scrollY),x:Math.round(r.x),w:Math.round(r.width),h:Math.round(r.height),text:e.innerText.slice(0,80)}}),t);console.log(JSON.stringify({t,r}));}
console.log('docH',await p.evaluate(()=>document.documentElement.scrollHeight));
await b.close();
