import fs from "node:fs";
import {execFileSync} from "node:child_process";
const sourceDir="docs/2610/1004/sources";
const pages=[
["a-aihot-daily-20261003","https://aihot.news/daily/2026-10-03"],
["a-aihot-daily-20261004","https://aihot.news/daily/2026-10-04"],
["a-aihot-perl","https://aihot.news/items/w0twto4412g72n2ryamc6xpi1"],
["a-aihot-slack","https://aihot.news/items/oqxyqwdulf3zj8plv1ckcl4cj"],
["a-techcrunch","https://techcrunch.com/2026/10/03/openai-safety-employee-resigns-claiming-the-companys-culture-is-broken/"],
["a-guardian","https://www.theguardian.com/technology/2026/oct/03/openai-safety-leader-quits-warning-ai-companys-culture-is-broken"],
["a-verge","https://www.theverge.com/ai-artificial-intelligence/1004408/openai-safety-quits-sounding-the-alarm"],
["a-ser","https://cadenaser.com/nacional/2026/10/04/dimite-el-jefe-de-seguridad-de-openai-tras-denunciar-la-falta-de-control-en-la-ia-de-la-empresa-cadena-ser/"],
["a-livemint","https://www.livemint.com/ai/artificial-intelligence/who-is-david-robinson-safety-systems-team-lead-at-openai-joining-the-list-of-execs-who-left-the-ai-firm-this-year-11791007693908.html"]
];
const clean=(raw)=>{const u=new URL(raw);for(const k of [...u.searchParams.keys()])if(k.toLowerCase().startsWith("utm_"))u.searchParams.delete(k);return u.toString()};
const now=()=>new Date(Date.now()+8*3600000).toISOString().replace("Z","+08:00");
const logs=[];
const pause=(ms)=>new Promise(r=>setTimeout(r,ms));
for(const [name,url] of pages){
 const started=now();
 try{
  const opened=execFileSync("opencli.exe",["browser","1004-a","open",url],{encoding:"utf8",timeout:90000});
  await pause(600);
  const js="JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time,link[rel=canonical]')].map(x=>x.outerHTML)})";
  let raw=execFileSync("opencli.exe",["browser","1004-a","eval",js],{encoding:"utf8",timeout:90000}).split("\n  Update available:")[0].trim();
  const d=JSON.parse(raw); d.url=clean(d.url);
  fs.writeFileSync(sourceDir+"/"+name+".json",JSON.stringify(d,null,2),"utf8");
  fs.writeFileSync(sourceDir+"/"+name+".txt",d.text||"","utf8");
  logs.push({time_bj:started,url:clean(url),tool:"opencli browser 1004-a open/eval",result:"成功；"+(d.text||"").length+" 字符；标题="+d.title,files:[name+".json",name+".txt"]});
  console.log(name,started,d.title,(d.text||"").length);
 }catch(e){
  logs.push({time_bj:started,url:clean(url),tool:"opencli browser 1004-a open/eval",result:"失败："+String(e.message||e).slice(0,400),files:[]});
  console.log(name,started,"FAILED",String(e.message||e).slice(0,300));
 }
 fs.writeFileSync(sourceDir+"/a-followup-capture.jsonl",logs.map(x=>JSON.stringify(x)).join("\n")+"\n","utf8");
}

