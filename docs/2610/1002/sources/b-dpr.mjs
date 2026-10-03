import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const session=process.argv[2]||'evidence1002b';
const cmd=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
await cmd('Emulation.setDeviceMetricsOverride',{width:1100,height:Number(process.argv[5]||850),deviceScaleFactor:2,mobile:false});
console.log('set DPR2 1100x850');
let shotY=Number(process.argv[4]||0);
const locate=()=>{const needle=process.argv[6];if(!needle)return shotY;const js='JSON.stringify([...document.querySelectorAll("p")].find(e=>e.innerText.includes('+JSON.stringify(needle)+'))?.getBoundingClientRect().top+scrollY)';const out=execFileSync('opencli.exe',['browser',session,'eval',js],{encoding:'utf8'});return Math.max(0,Number(out.split('\n')[0])-90);};
if(process.argv[6])shotY=locate();
if(process.argv[3]){execFileSync('opencli.exe',['browser',session,'scroll','up','--amount','100000']);execFileSync('opencli.exe',['browser',session,'scroll','down','--amount',String(shotY)]);await new Promise(r=>setTimeout(r,1800));if(process.argv[6]){shotY=locate();execFileSync('opencli.exe',['browser',session,'scroll','up','--amount','100000']);execFileSync('opencli.exe',['browser',session,'scroll','down','--amount',String(shotY)]);await new Promise(r=>setTimeout(r,1200));}const r=await cmd('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y:shotY,width:1100,height:Number(process.argv[5]||850),scale:1}});fs.writeFileSync(process.argv[3],Buffer.from(r.data,'base64')); console.log('saved');}
process.exit(0);
