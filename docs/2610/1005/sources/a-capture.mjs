import fs from 'node:fs';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
const [file,url,y,height]=process.argv.slice(2),session='1005-a';
const cdp=(cdpMethod,cdpParams)=>Promise.race([sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams}),new Promise((_,reject)=>setTimeout(()=>reject(Error('30s timeout '+cdpMethod)),30000))]);
try{
 await cdp('Emulation.setDeviceMetricsOverride',{width:700,height:1200,deviceScaleFactor:2,mobile:false});
 const box={url,x:0,y:Number(y),width:700,height:Number(height),dpr:2};
 console.log(JSON.stringify(box));
 const shot=await cdp('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:box.x,y:box.y,width:700,height:box.height,scale:1}});
 fs.writeFileSync(`docs/2610/1005/images/${file}`,Buffer.from(shot.data,'base64'),{flag:'wx'});
 fs.appendFileSync('docs/2610/1005/sources/a-shots-log.jsonl',JSON.stringify({at:new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00'),file,tool:'opencli 1005-a CDP; unchanged live page',...box})+'\n');
 process.exit(0);
}catch(e){console.error(String(e));process.exit(1);}
