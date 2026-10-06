import fs from 'node:fs';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const session='1005-a',dir='docs/2610/1005';
const cli=a=>execFileSync('opencli.exe',['browser',session,...a],{encoding:'utf8',timeout:90000,stdio:['ignore','pipe','pipe']}).split('\n  Update available:')[0].trim();
const cdp=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
const pages=[
 {url:'https://diff.wikimedia.org/2026/10/05/openai-rogue-agent-activities-found-on-wikimedia-projects/',shots:[
 ['01-a-wikimedia-opening.png','OpenAI “rogue” agent activities found on Wikimedia projects','These types of successful intrusions'],
 ['02-a-wikimedia-no-compromise.png','The Wikimedia Foundation conducted','Wiki editing:'],
 ['03-a-wikimedia-summary.png','In summary, we saw:','Excessive data downloading:']]},
 {url:'https://wikitech.wikimedia.org/wiki/Incidents/2026-05-13_wdqs',shots:[
 ['04-a-wdqs-summary.png','Summary','Despite the aggressive global edge rate limiting'],
 ['05-a-wdqs-timeline.png','Timeline','2026-05-11 16:40']]}];
for(const page of pages){
 if(!page.shots.some(s=>s[0]===process.argv[2]))continue;
 cli(['open',page.url]);cli(['state']);
 await cdp('Emulation.setDeviceMetricsOverride',{width:700,height:1200,deviceScaleFactor:2,mobile:false});
 await new Promise(r=>setTimeout(r,800));
 for(const [file,start,end] of page.shots){
  if(file!==process.argv[2])continue;
  const js=`(()=>{const find=s=>[...document.querySelectorAll('h1,h2,h3,p,li,table')].filter(e=>e.getBoundingClientRect().height>0&&(e.innerText||'').includes(s)).sort((a,b)=>a.innerText.length-b.innerText.length)[0];const a=find(${JSON.stringify(start)}),b=find(${JSON.stringify(end)});if(!a||!b)throw Error('anchor missing');const ar=a.getBoundingClientRect(),br=b.getBoundingClientRect();return {x:0,y:Math.max(0,Math.floor(ar.top+scrollY-20)),width:700,height:Math.ceil(br.bottom-ar.top+45),title:document.title,dpr:devicePixelRatio}})()`;
  const box=JSON.parse(cli(['eval','JSON.stringify('+js+')']));if(box.height<=0)throw Error('invalid crop');
  const shot=await cdp('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:box.x,y:box.y,width:box.width,height:box.height,scale:1}});
  fs.writeFileSync(`${dir}/images/${file}`,Buffer.from(shot.data,'base64'),{flag:'wx'});
  const rec={at:new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00'),url:page.url,file,tool:'opencli 1005-a + CDP; original live DOM',...box};
  fs.appendFileSync(`${dir}/sources/a-shots-log.jsonl`,JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));
  process.exit(0);
 }
}
