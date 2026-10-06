import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const mode=process.argv[2];
const urls={hn:'https://news.ycombinator.com/item?id=49968716',reddit:'https://old.reddit.com/r/ChatGPT/comments/1wymu43/i_read_openais_whole_post_about_watermarking/',search:'https://old.reddit.com/search/?q=textGrain&sort=top&t=week'};
const url=urls[mode];if(!url)throw Error('mode');
const cli=(...a)=>execFileSync('opencli.exe',['browser','1005-b',...a],{encoding:'utf8',timeout:100000});
const started=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
try{
 cli('open',url);await new Promise(r=>setTimeout(r,1800));
 const expr=mode==='hn'?`({title:document.title,story:document.querySelector('.fatitem')?.innerText,comments:[...document.querySelectorAll('.comtr')].map(e=>({id:e.id,text:e.querySelector('.commtext')?.innerText,score:e.querySelector('.score')?.innerText,time:e.querySelector('.age')?.title}))})`:`({title:document.title,posts:[...document.querySelectorAll('.thing.link,.search-result-link')].map(e=>({title:e.querySelector('a.title,.search-title')?.innerText,text:e.querySelector('.usertext-body')?.innerText,score:e.querySelector('.score,.search-score')?.innerText,comments:e.querySelector('a.comments,.search-comments')?.innerText,url:e.querySelector('a.comments,.search-comments')?.href,time:e.querySelector('time')?.dateTime})),comments:[...document.querySelectorAll('.thing.comment')].map(e=>({id:e.id,text:e.querySelector('.usertext-body')?.innerText,score:e.querySelector('.tagline .score')?.title,url:e.querySelector('a.bylink')?.href,time:e.querySelector('time')?.dateTime}))})`;
 const raw=cli('eval','JSON.stringify('+expr+')');
 const data=JSON.parse(raw.split('\n  Update available:')[0].trim());
 // Deliberately omit account header, handles, avatars, reply forms and navigation.
 data.url=url;data.captured_bj=started;
 fs.writeFileSync('docs/2610/1005/sources/b-'+mode+'.json',JSON.stringify(data,null,2));
 fs.appendFileSync('docs/2610/1005/sources/b-captures.jsonl',JSON.stringify({started_bj:started,url,tool:'opencli browser 1005-b targeted public DOM',result:data.title,file:'b-'+mode+'.json'})+'\n');console.log(JSON.stringify(data));
}catch(e){fs.appendFileSync('docs/2610/1005/sources/b-captures.jsonl',JSON.stringify({started_bj:started,url,error:e.message.slice(0,500)})+'\n');console.error(e.message);}
