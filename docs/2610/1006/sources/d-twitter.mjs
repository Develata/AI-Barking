import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {execFileSync} from 'node:child_process';
const [name,mode,query]=process.argv.slice(2);
if(!/^d-[a-z0-9-]+$/.test(name)||!['search','tweet'].includes(mode))throw Error('read only');
const cfg=fs.readFileSync(path.join(os.homedir(),'.agent-reach/config.yaml'),'utf8');
const val=k=>{const m=cfg.match(new RegExp('^'+k+':\\s*(.+)$','m'));if(!m)throw Error('Missing configured credential');return m[1].trim().replace(/^['"]|['"]$/g,'');};
const auth=val('twitter_auth_token'),ct0=val('twitter_ct0');
const rec={time_bj:new Date(Date.now()+28800000).toISOString().replace('Z','+08:00'),tool:'agent-reach Twitter backend: twitter-cli, explicit existing configuration',mode,query,name};
try{
 const args=mode==='search'?['search',query,'-t','latest','-n','30','--json']:['tweet',query,'-n','0','--json'];
 const raw=execFileSync('twitter.exe',args,{encoding:'utf8',timeout:90000,maxBuffer:4000000,env:{...process.env,TWITTER_AUTH_TOKEN:auth,TWITTER_CT0:ct0},stdio:['ignore','pipe','pipe']});
 const data=JSON.parse(raw);
 // CLI read output contains public posts, never page navigation/account menu. Drop avatars recursively.
 const clean=o=>{if(Array.isArray(o))return o.map(clean);if(o&&typeof o==='object')return Object.fromEntries(Object.entries(o).filter(([k])=>!/avatar|profile_image|cookie|token|authorization/i.test(k)).map(([k,v])=>[k,clean(v)]));if(typeof o==='string')return o.replaceAll(auth,'[REDACTED]').replaceAll(ct0,'[REDACTED]');return o;};
 fs.writeFileSync('docs/2610/1006/sources/'+name+'.json',JSON.stringify({captured_bj:rec.time_bj,query,data:clean(data)},null,2),{flag:'wx'});
 rec.result='saved public result';
}catch(e){rec.result='FAILED';rec.error=(e.stderr?.toString()||e.message).replaceAll(auth,'[REDACTED]').replaceAll(ct0,'[REDACTED]').slice(0,600);}
fs.appendFileSync('docs/2610/1006/sources/d-twitter-records.jsonl',JSON.stringify(rec)+'\n');console.log(JSON.stringify(rec));
