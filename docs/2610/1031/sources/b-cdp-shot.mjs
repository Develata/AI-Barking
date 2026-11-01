// B 组：headless Chrome（全新临时 profile，匿名，不读用户 Cookie）经 CDP 渲染页面，按元素选择器或像素范围截 2x 图。
// 用法：bun b-cdp-shot.mjs <url> <out.png|-> <mode> [arg]
//   mode=tweet            截 X 官方嵌入页里的帖子卡片（article 元素）
//   mode=text:<css选择器> 截 selector 匹配的第 N 个到第 M 个元素的并集，arg 形如 "0-3"
//   mode=dump             只输出各块元素的位置，供选择裁切边界
import { spawn } from "bun";
import { mkdirSync, writeFileSync, appendFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const [url, out, mode, arg] = process.argv.slice(2);
const port = 9400 + Math.floor(Math.random() * 50);
const prof = join(tmpdir(), `b-cdp-prof-${Date.now()}`); mkdirSync(prof, { recursive: true });
const chrome = spawn(["google-chrome", "--headless=new", `--remote-debugging-port=${port}`, `--user-data-dir=${prof}`, "--no-first-run", "--lang=en-US", "--accept-lang=en-US", "--window-size=900,1000", "--disable-gpu", "about:blank"]);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let targets;
for (let i = 0; i < 40; i++) { try { targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json(); if (targets.length) break; } catch {} await sleep(500); }
const page = targets.find((t) => t.type === "page");
const ws = new WebSocket(page.webSocketDebuggerUrl); await new Promise((r) => (ws.onopen = r));
let id = 0; const pending = new Map();
ws.onmessage = (e) => { const m = JSON.parse(e.data); if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); } };
const send = (method, params = {}) => new Promise((res) => { const i = ++id; pending.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
const ev = async (expr) => (await send("Runtime.evaluate", { expression: expr, returnByValue: true, awaitPromise: true })).result?.result?.value;
await send("Page.enable");
await send("Emulation.setLocaleOverride", { locale: "en-US" });
await send("Emulation.setTimezoneOverride", { timezoneId: "Asia/Shanghai" });
await send("Emulation.setDeviceMetricsOverride", { width: 700, height: 1000, deviceScaleFactor: 2, mobile: false });
await send("Network.enable");
await send("Network.setExtraHTTPHeaders", { headers: { "Accept-Language": "en-US,en;q=0.9" } });
await send("Page.navigate", { url });
const sel = mode === "tweet" ? "article" : mode.startsWith("text:") ? mode.slice(5) : "body";
let ok = false;
for (let i = 0; i < 40; i++) { await sleep(1000); const n = await ev(`document.querySelectorAll(${JSON.stringify(sel)}).length`); const len = await ev("document.body.innerText.length"); if (n > 0 && len > 80) { ok = true; break; } }
await sleep(3500);
let mediaRect = null;
if (mode === "tweet") { mediaRect = await ev("(()=>{const a=document.querySelector('article');const ar=a.getBoundingClientRect();let tb=0;a.querySelectorAll('[data-testid=tweetText]').forEach(e=>{tb=Math.max(tb,e.getBoundingClientRect().bottom)});const t=a.querySelector('time');const tt=t?t.getBoundingClientRect().top:0;return {textBottom:Math.round(tb-ar.top),timeTop:Math.round(tt-ar.top),h:Math.round(ar.height)}})()"); console.log("MEDIA", JSON.stringify(mediaRect)); }
const log = (o) => appendFileSync(join(import.meta.dir, "b-shots-log.jsonl"), JSON.stringify({ tool: "b-cdp-shot.mjs", url, out, mode, arg, ts: new Date().toISOString(), ...o }) + "\n");
if (!ok) { console.log("FAILED no selector", sel, await ev("document.title")); log({ result: "failed" }); chrome.kill(); process.exit(1); }
if (mode === "dom") {
  console.log(await ev("(()=>{const a=document.querySelector('article');const out=[];const walk=(e,d)=>{const r=e.getBoundingClientRect();if(r.height>150&&d<12)out.push(' '.repeat(d)+e.tagName+' h='+Math.round(r.height)+' cls='+(e.className||'').toString().slice(0,50)+' '+(e.getAttribute('data-testid')||''));for(const c of e.children)walk(c,d+1)};walk(a,0);return out.join('\\n')})()"));
  chrome.kill(); process.exit(0);
}
if (mode === "dump") {
  const boxes = await ev(`[...document.querySelectorAll('h1,h2,p,figure,figcaption')].map((e,i)=>{const r=e.getBoundingClientRect();return i+' '+e.tagName+' y='+Math.round(r.y+scrollY)+' h='+Math.round(r.height)+' '+e.innerText.slice(0,60).replace(/\\n/g,' ')}).join('\\n')`);
  console.log(boxes); console.log(await ev("document.body.innerText.slice(0,400)")); chrome.kill(); process.exit(0);
}
let rect;
if (mode === "tweet") rect = await ev(`(()=>{const r=document.querySelector('article').getBoundingClientRect();return {x:r.x,y:r.y+scrollY,w:r.width,h:r.height}})()`);
else { const [a, b] = arg.split("-").map(Number); rect = await ev(`(()=>{const els=[...document.querySelectorAll(${JSON.stringify(sel)})].slice(${a},${b + 1});let x0=1e9,y0=1e9,x1=0,y1=0;for(const e of els){const r=e.getBoundingClientRect();x0=Math.min(x0,r.x);y0=Math.min(y0,r.y+scrollY);x1=Math.max(x1,r.right);y1=Math.max(y1,r.bottom+scrollY)}return {x:x0,y:y0,w:x1-x0,h:y1-y0}})()`); }
const pad = mode === "tweet" ? 0 : 16;
const clip = { x: Math.max(0, rect.x - pad), y: Math.max(0, rect.y - pad), width: rect.w + 2 * pad, height: rect.h + 2 * pad, scale: 1 };
const shot = await send("Page.captureScreenshot", { format: "png", captureBeyondViewport: true, clip });
writeFileSync(out, Buffer.from(shot.result.data, "base64"));
const text = await ev("document.body.innerText");
console.log("OK", JSON.stringify(clip), "title:", await ev("document.title"));
log({ result: "ok", clip, innerTextHead: text.slice(0, 120) });
chrome.kill(); process.exit(0);
