import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const base='docs/2610/1006/sources/';
const [name,url,mode='page']=process.argv.slice(2);
if(!/^d-[a-z0-9-]+$/.test(name)) throw Error('D prefix required');
const bj=()=>new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const cli=(args)=>execFileSync('opencli.exe',['browser','1006-d',...args],{encoding:'utf8',timeout:170000,maxBuffer:16000000,env:{...process.env,OPENCLI_BROWSER_COMMAND_TIMEOUT:'150'}}).split('\n  Update available:')[0].trim();
let record={time_bj:bj(),url,tool:'opencli browser 1006-d open/eval',name};
try {
 cli(['open',url]);
 await new Promise(r=>setTimeout(r,1600));
 const current=JSON.parse(cli(['eval','JSON.stringify({url:location.href})']));
 if(new URL(current.url).hostname==='chatgpt.com')throw Error('Private-app redirect; no page extraction or account content written');
 const common="url:location.href,title:document.title,meta:[...document.querySelectorAll('meta[name],meta[property],time,link[rel=canonical]')].map(x=>x.outerHTML)";
 const expr=mode==='social'?"({url:location.href,title:document.title,timezone:Intl.DateTimeFormat().resolvedOptions().timeZone,cards:[...document.querySelectorAll('article[data-testid=tweet]')].map(e=>({text:e.innerText,times:[...e.querySelectorAll('time')].map(t=>({text:t.innerText,datetime:t.dateTime})),links:[...e.querySelectorAll('a[href]')].map(a=>({text:a.innerText,url:a.href})),metrics:[...e.querySelectorAll('[aria-label]')].map(a=>a.getAttribute('aria-label'))}))})":`({${common},text:document.body.innerText,links:[...document.querySelectorAll('a[href]')].map(a=>({text:a.innerText,url:a.href})),jsonld:[...document.querySelectorAll('script[type="application/ld+json"]')].map(e=>e.textContent)})`;
 const data=JSON.parse(cli(['eval','JSON.stringify('+expr+')']));
 if(mode==='social') data.original_posts=JSON.parse(cli(['eval',`JSON.stringify((()=>{const select=(t,depth=0)=>{if(!t)return null;return {id:t.id_str,text:t.note_tweet?.note_tweet_results?.result?.text||t.full_text||t.text,created_at:t.created_at,is_quote_status:t.is_quote_status,in_reply_to_status_id_str:t.in_reply_to_status_id_str,in_reply_to_screen_name:t.in_reply_to_screen_name,author:t.user?.screen_name,likes:t.favorite_count,replies:t.reply_count,retweets:t.retweet_count,bookmarks:t.bookmark_count,views:t.views,quoted_url:t.quoted_status_permalink?.expanded,quoted:depth<1?select(t.quoted_status,depth+1):null}};return [...document.querySelectorAll('article[data-testid=tweet]')].map(e=>{let f=e[Object.keys(e).find(k=>k.startsWith('__reactFiber'))];for(let i=0;f&&i<45;i++,f=f.return){if(f.memoizedProps?.tweet)return select(f.memoizedProps.tweet)}return null})})())`]));
 // Social extraction never includes navigation, composer, avatar URLs, or account-menu data.
 function clean(o){if(typeof o==='string'){return o.replace(/([?&])utm_[^&#\s"<>]+/g,'$1').replace(/\?&/g,'?').replace(/[?&]$/,'');}if(Array.isArray(o))return o.map(clean);if(o&&typeof o==='object')return Object.fromEntries(Object.entries(o).map(([k,v])=>[k,clean(v)]));return o;}
 const safe=clean(data); safe.captured_bj=record.time_bj;
 if(mode==='social')safe.title='Public post cards';
 fs.writeFileSync(base+name+'.json',JSON.stringify(safe,null,2),{flag:'wx'});
 fs.writeFileSync(base+name+'.txt',safe.text??safe.cards.map(c=>c.text).join('\n\n---POST---\n\n')+'\n\nORIGINAL PUBLIC POST FIELDS\n'+JSON.stringify(safe.original_posts,null,2),{flag:'wx'});
 record.result='captured; validate body separately'; record.chars=(safe.text??JSON.stringify(safe.cards)).length;
 console.log(JSON.stringify(record));
}catch(e){record.result='FAILED: '+String(e.message).slice(0,500);console.log(JSON.stringify(record));}
fs.appendFileSync(base+'d-capture-records.jsonl',JSON.stringify(record)+'\n');
