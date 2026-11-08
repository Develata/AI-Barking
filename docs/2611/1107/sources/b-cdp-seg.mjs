// B 组：用 headless Chrome（CDP）把 doubleai 文章按 4500 CSS px 分段截成 2 倍像素 PNG，
// 先滚动一遍触发懒加载 iframe；同时输出关键元素的页面 y 坐标。匿名、无登录、无 Cookie。
// 用法：node b-cdp-seg.mjs <url> <输出目录> [宽度=700]
import { spawn } from 'node:child_process';
import { mkdtempSync, writeFileSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
const [url, outDir, W = '700', clipsFile] = process.argv.slice(2);
import { readFileSync } from 'node:fs';
mkdirSync(outDir, { recursive: true });
const port = 9333;
const profile = mkdtempSync(join(tmpdir(), 'b-cdp-'));
const chrome = spawn('C:/Program Files/Google/Chrome/Application/chrome.exe',
  ['--headless=new', `--remote-debugging-port=${port}`, `--user-data-dir=${profile}`, '--no-first-run', '--hide-scrollbars', 'about:blank'], { stdio: 'ignore' });
const sleep = ms => new Promise(r => setTimeout(r, ms));
let targets;
for (let i = 0; i < 40; i++) { try { targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json(); if (targets.length) break; } catch {} await sleep(250); }
const page = targets.find(t => t.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise(r => ws.addEventListener('open', r));
let id = 0; const pend = new Map();
ws.addEventListener('message', e => { const m = JSON.parse(e.data); if (m.id && pend.has(m.id)) { pend.get(m.id)(m); pend.delete(m.id); } });
const send = (method, params = {}) => new Promise(r => { const i = ++id; pend.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
const ev = async expr => (await send('Runtime.evaluate', { expression: expr, returnByValue: true, awaitPromise: true })).result.result.value;
await send('Page.enable');
await send('Emulation.setDeviceMetricsOverride', { width: +W, height: +(process.env.VH||1000), deviceScaleFactor: +(process.env.DPR||2), mobile: false });
await send('Page.navigate', { url });
await sleep(6000);
let H = await ev('document.documentElement.scrollHeight');
for (let y = 0; y < H; y += 700) { await ev(`window.scrollTo(0,${y})`); await sleep(250); H = await ev('document.documentElement.scrollHeight'); }
await ev('window.scrollTo(0,0)'); await sleep(1500);
H = await ev('document.documentElement.scrollHeight');
const marks = await ev(`(function(){try{return JSON.stringify(Array.from(document.querySelectorAll('figcaption,h2,h3,pre,figure')).map(function(e){var r=e.getBoundingClientRect();return {t:e.tagName,y:Math.round(r.top+window.scrollY),h:Math.round(r.height),s:(e.innerText||'').slice(0,70).replace(/\s+/g,' ')}}))}catch(x){return 'ERR '+x}})()`) || 'null';
writeFileSync(join(outDir, 'marks.json'), marks);
console.log('docHeight', H);
if (process.env.FINDS) {
  for (const t of process.env.FINDS.split('|')) {
    const y = await ev(`(function(){var w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);var n;while(n=w.nextNode()){var i=n.nodeValue.indexOf(${JSON.stringify(t)});if(i>=0){var r=document.createRange();r.setStart(n,i);r.setEnd(n,i+1);var b=r.getBoundingClientRect();return [Math.round(b.top+scrollY),Math.round(b.left)]}}return null})()`);
    console.log('FIND', t, JSON.stringify(y));
  }
}
if (clipsFile) {
  // 模式二：按 [{name,y0,y1}]（CSS px）直接截指定页面区间，写 <name>.png
  for (const c of JSON.parse(readFileSync(clipsFile, 'utf-8'))) {
    const r = await send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: true, clip: { x: 0, y: c.y0, width: +W, height: c.y1 - c.y0, scale: 1 } });
    writeFileSync(join(outDir, c.name + '.png'), Buffer.from(r.result.data, 'base64'));
    console.log('clip', c.name, c.y0, c.y1);
  }
} else {
const SEG = 4500; let n = 0;
for (let y = 0; y < H; y += SEG, n++) {
  const h = Math.min(SEG, H - y);
  const r = await send('Page.captureScreenshot', { format: 'png', captureBeyondViewport: true, clip: { x: 0, y, width: +W, height: h, scale: 1 } });
  writeFileSync(join(outDir, `seg-${String(n).padStart(2, '0')}.png`), Buffer.from(r.result.data, 'base64'));
}
console.log('segments', n);
}
ws.close(); chrome.kill();
