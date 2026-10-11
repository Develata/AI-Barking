// 1011 A 组：X 官方嵌入帖子卡片截图（匿名 headless Chrome，全新临时 profile，不登录）。
// 用法：node docs/2610/1011/sources/a-xshot.mjs <tweetId> <out.png>   输出到 images/
import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.join(os.homedir(),'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const [id,out]=process.argv.slice(2);
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'anonymous headless Chrome via Playwright Core; fresh temp profile; DPR 2; X embed',id,out};
let b;
try{
 b=await chromium.launchPersistentContext(path.join(os.tmpdir(),'1011-x-'+Date.now()),{executablePath:'C:/Program Files/Google/Chrome/Application/chrome.exe',headless:true,viewport:{width:600,height:1400},deviceScaleFactor:2});
 const p=await b.newPage();
 await p.goto('https://platform.twitter.com/embed/Tweet.html?id='+id+'&lang=en&theme=light',{waitUntil:'networkidle',timeout:60000});
 await new Promise(r=>setTimeout(r,5000));
 const el=await p.$('article')||await p.$('[data-testid="tweet"]')||await p.$('body');
 const dst='docs/2610/1011/images/'+out;
 if(fs.existsSync(dst)&&!process.env.OVERWRITE)throw Error('exists '+dst);
 await el.screenshot({path:dst});
 rec.text=(await p.evaluate(()=>document.body.innerText)).slice(0,600);
 rec.result='saved; QA pending';
}catch(e){rec.result='FAILED '+String(e.message).slice(0,300);}
finally{if(b)await b.close();fs.appendFileSync('docs/2610/1011/sources/a-browser-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));}
