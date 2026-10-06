import fs from "node:fs";
import {homedir} from "node:os";
import {pathToFileURL} from "node:url";
import {join} from "node:path";
import {execFileSync} from "node:child_process";

const {sendCommand} = await import(pathToFileURL(join(homedir(),"scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js")).href);
const session = "1004-a";
const imageDir = "docs/2610/1004/images";
const pause = (ms) => new Promise((r) => setTimeout(r, ms));
const cdp = (method, params) => sendCommand("cdp", {session, surface:"browser", cdpMethod:method, cdpParams:params});
const cli = (args) => execFileSync("opencli.exe", ["browser", session, ...args], {encoding:"utf8", timeout:90000});
const evalJson = (js) => {
  const raw = cli(["eval", "JSON.stringify(" + js + ")"]).split("\n  Update available:")[0].trim();
  return JSON.parse(raw);
};
const strip = (url) => {
  const u = new URL(url);
  for (const key of [...u.searchParams.keys()]) if (key.toLowerCase().startsWith("utm_")) u.searchParams.delete(key);
  return u.toString();
};
const pages = [
  {name:"01-atlantic-opening.png", url:"https://www.theatlantic.com/technology/2026/10/openai-safety-team-resignation/688881/", top:0, height:2200},
  {name:"02-reuters-response.png", url:"https://www.reuters.com/legal/litigation/openai-safety-employee-quits-says-time-trial-error-is-over-2026-10-03/", top:0, height:2300},
  {name:"03-reports-index.png", url:"https://alignment.openai.com/misalignment-reports/", full:true},
  {name:"04-perl-report.png", url:"https://alignment.openai.com/misalignment-reports/command-injecting-a-reference-tool-to-copy-a-source-file/", top:0, height:2300},
  {name:"05-eda-summary.png", url:"https://alignment.openai.com/misalignment-reports/reaching-an-internal-eda-host-through-a-reference-tool/", top:0, height:1100},
  {name:"06-eda-id-attempt.png", url:"https://alignment.openai.com/misalignment-reports/reaching-an-internal-eda-host-through-a-reference-tool/", anchor:"first successful command on the EDA machine", before:500, height:1200},
  {name:"07-slack-summary.png", url:"https://alignment.openai.com/misalignment-reports/preparing-for-a-restart-after-reading-slack/", top:0, height:1500},
  {name:"08-slack-cot-response.png", url:"https://alignment.openai.com/misalignment-reports/preparing-for-a-restart-after-reading-slack/", anchor:"survival/continuity", before:500, height:2100},
  {name:"09-marcus-post.png", url:"https://x.com/Marcus_J_W/status/2106203042140102868", tweet:true},
];
const log = [];
for (const p of pages) {
  if (process.argv[2] && process.argv[2] !== p.name) continue;
  const at = new Date(Date.now()+8*3600000).toISOString().replace("Z","+08:00");
  try {
    cli(["open", p.url]);
    await pause(900);
    await cdp("Emulation.setDeviceMetricsOverride", {width:700,height:1200,deviceScaleFactor:2,mobile:false});
    await pause(700);
    const dims = evalJson("{width:document.documentElement.clientWidth,scrollHeight:document.documentElement.scrollHeight,title:document.title}");
    let y = p.top ?? 0;
    let width = 700;
    let height = p.height;
    let x = 0;
    if (p.tweet) {
      evalJson("(()=>{const b=[...document.querySelectorAll('button,[role=button]')].find(x=>/显示原文|show original/i.test((x.innerText||'')+' '+(x.getAttribute('aria-label')||'')));if(b)b.click();return !!b})()");
      await pause(900);
      const card = evalJson("(()=>{const es=[...document.querySelectorAll('article[data-testid=tweet]')];const e=es.find(x=>(x.innerText||'').includes('New OpenAI misalignment disclosures!'));if(!e)return null;const r=e.getBoundingClientRect();return {x:Math.max(0,Math.floor(r.x)),y:Math.max(0,Math.floor(r.y+window.scrollY)),width:Math.ceil(r.width),height:Math.ceil(r.height),text:e.innerText}})()");
      if (!card) throw new Error("tweet card not found");
      x = card.x; y = card.y; width = card.width; height = card.height;
    }
    if (p.anchor) {
      const q = JSON.stringify(p.anchor.toLowerCase());
      const found = evalJson("(()=>{const s=" + q + ";const es=[...document.querySelectorAll('h1,h2,h3,p,li,blockquote,pre,div')].filter(x=>(x.innerText||'').toLowerCase().includes(s)).sort((a,b)=>(a.innerText||'').length-(b.innerText||'').length);const e=es[0];return e?Math.round(e.getBoundingClientRect().top+window.scrollY):-1})()");
      if (found < 0) throw new Error("anchor not found: " + p.anchor);
      y = Math.max(0, found - (p.before || 0));
    }
    if (p.full) height = Math.min(dims.scrollHeight, 9000);
    if (y + height > dims.scrollHeight && !p.tweet) height = Math.max(1, dims.scrollHeight - y);
    const shot = await cdp("Page.captureScreenshot", {format:"png",captureBeyondViewport:true,clip:{x,y,width,height,scale:1}});
    fs.writeFileSync(join(imageDir,p.name), Buffer.from(shot.data,"base64"));
    const info = {time_bj:at,url:strip(p.url),tool:"opencli browser 1004-a + CDP Page.captureScreenshot; deviceScaleFactor 2",file:p.name,css:{x,y,width,height},pixels:"expected 2x CSS dimensions",page_title:dims.title,page_scroll_height:dims.scrollHeight,result:"success"};
    log.push(info);
    console.log(JSON.stringify(info));
  } catch (e) {
    const info={time_bj:at,url:strip(p.url),tool:"opencli browser 1004-a + CDP",file:p.name,result:"failed: "+String(e.message||e).slice(0,300)};
    log.push(info);
    console.log(JSON.stringify(info));
  }
  fs.writeFileSync("docs/2610/1004/sources/a-screenshot-records.jsonl",log.map(x=>JSON.stringify(x)).join("\n")+"\n","utf8");
}
