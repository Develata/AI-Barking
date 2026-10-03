import {homedir} from 'node:os';import {pathToFileURL} from 'node:url';import {join} from 'node:path';import fs from 'node:fs';import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const session='ev1003x';const cmd=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
await cmd('Emulation.setDeviceMetricsOverride',{width:700,height:2000,deviceScaleFactor:2,mobile:false});await new Promise(r=>setTimeout(r,1000));
const out=execFileSync('opencli.exe',['browser',session,'eval','JSON.stringify((()=>{const r=document.querySelector("article").getBoundingClientRect();return {x:r.x,y:r.y+scrollY,width:r.width,height:r.height,scale:1}})())'],{encoding:'utf8'});const clip=JSON.parse(out.split('\n')[0]);
const r=await cmd('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip});fs.writeFileSync('docs/2610/1003/images/22-tavus-x-note.png',Buffer.from(r.data,'base64'));console.log(clip);process.exit(0);
