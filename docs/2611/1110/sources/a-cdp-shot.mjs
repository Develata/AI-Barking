// 1110 A 组：用 headless Chrome + CDP 渲染页面、执行一段展开折叠区的 JS、整页截图（2 倍像素）。
// 用法：node a-cdp-shot.mjs <url> <out.png> [--width 700] [--js "<表达式>"] [--wait 3000] [--hide "<css选择器,...>"] [--dark]
// 只展开折叠区、隐藏站点导航/弹窗，不改正文文字。未登录、临时 user-data-dir。
import { spawn } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const args = process.argv.slice(2);
const url = args[0], out = args[1];
const opt = (k, d) => { const i = args.indexOf(k); return i >= 0 ? args[i + 1] : d; };
const width = +opt('--width', 700), wait = +opt('--wait', 3000);
const js = opt('--js', ''), hide = opt('--hide', ''), find = opt('--find', ''), clips = opt('--clips', '');
const chrome = process.env.BARKING_CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const port = 9300 + Math.floor(Math.random() * 500);
const dir = mkdtempSync(join(tmpdir(), 'a-cdp-'));
const p = spawn(chrome, ['--headless=new', `--remote-debugging-port=${port}`, `--user-data-dir=${dir}`, '--no-first-run', '--hide-scrollbars', '--force-device-scale-factor=2', `--window-size=${width},1000`, 'about:blank'], { stdio: 'ignore' });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let list;
for (let i = 0; i < 40; i++) { try { list = await (await fetch(`http://127.0.0.1:${port}/json`)).json(); if (list.length) break; } catch {} await sleep(250); }
const page = list.find((t) => t.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0; const pend = new Map();
ws.onmessage = (m) => { const d = JSON.parse(m.data); if (d.id && pend.has(d.id)) { pend.get(d.id)(d); pend.delete(d.id); } };
const send = (method, params = {}) => new Promise((r) => { const i = ++id; pend.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
await send('Page.enable');
await send('Emulation.setDeviceMetricsOverride', { width, height: 1000, deviceScaleFactor: 2, mobile: false });
await send('Page.navigate', { url });
await sleep(wait);
if (js) { const r = await send('Runtime.evaluate', { expression: js, awaitPromise: true, returnByValue: true }); console.log('js ->', JSON.stringify(r.result?.result?.value ?? r.result)); await sleep(4000); }
if (find) { const expr = `(()=>{const out=[];for(const s of ${JSON.stringify(find)}.split('|')){const w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);let n;while(n=w.nextNode()){if(n.nodeValue.includes(s)){const r=document.createRange();r.selectNodeContents(n);const b=r.getBoundingClientRect();out.push(s.slice(0,40)+' => top '+Math.round(b.top+scrollY)+' bottom '+Math.round(b.bottom+scrollY));}}}return out})()`; const r = await send('Runtime.evaluate', { expression: expr, returnByValue: true }); console.log((r.result?.result?.value||[]).join('\n')); }
if (hide) await send('Runtime.evaluate', { expression: `(()=>{const s=document.createElement('style');s.textContent=${JSON.stringify(hide + '{display:none !important}')};document.head.appendChild(s);return 1})()` });
await sleep(500);
if (clips) { // 名称=y0-y1（CSS px）；不改视口，按同一布局直接截区间，输出 <out>-<名称>.png
  for (const c of clips.split(';')) { const [nm, rg] = c.split('='); const [y0, y1] = rg.split('-').map(Number);
    const s = await send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: true, clip: { x: 0, y: y0, width, height: y1 - y0, scale: 1 } });
    const f = out.replace(/\.png$/, '') + '-' + nm + '.png'; writeFileSync(f, Buffer.from(s.result.data, 'base64')); console.log('clip', f); }
  ws.close(); p.kill(); process.exit(0); }
const h = (await send('Runtime.evaluate', { expression: 'Math.ceil(Math.max(document.documentElement.scrollHeight, document.body.scrollHeight))', returnByValue: true })).result.result.value;
console.log('height css px', h);
await send('Emulation.setDeviceMetricsOverride', { width, height: h, deviceScaleFactor: 2, mobile: false });
await sleep(800);
const shot = await send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
writeFileSync(out, Buffer.from(shot.result.data, 'base64'));
console.log('wrote', out, width * 2, 'x', h * 2);
ws.close(); p.kill();
