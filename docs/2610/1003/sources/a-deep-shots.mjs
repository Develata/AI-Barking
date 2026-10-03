import {homedir} from 'node:os';import {pathToFileURL} from 'node:url';import {join} from 'node:path';import fs from 'node:fs';import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const session='ev1003a';const cmd=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
await cmd('Emulation.setDeviceMetricsOverride',{width:700,height:12000,deviceScaleFactor:2,mobile:false});await new Promise(r=>setTimeout(r,1200));
const out=execFileSync('opencli.exe',['browser',session,'eval','JSON.stringify([...document.querySelectorAll("h2")].slice(0,6).map(e=>({text:e.innerText,y:e.getBoundingClientRect().top})))'],{encoding:'utf8'});const heads=JSON.parse(out.split('\n')[0]);
for(let i=0;i<6;i++){const y=Math.max(0,heads[i].y-40);const height=i===0?1400:Math.min(1500,(heads[i+1]?.y||heads[i].y+1000)-y);const r=await cmd('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y,width:700,height,scale:1}});const names=['probability','pde','group','optimization','arithmetic','algebra'];const path='docs/2610/1003/images/'+String(i+2).padStart(2,'0')+'-meta-'+names[i]+'.png';fs.writeFileSync(path,Buffer.from(r.data,'base64'));console.log(path,y,height);}
process.exit(0);
