import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const lib=join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js');
const {sendCommand}=await import(pathToFileURL(lib).href);
const session='1004-c';
const cdp=(method,params)=>sendCommand('cdp',{session,surface:'browser',cdpMethod:method,cdpParams:params});
const evalPage=js=>JSON.parse(execFileSync('opencli.exe',['browser',session,'eval',js],{encoding:'utf8'}).split('\n')[0]);
await cdp('Emulation.setDeviceMetricsOverride',{width:700,height:1800,deviceScaleFactor:2,mobile:false});
await new Promise(r=>setTimeout(r,500));
evalPage('(()=>{const d=[...document.querySelectorAll("article details")].find(x=>x.querySelector("table tr"));d.open=true;d.querySelector("table").style.zoom="0.72";return true})()');
await new Promise(r=>setTimeout(r,500));
const loc=evalPage('JSON.stringify({h1:(()=>{let e=document.querySelector("article h1");return {top:Math.round(e.getBoundingClientRect().top+scrollY),height:Math.round(e.getBoundingClientRect().height)}})(),h2:(()=>{let e=document.querySelector("article h2");return {top:Math.round(e.getBoundingClientRect().top+scrollY)}})(),table:(()=>{let d=[...document.querySelectorAll("article details")].find(x=>x.querySelector("table tr"));let e=d.querySelector("table"),r=e.getBoundingClientRect(),dr=d.getBoundingClientRect();return {top:Math.round(dr.top+scrollY),height:Math.round(dr.height),tableHeight:Math.round(r.height),tableWidth:Math.round(r.width),rows:e.querySelectorAll("tr").length}})()})');
async function capture(name,y,height){const r=await cdp('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:0,y,width:700,height,scale:1}});const p=`docs/2610/1004/images/${name}`;fs.writeFileSync(p,Buffer.from(r.data,'base64'));console.log(`${p} y=${y} height=${height} bytes=${fs.statSync(p).size}`);}
await capture('15-kolibri-blog-claims.png',0,loc.h2.top+80);
const ty=Math.max(0,loc.table.top-150); await capture('16-kolibri-blog-benchmarks.png',ty,loc.table.height+300);
console.log(JSON.stringify(loc));

