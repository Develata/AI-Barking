import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const [session,path,needle,height='900']=process.argv.slice(2);
const cmd=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
await cmd('Emulation.setDeviceMetricsOverride',{width:700,height:Number(height),deviceScaleFactor:2,mobile:false});
await new Promise(r=>setTimeout(r,700));
let y=0;
if(needle){const expr=needle.startsWith("@") ? "Math.max(0,document.querySelector("+JSON.stringify(needle.slice(1))+").getBoundingClientRect().top+scrollY-50)" : '(()=>{const es=[...document.querySelectorAll("h1,h2,h3,h4,p,figure,section,div")].filter(e=>e.innerText?.includes('+JSON.stringify(needle)+'));es.sort((a,b)=>a.innerText.length-b.innerText.length);return es.length?Math.max(0,es[0].getBoundingClientRect().top+scrollY-50):-1})()';const out=execFileSync('opencli.exe',['browser',session,'eval',expr],{encoding:'utf8'});y=Number(out.split('\n')[0]);if(y<0||!Number.isFinite(y))throw Error('needle missing');}
execFileSync('opencli.exe',['browser',session,'scroll','up','--amount','100000'],{stdio:'pipe'});
execFileSync('opencli.exe',['browser',session,'scroll','down','--amount',String(y)],{stdio:'pipe'});
await new Promise(r=>setTimeout(r,1500));
const r=await cmd('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y,width:700,height:Number(height),scale:1}});
fs.writeFileSync(path,Buffer.from(r.data,'base64'));console.log(JSON.stringify({path,y,width:1400,height:2*Number(height),dpr:2}));process.exit(0);



