import fs from 'node:fs';
import {homedir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const cdp=(cdpMethod,cdpParams)=>sendCommand('cdp',{session:'1005-b',surface:'browser',cdpMethod,cdpParams});
await cdp('Emulation.setDeviceMetricsOverride',{width:700,height:2400,deviceScaleFactor:2,mobile:false});
await new Promise(r=>setTimeout(r,2500));
const raw=execFileSync('opencli.exe',['browser','1005-b','eval','JSON.stringify({url:location.href,heads:[...document.querySelectorAll("h1,h2,h3")].map(e=>({t:e.innerText,y:e.getBoundingClientRect().top+scrollY})),tables:[...document.querySelectorAll("table")].map(e=>({text:e.innerText,x:e.getBoundingClientRect().x,width:e.getBoundingClientRect().width}))})'],{encoding:'utf8'});
const d=JSON.parse(raw.split('\n  Update available:')[0].trim());
if(d.url!=='https://openai.com/index/eu-text-provenance/')throw Error('Wrong source');
const h=d.heads;
const shots=[['10-b-openai-rollout.png',Math.max(0,h[0].y-75),h[1].y-70],['11-b-openai-detection.png',h[1].y-100,h[2].y+70],['12-b-openai-quality.png',h[2].y-100,h[3].y+70],['13-b-openai-limits.png',h[3].y-100,h[4].y+150]];
for(const [file,y,end] of shots){const shot=await cdp('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y,width:700,height:Math.ceil(end-y),scale:1}});fs.writeFileSync('docs/2610/1005/images/'+file,Buffer.from(shot.data,'base64'));console.log(file,700*2,Math.ceil(end-y)*2);}
fs.writeFileSync('docs/2610/1005/sources/b-shot-manifest.json',JSON.stringify({captured_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),url:d.url,dpr:2,geometry:d,shots},null,2));
