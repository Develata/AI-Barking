// B 组：用有界面的 Chrome（全新临时 profile，不读用户 Cookie）经 CDP 取 help.openai.com 渲染文本、页面数据与整页 2x 截图。
// 不绕过验证：只等页面自己通过 Cloudflare 的自动检测；超时就记失败。
// 用法：bun b-cdp-shot.mjs <url> <outdir> <name> [width=700]
import { spawn } from "bun";
import { mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [url, outdir, name, widthArg, maxHArg] = process.argv.slice(2);
const width = Number(widthArg || 700);
const port = 9340 + Math.floor(Math.random() * 50);
const prof = join(tmpdir(), `b-cdp-prof-${Date.now()}`);
mkdirSync(prof, { recursive: true });
const chrome = spawn([
  "C:/Program Files/Google/Chrome/Application/chrome.exe",
  `--remote-debugging-port=${port}`,
  `--user-data-dir=${prof}`,
  "--no-first-run", "--no-default-browser-check", "--lang=en-US", "--accept-lang=en-US",
  "--window-size=900,1000",
  "about:blank",
]);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let targets;
for (let i = 0; i < 40; i++) {
  try { targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json(); if (targets.length) break; } catch {}
  await sleep(500);
}
const page = targets.find((t) => t.type === "page");
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0; const pending = new Map();
ws.onmessage = (e) => { const m = JSON.parse(e.data); if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); } };
const send = (method, params = {}) => new Promise((res) => { const i = ++id; pending.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
const ev = async (expr) => (await send("Runtime.evaluate", { expression: expr, returnByValue: true, awaitPromise: true })).result?.result?.value;

await send("Page.enable");
await send("Emulation.setLocaleOverride", { locale: "en-US" });
await send("Network.enable");
await send("Network.setExtraHTTPHeaders", { headers: { "Accept-Language": "en-US,en;q=0.9" } });
await send("Page.navigate", { url });
let ok = false;
for (let i = 0; i < 60; i++) {
  await sleep(1000);
  const t = await ev("document.title");
  if (t && !/请稍候|Just a moment|Attention Required/i.test(t) && (await ev("document.body.innerText.length")) > Number(process.env.MINLEN || 2000)) { ok = true; break; }
}
const title = await ev("document.title");
if (!ok) { console.log("FAILED: still challenge page, title =", title); chrome.kill(); process.exit(1); }
await sleep(2500);
await send("Emulation.setDeviceMetricsOverride", { width, height: 1000, deviceScaleFactor: 2, mobile: false });
await sleep(1500);
const text = await ev("document.body.innerText");
const meta = await ev(`(()=>{const a=window.__reactRouterContext?.state?.loaderData?.['routes/articles']?.article; return a?{updatedAt:a.updatedAt,fetchedAt:a.fetchedAt,title:a.title,id:a.id,content:a.content}:null})()`);
const h = Math.min(await ev("Math.ceil(document.documentElement.scrollHeight)"), Number(maxHArg || 1e9));
const boxes = await ev(`[...document.querySelectorAll('h1,h2,h3,p,table,li')].map(e=>{const r=e.getBoundingClientRect();return {tag:e.tagName,text:e.innerText.slice(0,70),x:Math.round(r.x),y:Math.round(r.y+scrollY),w:Math.round(r.width),h:Math.round(r.height)}})`);
const shot = await send("Page.captureScreenshot", { format: "png", captureBeyondViewport: true, clip: { x: 0, y: 0, width, height: h, scale: 1 } });
writeFileSync(join(outdir, `${name}.png`), Buffer.from(shot.result.data, "base64"));
writeFileSync(join(outdir, `${name}.txt`), text);
writeFileSync(join(outdir, `${name}-article.json`), JSON.stringify(meta, null, 1));
writeFileSync(join(outdir, `${name}-boxes.json`), JSON.stringify({ height: h, width, boxes }, null, 1));
console.log("OK", title, "height", h, "updatedAt", meta?.updatedAt);
chrome.kill();
process.exit(0);
