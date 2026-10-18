// B 组自写：headless Chrome + CDP 截图（2 倍像素比、指定 CSS 宽度），用于 opencli 对重型滚动页面卡死的情况。
// 用法：node b-cdp-shot.mjs <url> <out.png> <cssWidth> "<JS 表达式：滚到目标并返回 {x,y,w,h}（文档坐标，CSS px）>" [waitMs]
// 不读取任何浏览器 Cookie：使用全新临时 user-data-dir；匿名会话。
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [url, out, widthStr, expr, waitStr] = process.argv.slice(2);
const width = Number(widthStr);
const VH = Number(process.env.VH || 1400);
const wait = Number(waitStr || 6000);
const chrome = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
const dir = mkdtempSync(join(tmpdir(), "b-cdp-"));
const port = 9300 + Math.floor(Math.random() * 500);
const p = spawn(chrome, ["--headless=new", `--remote-debugging-port=${port}`, `--user-data-dir=${dir}`,
  "--disable-gpu", "--hide-scrollbars", "--no-first-run", `--window-size=${width},1400`, "about:blank"], { stdio: "ignore" });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
try {
  let targets;
  for (let i = 0; i < 40; i++) {
    try { targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json(); if (targets.length) break; } catch {}
    await sleep(500);
  }
  const t = targets.find((x) => x.type === "page");
  const ws = new WebSocket(t.webSocketDebuggerUrl);
  await new Promise((r) => (ws.onopen = r));
  let id = 0; const pending = new Map();
  ws.onmessage = (m) => { const d = JSON.parse(m.data); if (d.id && pending.has(d.id)) { pending.get(d.id)(d); pending.delete(d.id); } };
  const send = (method, params = {}) => new Promise((res) => { const i = ++id; pending.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
  const ev = async (e) => { const r = await send("Runtime.evaluate", { expression: e, awaitPromise: true, returnByValue: true }); if (r.result?.exceptionDetails) console.log("EXC", JSON.stringify(r.result.exceptionDetails).slice(0,400)); return r.result?.result?.value ?? r; };
  await send("Page.enable");
  await send("Emulation.setDeviceMetricsOverride", { width, height: VH, deviceScaleFactor: 2, mobile: false });
  await send("Page.navigate", { url });
  await sleep(wait);
  // 逐步滚动以触发懒加载
  const H = await ev("document.documentElement.scrollHeight");
  for (let y = 0; y < H; y += 700) { await ev(`window.scrollTo(0,${y})`); await sleep(120); }
  await sleep(1500);
  const box = await ev(expr);
  await sleep(2500);
  console.log("clip", JSON.stringify(box));
  const shot = await send("Page.captureScreenshot", { format: "png", captureBeyondViewport: false,
    clip: { x: box.x, y: box.y, width: box.w, height: box.h, scale: 1 } });
  writeFileSync(out, Buffer.from(shot.result.data, "base64"));
  console.log("saved", out);
  ws.close();
} finally { p.kill(); }
