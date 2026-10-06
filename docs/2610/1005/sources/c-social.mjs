import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
const mode=process.argv[2],name=process.argv[3];
const js=mode==='reddit'?'JSON.stringify({url:location.href,post:[...document.querySelectorAll("shreddit-post")].map(e=>({title:e.getAttribute("post-title"),author:e.getAttribute("author"),score:e.getAttribute("score"),comments:e.getAttribute("comment-count"),created:e.getAttribute("created-timestamp"),text:e.querySelector("[slot=text-body]")?.innerText})),comments:[...document.querySelectorAll("shreddit-comment")].map(e=>({id:e.getAttribute("thingid"),author:e.getAttribute("author"),score:e.getAttribute("score"),created:e.querySelector("time")?.dateTime,text:e.querySelector("[slot=comment]")?.innerText}))})':'JSON.stringify({url:location.href,posts:[...document.querySelectorAll("article[data-testid=tweet]")].map(e=>({text:e.querySelector("[data-testid=tweetText]")?.innerText,time:e.querySelector("time")?.dateTime,links:[...e.querySelectorAll("a[href*=status]")].map(a=>a.href.split("?")[0]),engagement:[...e.querySelectorAll("[role=group]")].map(x=>x.getAttribute("aria-label"))}))})';
const out=execFileSync('opencli.exe',['browser','1005-c','eval',js],{encoding:'utf8',timeout:150000,maxBuffer:4000000});
const data=JSON.parse(out.split('\n')[0]);
data.captured_bj=new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
// Allowlisted public post fields only: no navigation, account menu, reply composer or avatar URLs.
fs.writeFileSync(`docs/2610/1005/sources/c-${name}.json`,JSON.stringify(data,null,2),{flag:'wx'});
fs.appendFileSync('docs/2610/1005/sources/c-capture-records.jsonl',JSON.stringify({time_bj:data.captured_bj,url:data.url,tool:'opencli browser 1005-c scoped public post extraction',result:`${name}: ${data.posts?.length??data.comments?.length} records; collector fields excluded before write`})+'\n');
console.log(JSON.stringify(data));
