import {homedir} from 'node:os';import {pathToFileURL} from 'node:url';import {join} from 'node:path';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
console.log(await sendCommand('cdp',{session:'ev1003a',surface:'browser',cdpMethod:'Input.dispatchMouseEvent',cdpParams:{type:'mouseWheel',x:350,y:400,deltaX:0,deltaY:1900}}));process.exit(0);
