// 1007 期 C 组：匿名 headless Chrome（Playwright Core）按 700 CSS px 宽、DPR 2 打开页面，
// 存 DOM 文本/元数据/布局，并按“起止文字锚点”裁切截图（只裁切，不改像素）。
// 用法：node docs/2610/1007/sources/c-browser.mjs <name> <url> [shots.json | -] [--layout] [--full <file>] [--height <px>] [--wait <ms>]
//   shots.json: [{"file":"25-c-xxx.png","from":"起始文字","to":"结束文字","padTop":24,"padBottom":24,"fromIdx":0,"toIdx":0,"toTop":false,"fig":N,"toFig":M}]
//   from/to 为元素文字（取最内层命中元素的上沿/下沿）；fig/toFig 取第 N 个 <figure>（用于无文字的图表）
//   --no-archive  不写 .json/.txt（已存档过，只补截图）
//   --layout  只打印布局清单（标题/段落/图片的 y 与高度），不写 images
//   --full f  把整页截图写到给定路径（用于目视检查，通常写到临时目录而不是 images/）
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const {chromium} = await import(pathToFileURL(path.join(os.homedir(), 'scoop/persist/bun/install/cache/playwright-core@1.63.0@@@1/index.mjs')));
const args = process.argv.slice(2);
const flag = (n) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : null; };
const has = (n) => args.includes(n);
const [name, url, shotsArg] = args;
if (!/^c-[a-z0-9-]+$/.test(name)) throw Error('name must start with c-');
const base = 'docs/2610/1007/';
const layoutOnly = has('--layout');
const noArchive = has('--no-archive'); // 页面文本已存档过时，只补截图
const fullPath = flag('--full');
const viewH = Number(flag('--height') || 1200);
const waitMs = Number(flag('--wait') || 6000);
const shots = shotsArg && shotsArg !== '-' && !shotsArg.startsWith('--') ? JSON.parse(fs.readFileSync(shotsArg, 'utf8')) : [];
const record = {time_bj: new Date(Date.now() + 28800000).toISOString().replace('Z', '+08:00'), url, name,
  tool: 'anonymous headless Chrome via Playwright Core; CSS 700; DPR 2; clean temp profile'};
let browser;
try {
  browser = await chromium.launchPersistentContext(path.join(os.tmpdir(), '1007-c-chrome-' + Date.now()), {
    executablePath: 'C:/Program Files/Google/Chrome/Application/chrome.exe', headless: true,
    viewport: {width: 700, height: viewH}, deviceScaleFactor: 2, locale: 'en-US',
    args: ['--no-first-run', '--no-default-browser-check']});
  const page = await browser.newPage();
  await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 60000});
  await new Promise(r => setTimeout(r, waitMs));
  // 触发懒加载图片：自上而下滚一遍，再回到顶部
  await page.evaluate(async () => {
    const h = () => document.documentElement.scrollHeight;
    for (let y = 0; y < h(); y += 500) { window.scrollTo(0, y); await new Promise(r => setTimeout(r, 150)); }
    window.scrollTo(0, 0);
    await Promise.all([...document.images].map(i => i.complete ? 1 : new Promise(r => { i.onload = i.onerror = r; setTimeout(r, 4000); })));
  });
  await new Promise(r => setTimeout(r, 1500));
  const data = await page.evaluate(() => ({
    url: location.href, title: document.title, text: document.body.innerText,
    meta: [...document.querySelectorAll('meta[name],meta[property],time')].map(e => e.outerHTML),
    docHeight: document.documentElement.scrollHeight,
    layout: [...document.querySelectorAll('h1,h2,h3,p,li,table,img,figure')].map(e => {
      const r = e.getBoundingClientRect();
      return {tag: e.tagName, text: (e.innerText || e.alt || '').slice(0, 90).replace(/\s+/g, ' '), y: Math.round(r.y + scrollY), h: Math.round(r.height)};
    }).filter(o => o.h > 0)}));
  data.captured_bj = record.time_bj;
  if (layoutOnly) {
    for (const o of data.layout) console.log(o.y, o.h, o.tag, o.text);
    console.log('docHeight', data.docHeight);
  } else if (!noArchive) {
    fs.writeFileSync(base + 'sources/' + name + '.json', JSON.stringify(data, null, 2), {flag: 'wx'});
    fs.writeFileSync(base + 'sources/' + name + '.txt', data.text, {flag: 'wx'});
  }
  if (fullPath) await page.screenshot({path: fullPath, fullPage: true});
  record.resolved_shots = [];
  for (const s of shots) {
    if (!/^(2[5-9]|3[0-4])-c-[a-z0-9-]+\.png$/.test(s.file)) throw Error('C image range: ' + s.file);
    const dst = base + 'images/' + s.file;
    if (fs.existsSync(dst)) throw Error('Existing image ' + s.file);
    const area = await page.evaluate(({from, to, padTop, padBottom, fromIdx, toIdx, toTop, fig, toFig}) => {
      const sel = 'h1,h2,h3,p,li,figure,img,td,th,div';
      const find = (txt, idx) => {
        const hits = [...document.querySelectorAll(sel)].filter(e => ((e.innerText || e.alt || '').replace(/\s+/g, ' ')).includes(txt));
        // 取最小（最内层）元素：不含其它命中元素
        const leaf = hits.filter(e => !hits.some(o => o !== e && e.contains(o)));
        if (!leaf.length) throw Error('anchor missing: ' + txt);
        return leaf[idx || 0];
      };
      const figs = [...document.querySelectorAll('figure')];
      const a = (fig !== undefined ? figs[fig] : find(from, fromIdx)).getBoundingClientRect();
      const b = (toFig !== undefined ? figs[toFig] : fig !== undefined ? figs[fig] : find(to || from, toIdx)).getBoundingClientRect();
      const y0 = Math.max(0, Math.floor(a.y + scrollY) - (padTop ?? 24));
      const y1 = Math.ceil((toTop ? b.y : b.bottom) + scrollY) + (padBottom ?? 24);
      return {y: y0, height: y1 - y0};
    }, s);
    await page.screenshot({path: dst, clip: {x: 0, y: area.y, width: 700, height: area.height}, fullPage: true});
    record.resolved_shots.push({file: s.file, from: s.from, to: s.to, ...area});
  }
  record.result = layoutOnly ? 'layout only' : 'saved; QA pending';
  record.chars = data.text.length;
} catch (e) {
  record.result = 'FAILED ' + String(e.message).slice(0, 400);
} finally {
  if (browser) await browser.close();
  if (!layoutOnly) fs.appendFileSync(base + 'sources/c-browser-records.jsonl', JSON.stringify(record) + '\n');
  console.log(JSON.stringify(record));
}
