import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const [name,mode,arg]=process.argv.slice(2);
const base='docs/2610/1005/sources/';
const cli=a=>execFileSync('opencli.exe',['browser','1005-d',...a],{encoding:'utf8',timeout:170000,maxBuffer:16000000,env:{...process.env,OPENCLI_BROWSER_COMMAND_TIMEOUT:'150'}}).split('\n  Update available:')[0].trim();
const js={
 reddit:`({url:location.href,post:[...document.querySelectorAll('shreddit-post')].map(e=>({title:e.getAttribute('post-title'),score:e.getAttribute('score'),created:e.getAttribute('created-timestamp'),text:e.innerText})),comments:[...document.querySelectorAll('shreddit-comment')].slice(0,30).map(e=>({id:e.getAttribute('thingid'),score:e.getAttribute('score'),permalink:e.getAttribute('permalink'),text:e.querySelector('[slot=comment]')?.innerText||null}))})`,
 hn:`({url:location.href,comments:[...document.querySelectorAll('tr.comtr')].map(e=>({id:e.id,author:e.querySelector('.hnuser')?.innerText,text:e.querySelector('.commtext')?.innerText,score:e.querySelector('.score')?.innerText||null,time:e.querySelector('.age')?.getAttribute('title')}))})`,
 note:`(()=>{const e=document.querySelector('article');let f=e?.[Object.keys(e).find(k=>k.startsWith('__reactFiber'))];for(let i=0;f&&i<45;i++,f=f.return){const t=f.memoizedProps?.tweet;if(t)return {url:location.href,id:t.id_str,note:t.note_tweet}}return null})()`,
 status:`({url:location.href,title:document.title,public_post_present:!!document.querySelector('article[data-testid=tweet],shreddit-post'),message:document.body.innerText.match(/This post is unavailable|This account doesn’t exist|This community is private|Page not found|该帖子不可用|此帖子不可用|此账号不存在/)?.[0]||null})`,
 beam:`({url:location.href,tables:[...document.querySelectorAll('table')].map(e=>({text:e.innerText,rect:{width:e.getBoundingClientRect().width,height:e.getBoundingClientRect().height},html:e.outerHTML})),buttons:[...document.querySelectorAll('button')].map(e=>({text:e.innerText,role:e.getAttribute('role')}))})`
};
if(!name.startsWith('d-')||!js[mode])throw Error('invalid');
if(arg)cli(['open',arg]);
const data=JSON.parse(cli(['eval','JSON.stringify('+js[mode]+')']));
data.captured_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
fs.writeFileSync(base+name+'.json',JSON.stringify(data,null,2),{flag:'wx'});
fs.appendFileSync(base+'d-capture-records.jsonl',JSON.stringify({time_bj:data.captured_bj,url:data.url,name,tool:'opencli browser 1005-d eval '+mode,result:'public fields captured'})+'\n');
console.log(name,JSON.stringify(data).length);
