import os from 'node:os';import path from 'node:path';import fs from 'node:fs';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const urls=process.argv.slice(2);
const b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'rd-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:1100,height:1200},locale:'en-US',userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36'});
for(const u of urls){const p=await b.newPage();
 try{await p.goto(u,{waitUntil:'domcontentloaded',timeout:45000});await new Promise(r=>setTimeout(r,4000));
 const r=await p.evaluate(()=>{const s=document.querySelector('shreddit-post');return {url:location.href,title:document.title,at:s?{score:s.getAttribute('score'),comments:s.getAttribute('comment-count'),created:s.getAttribute('created-timestamp'),sub:s.getAttribute('subreddit-prefixed-name'),ptitle:s.getAttribute('post-title'),link:s.getAttribute('content-href')}:null,len:document.body.innerText.length}});
 console.log(JSON.stringify({at_bj:new Date(Date.now()+28800000).toISOString(),...r}));}catch(e){console.log(JSON.stringify({u,err:String(e.message).slice(0,100)}))}
 await p.close();}
await b.close();
