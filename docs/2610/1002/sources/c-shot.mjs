import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
import fs from 'node:fs';
const [session,file,x,y,width,height]=process.argv.slice(2);const cmd=(cdpMethod,cdpParams)=>sendCommand('cdp',{session,surface:'browser',cdpMethod,cdpParams});
await cmd('Emulation.setDeviceMetricsOverride',{width:Number(process.argv[8]||1100),height:850,deviceScaleFactor:2,mobile:false});
const r=await cmd('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:{x:+x,y:+y,width:+width,height:+height,scale:1}});fs.writeFileSync(file,Buffer.from(r.data,'base64'));
console.log(file);
process.exit(0);

