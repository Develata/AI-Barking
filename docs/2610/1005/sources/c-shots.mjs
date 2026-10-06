import fs from 'node:fs';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const cdp=(method,params)=>sendCommand('cdp',{session:'1005-c',surface:'browser',cdpMethod:method,cdpParams:params});
await cdp('Emulation.setDeviceMetricsOverride',{width:700,height:1800,deviceScaleFactor:2,mobile:false});
await new Promise(r=>setTimeout(r,700));
const js='JSON.stringify([...document.querySelectorAll("h1,h2,h3,p,img,article")].map(e=>({tag:e.tagName,text:e.innerText?.slice(0,120),y:Math.round(e.getBoundingClientRect().top+scrollY),h:Math.round(e.getBoundingClientRect().height)})))';
const loc=JSON.parse(execFileSync('opencli.exe',['browser','1005-c','eval',js],{encoding:'utf8'}).split('\n')[0]);
const y=s=>loc.find(e=>e.text?.startsWith(s))?.y;
const mode=process.argv[2]||'blog';
let shots;
if(mode==='blog') shots=[['15-c-summary',y('Two Room-Temperature'),y('We’re all used')],['16-c-designed',y('Candidate 1:'),y('Candidate 2:')],['17-c-prior-work',y('Candidate 2:'),y('These predictions are')],['18-c-water',y('Our agent identified'),y('The bottom line')],['19-c-bottom-line',y('The bottom line'),y('In the spirit of transparency')+200]];
else if(mode==='readme') shots=[['20-c-readme-status',y('Status of the main claims'),y('Corrections found')],['21-c-readme-caveats',y('How this was produced'),y('Experiments that would settle it')]];
else shots=JSON.parse(process.argv[3]);
for(const [name,start,end] of shots){const top=Math.max(0,start-20),height=end-top+20; if(!Number.isFinite(height))throw Error('Missing locator '+name);const r=await cdp('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y:top,width:700,height,scale:1}});const path=`docs/2610/1005/images/${name}.png`;fs.writeFileSync(path,Buffer.from(r.data,'base64'),{flag:'wx'});console.log(path,height);}
fs.appendFileSync('docs/2610/1005/sources/c-shot-records.jsonl',JSON.stringify({time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),mode,deviceScaleFactor:2,width:700,shots,loc})+'\n');
