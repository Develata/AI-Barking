// a-xcard.ts: 用匿名 headless Chrome 渲染 X 官方嵌入页，点“Show more”展开长帖，只截帖子卡片本身（2 倍像素比）。
// 用法：bun a-xcard.ts <tweetId> <out.png>
// 不读本机浏览器 Cookie：user-data-dir 为全新临时目录。
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [id, out] = process.argv.slice(2);
const chrome = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
const port = 9300 + Math.floor(Math.random() * 500);
const dir = mkdtempSync(join(tmpdir(), "xcard-"));
const p = spawn(chrome, [
  "--headless=new", `--remote-debugging-port=${port}`, `--user-data-dir=${dir}`,
  "--window-size=700,1600", "--force-device-scale-factor=2", "about:blank",
], { stdio: "ignore" });
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
try {
  let targets: any[] = [];
  for (let i = 0; i < 40; i++) {
    try { targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json(); if (targets.length) break; } catch {}
    await sleep(300);
  }
  const ws = new WebSocket(targets.find((t) => t.type === "page").webSocketDebuggerUrl);
  await new Promise((r) => (ws.onopen = r));
  let n = 0; const pend = new Map<number, (v: any) => void>();
  ws.onmessage = (m) => { const d = JSON.parse(m.data as string); if (d.id && pend.has(d.id)) pend.get(d.id)!(d.result); };
  const send = (method: string, params: any = {}) => new Promise<any>((r) => { const i = ++n; pend.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
  await send("Page.enable");
  await send("Emulation.setDeviceMetricsOverride", { width: 700, height: 1600, deviceScaleFactor: 2, mobile: false });
  await send("Page.navigate", { url: `https://platform.twitter.com/embed/Tweet.html?id=${id}&theme=light&dnt=true` });
  await sleep(9000);
  const ev = (expression: string) => send("Runtime.evaluate", { expression, returnByValue: true });
  // 点“Show more”（若有）
  await ev(`(()=>{const a=[...document.querySelectorAll('a,button,span')].find(e=>/^Show more$/i.test((e.textContent||'').trim()));if(a){a.click();return 'clicked'}return 'none'})()`);
  await sleep(3000);
  const r = await ev(`(()=>{const e=document.querySelector('article')||document.querySelector('[data-testid=tweet]')||document.body.firstElementChild;const b=e.getBoundingClientRect();return {x:b.x,y:b.y,w:b.width,h:b.height}})()`);
  const b = r.result.value;
  const shot = await send("Page.captureScreenshot", { format: "png", captureBeyondViewport: true, clip: { x: b.x, y: b.y, width: b.w, height: b.h, scale: 1 } });
  writeFileSync(out, Buffer.from(shot.data, "base64"));
  console.log("saved", out, JSON.stringify(b));
  ws.close();
} finally { p.kill(); }
