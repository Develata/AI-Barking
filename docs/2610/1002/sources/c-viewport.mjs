import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
const {sendCommand}=await import(pathToFileURL(join(homedir(),'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js')).href);
await sendCommand('cdp',{session:'evidence1002c',surface:'browser',cdpMethod:'Emulation.setDeviceMetricsOverride',cdpParams:{width:1400,height:2250,deviceScaleFactor:2,mobile:false}});process.exit(0);
